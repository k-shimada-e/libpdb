use crate::structs::PDB;

use anyhow::{Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::iter;


pub fn save_pdb(pdb: PDB, filename: &str) -> Result<()>
{
    let f = File::create(filename).unwrap();
    save_pdb_raw(&pdb, BufWriter::new(f), false)?;
    Ok(())
}

pub fn save_pdb_atom(pdb: PDB, filename: &str) -> Result<()>
{
    let f = File::create(filename).unwrap();
    save_pdb_raw(&pdb, BufWriter::new(f), true)?;
    Ok(())
}

fn save_pdb_raw<W: Write>(pdb: &PDB, mut stream: BufWriter<W>, atom_only: bool) -> Result<()>
{
    // write line utility closure
    let mut write_line = |mut line: String| -> Result<()> {
        if line.len() < 70 {
            let dif = 70 - line.len();
            line.reserve(dif);
            line.extend(iter::repeat(" ").take(dif));
        }
        stream.write_all(line.as_bytes())?;
        stream.write_all(b"\n")?;
        Ok(())
    };

    // if write atom only, other than "ATOM" or "HETATM" are not wrote 
    if !atom_only {
        // write header
        if let Some(identifier) = pdb.identifier() {
            write_line(format!(
                "HEADER                                                        {}",
                identifier
            ))?;
        }
        // write remarks
        for line in pdb.remarks() {
            write_line(format!(
                "REMARK {:3} {}", line.0, line.1
            ))?;
        }
    }

    // write atoms
    for atom in pdb.atoms() {
        write_line(format!(
            "{}{:5} {:^4}{:1}{:4}{:1}{:4}{:1}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{}",
            if *atom.hetero() {"HETATM"} else {"ATOM  "},
            atom.serial_number(),
            atom.atom_name(),
            atom.alt_location().unwrap_or(" "),
            atom.res_name(),
            atom.chain_id(),
            atom.res_seq(),
            atom.i_code().unwrap_or(" "),
            atom.x(),
            atom.y(),
            atom.z(),
            atom.occupancy(),
            atom.temp_factor(),
            atom.element(),
            atom.charge()
        ))?;
    }
    // TER
    write_line("TER".to_owned())?;

    stream.flush()?;
    Ok(())
}