use std::error::Error;
use std::fmt;
use thiserror::Error;

#[derive(Debug)]
pub struct KeyMissingError<'a> {
    key: &'a str,
}

impl<'a> KeyMissingError<'a> {
    pub fn new(key: &'a str) -> Self {
        Self { key }
    }
}

impl fmt::Display for KeyMissingError<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Key missing: {}", self.key)
    }
}

impl Error for KeyMissingError<'_> {}


#[derive(Error, Debug)]
pub enum KeyParsingError {
    #[error("Could'nt parse '{0}'. Expected a i32 value or a wilcard ('*') character.")]
    CantParse(String),
    #[error("expected exactly {expected} keys, found {found}. orignal string: {original_string}")]
    IncorrectNumKeys {
        expected: usize,
        found: usize,
        original_string: String
    }
}
