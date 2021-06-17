
pub(crate) enum ParsedItems {
    Header(String, String, String),
    Remark(usize, String),
    Atom(
        bool, // hetero
        usize, // serial number
        String, // atom name
        Option<String>, //alt location
        String, // residue name
        String, // chain id
        usize, // res sequence
        Option<String>, // i code
        f64, // x
        f64, // y
        f64, // z
        f64, // occupancy
        f64, // temp factor
        Option<String>, // segment id
        String, // element
        isize, // charge
    ),
    Ter,
    End,
    Empty,
}