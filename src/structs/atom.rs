use std::fmt;
use std::cmp::Ordering;

use getset::{CopyGetters, Getters, MutGetters, Setters};
use anyhow::Result;

use crate::validator;
use crate::error::PDBError;



#[derive(Debug, CopyGetters, Getters, MutGetters, Setters)]
pub struct Atom {
    /// Determines if this atom is an hetero atom (true), a non standard atom, or a normal atom (false)
    #[getset(get = "pub", set = "pub")]
    hetero: bool, // "ATOM" or "HETATM"

    /// The serial number of the Atom, should be unique within its model
    #[getset(get = "pub", set = "pub")]
    serial_number: usize, // "1"

    /// The name of the Atom, can only use the standard allowed characters
    #[getset(get = "pub")]
    atom_name: String, // "N"

    /// Alternative location indicator
    alt_location: Option<String>,

    /// The residue name
    #[getset(get = "pub")]
    res_name: String, // "SER"

    /// The Chain ID of Atom
    #[getset(get = "pub")]
    chain_id: String, // "A"

    /// The residue sequence number
    #[getset(get = "pub")]
    res_seq: usize, // "67"

    /// The code for insertion of residues
    i_code: Option<String>,

    /// The X position of the Atom (Å)
    #[getset(get = "pub")]
    x: f64, // "27.754"

    /// The Y position of the Atom (Å)
    #[getset(get = "pub")]
    y: f64, // "114.437"

    /// The Z position of the Atom (Å)
    #[getset(get = "pub")]
    z: f64, // "36.660"

    /// The occupancy of the Atom
    #[getset(get = "pub")]
    occupancy: f64, // "1.00"

    /// The B-factor (or temperature factor) of the Atom
    #[getset(get = "pub")]
    temp_factor: f64, // "24.36"

    /// The segmant idntifier
    #[getset(get = "pub")]
    segment_id: Option<String>,

    /// The element of the Atom, can only use the standard allowed characters
    #[getset(get = "pub")]
    element: String, // "N"

    /// The charge of the Atom
    #[getset(get = "pub", set = "pub")]
    charge: isize,
}

impl Atom {
    pub fn new(
        hetero: bool,
        serial_number: usize,
        atom_name: &str,
        res_name: &str,
        chain_id: &str,
        res_seq: usize,
        x: f64,
        y: f64,
        z: f64,
        occupancy: f64,
        temp_factor: f64,
        element: &str,
        charge: isize,
    ) -> Option<Atom> {
        if validator::valid_identifier(atom_name)
        && validator::valid_identifier(element)
        && x.is_finite()
        && y.is_finite()
        && z.is_finite()
        && occupancy.is_finite()
        && temp_factor.is_finite()
        {
            Some(Atom {
                hetero,
                serial_number,
                atom_name: atom_name.trim().to_ascii_uppercase(),
                alt_location: None,
                res_name: res_name.trim().to_ascii_uppercase(),
                chain_id: chain_id.trim().to_ascii_uppercase(),
                res_seq,
                i_code: None,
                x,
                y,
                z,
                occupancy,
                temp_factor,
                segment_id: None,
                element: element.trim().to_ascii_uppercase(),
                charge,
            })
        } else {
            None
        }
    
    }

    pub fn alt_location(&self) -> Option<&str> {
        self.alt_location.as_deref()
    }

    pub fn i_code(&self) -> Option<&str> {
        self.i_code.as_deref()
    }

