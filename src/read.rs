use anyhow::{anyhow, ensure, Result, Context as _};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::TryFrom as _;

use crate::structs::{PDB, Atom};
use crate::item::ParsedItems;

pub fn read_pdb(filename: &str) -> Result<PDB>
{
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    let pdb = read_pdb_raw(reader)?;
    Ok(pdb)
}

pub fn read_pdb_raw<T>(input: BufReader<T>) -> Result<PDB>
    where T: std::io::Read
{
    let mut pdb = PDB::new();
    for (mut line_number, read_line) in input.lines().enumerate() {
        line_number += 1;
        let line = if let Ok(l) = read_line {
            l
        } else {
            return Err(anyhow!(format!("could not read line {}", line_number)));
        };

        let parse_result = parse_line(&line, line_number);

        if let Ok(result) = parse_result {
            match result {
                ParsedItems::Header(_, _, idntifier) => pdb.set_identifier(&idntifier)?,
                ParsedItems::Remark(remark_type, remark_text) => pdb.add_remarks(remark_type, &remark_text)?,
                ParsedItems::Atom(
                    hetero,
                    serial_number,
                    atom_name,
                    _alt_location,
                    res_name,
                    chain_id,
                    res_seq,
                    _i_code,
                    x,
                    y,
                    z,
                    occupancy,
                    temp_factor,
                    _segment_id,
                    element,
                    charge,
                ) => pdb.add_atom(
                    Atom::new(
                        hetero,
                        serial_number,
                        &atom_name,
                        &res_name,
                        &chain_id,
                        res_seq,
                        x,
                        y,
                        z,
                        occupancy,
                        temp_factor,
                        &element,
                        charge
                    ).ok_or(anyhow!(""))?,
                ),
                _ => (),
            }   
        };

    }
    Ok(pdb)
}

fn parse_line(line: &str, line_number: usize) -> Result<ParsedItems>{
    if line.len() > 6 {
        match &line[..6] {
            "HEADER" => parse_header(line, line_number),
            "REMARK" => parse_remarks(line, line_number),
            "HETATM" => parse_atom(line, line_number, true),
            "ATOM  " => parse_atom(line, line_number, false),
            "TER   " => Ok(ParsedItems::Ter),
            "END   " => Ok(ParsedItems::End),
            _ => Ok(ParsedItems::Empty),
        }
    } else if line.len() > 2 {
        match &line[..2] {
            "TER" => Ok(ParsedItems::Ter),
            "END" => Ok(ParsedItems::End),
            _ => Ok(ParsedItems::Empty),
        }
    } else {
        Ok(ParsedItems::Empty)
    }
}

fn parse_header(line: &str, line_number: usize) -> Result<ParsedItems> {
    ensure!(line.len() >= 66, format!("Header is too short: line {}", line_number));

    Ok(ParsedItems::Header(
        line.get(10..50).unwrap_or("").to_owned(),
        line.get(50..59).unwrap_or("").to_owned(),
        line.get(62..66).unwrap_or("").to_owned(),
    ))
}

fn parse_remarks(line: &str, line_number: usize) -> Result<ParsedItems> {
    ensure!(line.len() <= 80, format!("remarks is too long"));
    let number = parse_usize(&line.chars().collect::<Vec<char>>()[7..10], line_number)?;
    Ok(ParsedItems::Remark(
        number,
        line.get(11..).unwrap_or("").trim_end().to_owned(),
    ))
}

fn parse_atom(line: &str, line_number: usize, hetero: bool) -> Result<ParsedItems>
{
    let chars: Vec<char> = line.chars().collect();
    ensure!(chars.len() >= 54,
            format!("Atom line is too short: line {}", line_number));

    let serial_number = parse_usize(&chars[6..11], line_number)?;
    let atom_name = chars[12..16].iter().collect::<String>();
    let res_name =  chars[17..20].iter().collect::<String>();
    let chain_id = String::from(chars[21]);
    let res_seq = parse_usize(&chars[22..26], line_number)?;

    let x = parse_f64(&chars[30..38], line_number)?;
    let y = parse_f64(&chars[38..46], line_number)?;
    let z = parse_f64(&chars[46..54], line_number)?;

    let mut occupancy = 1.0;
    if chars.len() >= 60 {
        occupancy = parse_f64(&chars[54..60], line_number)?;
    }

    let mut temp_factor = 0.0;
    if chars.len() >= 66 {
        temp_factor = parse_f64(&chars[60..66], line_number)?;
    }

    let mut element = String::new();
    if chars.len() >= 77 {
        element = chars[76..78].iter().collect::<String>();
    }

    let mut charge: isize = 0;
    if chars.len() >= 80 && !(chars[78].is_whitespace() && chars[79].is_whitespace())
    {
        ensure!(chars[78].is_ascii_digit(),
                format!("atom charge is not collect numeric ([0-9][+-]) at line {}: {}", line_number, chars[78..79].iter().collect::<String>()));
        
        ensure!((chars[79] == '-' || chars[79] == '+'),
                format!("atom charge is not properly signed ([0-9][+-]) at line {}: {}", line_number, chars[78..79].iter().collect::<String>()));
        charge = isize::try_from(
            chars[78].to_digit(10)
            .ok_or(
                anyhow!("can not parse character into digit at line {}: {:?}", line_number, chars[78])
            )?
        )?;
    }

    Ok(ParsedItems::Atom(
        hetero,
        serial_number,
        atom_name,
        None,
        res_name,
        chain_id,
        res_seq,
        None,
        x,
        y,
        z,
        occupancy,
        temp_factor,
        None,
        element,
        charge,
    ))


}

fn parse_f64(input: &[char], line_number: usize) -> Result<f64> {
    let string = input
    .iter()
    .collect::<String>()
    .split_whitespace()
    .collect::<String>();

    string.parse::<f64>().with_context(|| format!("can't parse the number as f64 at line {}: {:?}", line_number, input))
}

fn parse_usize(input: &[char], line_number: usize) -> Result<usize> {
    let string = input
    .iter()
    .collect::<String>()
    .split_whitespace()
    .collect::<String>();

    string.parse::<usize>().with_context(|| format!("can't parse the number as usize at line {}: {:?}", line_number, input))
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_parse_f64() {
        let chara: Vec<char> = "54.572".chars().collect();
        assert_eq!(54.572 as f64, parse_f64(&chara, 1).unwrap());    
    }

    #[test]
    fn can_parse_usize() {
        let chara: Vec<char> = "1234".chars().collect();
        assert_eq!(1234 as usize, parse_usize(&chara, 1).unwrap());
    }
}
