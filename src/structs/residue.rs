use std::fmt;
use std::cmp::Ordering;
#[cfg(feature = "rayon")]
use rayon::prelude::*

use crate::structs::atom::Atom;
use super::validator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Residue {
    name: String,
    serial_number: usize,
    atoms: Vec<Atom>,
}

impl Residue {
    pub fn new(name: &str, serial_number: usize, atom: Option<Atom>) -> Option<Residue> {
        if let Some(name) = validator::prepare_identifier(name) {
            let mut res = Residue {
                name,
                serial_number,
                atoms: Vec::new()
            };
            if let Some(atom) = atom {
                res.atoms.push(atom);
            }
            Some(res)
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) -> Result<(), String> {
        if validator::prepare_identifier(new_name) {
            self.name = new_name.trim().to_ascii_uppercase();
            Ok(())
        } else {
            Err(format!(
                "The new name has invalid for residue {}\n\tinvalid value: {}",
                self.serial_number, new_name
            ))
        }
    }

    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms.get(index)
    }

    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms.get_mut(index)
    }

    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms.iter()
    }

    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.atoms.iter_mut()
    }

    pub fn par_atoms(&self) -> impl ParallelIterator<Item = &Atom> + '_ {
        self.atoms.par_iter()
    }

    pub fn add_atom(&mut self, new_atom: Atom) {
        self.atoms.push(new_atom);
    }
}

impl fmt::Display for Residue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
        "Residue Number: {}, Name: {}, Atoms: {}",
        self.serial_number(), self.name(), self.atoms.len())
    }
}

impl PartialOrd for Residue {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        Some(self.serial_number().cmp(other.serial_number()))
    }
}

impl Ord for Residue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.serial_number().cmp(other.serial_number())
    }
}