    pub fn set_name(&mut self, new_name: &str) -> Result<(), PDBError>{
        if validator::valid_identifier(new_name) {
            self.atom_name = new_name.trim().to_ascii_uppercase();
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The new name has invalid characters for atom {}\n\tinvalid value: {}",
                self.serial_number, new_name
            )))
        }
    }

    pub fn set_residue_name(&mut self, new_res_name: &str) -> Result<(), PDBError> {
        if validator::valid_identifier(new_res_name) && new_res_name.len() == 3 {
            self.res_name = new_res_name.trim().to_ascii_uppercase();
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The new residue name has invalid characters or length for atom {}\n\tinvalid value: {}",
                self.serial_number, new_res_name
            )))
        }
    }

    pub fn position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn set_position(&mut self, new_position: (f64, f64, f64)) -> Result<(), PDBError> {
        if new_position.0.is_finite() && new_position.1.is_finite() && new_position.2.is_finite() {
            self.x = new_position.0;
            self.y = new_position.1;
            self.z = new_position.2;
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "One (or more) of values of the new position is not finate for atom {}\n\tinvalid values: {:?}",
                self.serial_number, new_position
            )))
        }
    }

    pub fn set_x(&mut self, new_x: f64) -> Result<(), PDBError> {
        if new_x.is_finite() {
            self.x = new_x;
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The value of the new x position is not finite for atom {}\n\tinvalid value: {}",
                self.serial_number, new_x
            )))
        }
    }
    
    pub fn set_y(&mut self, new_y: f64) -> Result<(), PDBError> {
        if new_y.is_finite() {
            self.y = new_y;
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The value of the new y position is not finite for atom {}\n\tinvalid value: {}",
                self.serial_number, new_y
            )))
        }
    }
    
    pub fn set_z(&mut self, new_z: f64) -> Result<(), PDBError> {
        if new_z.is_finite() {
            self.z = new_z;
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The value of the new z position is not finite for atom {}\n\tinvalid value: {}",
                self.serial_number, new_z
            )))
        }
    }
    
    pub fn set_occupancy(&mut self, new_occupancy: f64) -> Result<(), PDBError> {
        if new_occupancy.is_finite() {
            self.occupancy = new_occupancy;
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The value of the new occupancy is not finite for atom {}\n\tinvalid value: {}",
                self.serial_number, new_occupancy
            )))
        }
    }
    
    pub fn set_temp_factor(&mut self, new_temp_factor: f64) -> Result<(), PDBError> {
        if new_temp_factor.is_finite() {
            self.temp_factor = new_temp_factor;
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The value of the new temp_factor is not finite for atom {}\n\tinvalid value: {}",
                self.serial_number, new_temp_factor
            )))
        }
    }

    pub fn set_element(&mut self, new_element: &str) -> Result<(), PDBError> {
        if validator::valid_identifier(new_element) {
            self.element = new_element.trim().to_ascii_uppercase();
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The new element has invalid characters for atom {}\n\tinvalid values: {}",
                self.serial_number, new_element
            )))
        }
    }


    pub fn set_chain_id(&mut self, new_id: &str) -> Result<(), PDBError> {
        if validator::valid_identifier(new_id) && new_id.len() == 1 {
            self.chain_id = new_id.trim().to_ascii_uppercase();
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
                format!(
                "The new chain id has invalid character for atom {}\n\tinvalid value: {}",
                self.serial_number, new_id
            )))
        }
    }

}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atom = match self.hetero {
            true => "HETATM",
            false => "ATOM",
        };

        write!(
            f,
            "{{\n\t'type' : '{}',\n\t'number' : {},\n\t'element' : '{}',\n\t'residue' : '{}',\n\t'chain' : '{}',\n\t'position' : ({}, {}, {}),\n\t'occupancy' : {},\n\t'temp_factor' : {}\n}}",
            atom,
            self.serial_number,
            self.element,
            self.res_name,
            self.chain_id,
            self.x,
            self.y,
            self.z,
            self.occupancy,
            self.temp_factor
        )
    }
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        let atom = Atom::new(
            self.hetero,
            self.serial_number,
            &self.atom_name,
            &self.res_name,
            &self.chain_id,
            self.res_seq,
            self.x,
            self.y,
            self.z,
            self.occupancy,
            self.temp_factor,
            &self.element,
            self.charge
        )
        .expect("Invalid Atom properties in a clone");
        atom
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.serial_number == other.serial_number
        && self.atom_name() == other.atom_name()
        && self.element() == other.element()
        && self.res_name() == other.res_name()
        && self.chain_id() == other.chain_id()
        && self.position() == other.position()
        && self.occupancy() == other.occupancy()
        && self.temp_factor() == other.temp_factor()
    }
}

impl Eq for Atom {}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.serial_number.cmp(&other.serial_number))
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.serial_number.cmp(&other.serial_number)
    }
}



#[cfg(test)]
mod tests {
    //use super::Atom;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
