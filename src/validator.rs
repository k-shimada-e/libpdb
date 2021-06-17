/// Checks if a char is allowed in a PDB file.
/// The char has to be ASCII graphic or a space.
/// Returns `true` if the char is valid.
pub fn check_char(c: char) -> bool {
    (c as u32) < 127 && (c as u32) > 31
}

/// Checks a string using `check_char`.
/// Returns `true` if the text is valid.
pub fn valid_identifier(text: &str) -> bool {
    for c in text.chars() {
        if !check_char(c) {
            return false;
        }
    }
    true
}

pub fn prepare_identifier(text: &str) -> Option<String> {
    if valid_identifier(text) && !text.trim().is_empty() {
        Some(text.trim().to_ascii_uppercase())
    } else {
        None
    }
}