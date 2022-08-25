use std::error::Error;
use std::fmt;
use thiserror::Error;

use crate::util::model::SolutionKey;

#[derive(Debug)]
pub struct KeyMissingError {
    key: SolutionKey,
}

impl KeyMissingError {
    pub fn new(key: SolutionKey) -> Self {
        Self { key }
    }
}

impl fmt::Display for KeyMissingError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Key missing: {}", self.key)
    }
}

impl Error for KeyMissingError {}

#[derive(Error, Debug)]
pub enum KeyParsingError {
    #[error("Could'nt parse '{0}'. Expected a i32 value or a wilcard ('*') character.")]
    CantParse(String),
    #[error("expected exactly {expected} keys, found {found}. orignal string: {original_string}")]
    IncorrectNumKeys {
        expected: usize,
        found: usize,
        original_string: String,
    },
}
