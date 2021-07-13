#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::structs::atom::Atom;
use crate::validator;
use crate::error::PDBError;

#[derive(Debug, Clone, PartialEq)]
pub struct PDB {
    identifier: Option<String>,
    remarks: Vec<(usize, String)>,
    atoms: Vec<Atom>,
}

impl PDB {
    pub fn new() -> PDB {
        PDB {
            identifier: None,
            remarks: Vec::<(usize, String)>::new(),
            atoms: Vec::<Atom>::new(),
        }
    }

    pub fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    pub fn set_identifier(&mut self, new_name: &str) -> Result<(), PDBError> {
        if let Some(new_name) = validator::prepare_identifier(new_name) {
            self.identifier = Some(new_name.trim().to_ascii_uppercase());
            Ok(())
        } else {
            Err(PDBError::InvalidValue(
            format!(
                "invalid name for PDB: {}"
                , new_name
            )))
        }
    }

    pub fn remarks(&self) -> impl DoubleEndedIterator<Item = &(usize, String)> + '_ {
        self.remarks.iter()
    }

    pub fn add_remarks(&mut self, remark_type: usize, remark_text: &str) -> Result<(), PDBError> {
        if !REMARK_TYPES.contains(&remark_type) {
            return Err(PDBError::InvalidValue(
                format!("given remark-type '{}' is not valid", remark_type)
            ))
        }
        if remark_text.len() > 70 {
            panic!("given remark text is too long (>70)")
        }
        self.remarks.push((remark_type, remark_text.to_owned()));
        Ok(())
    }

    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms.iter()
    }

    #[cfg(feature = "rayon")]
    pub fn par_atoms(&self) -> impl ParallelIterator<Item = Atom> + '_ {
        self.atoms.par_iter()
    }

    pub fn add_atom(&mut self, new_atom: Atom) {
        self.atoms.push(new_atom);
    }
}


const REMARK_TYPES: [usize; 42] = [
    0, 1, 2, 3, 4, 5, 100, 200, 205, 210, 215, 217, 230, 240, 245, 247, 250, 265, 280, 285, 290,
    300, 350, 375, 400, 450, 465, 470, 475, 480, 500, 525, 600, 610, 615, 620, 630, 650, 700, 800, 900,
    999,
];
