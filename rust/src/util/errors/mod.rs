use std::error::Error;
use std::fmt;

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

#[derive(Debug)]
pub struct ArgsParsingError {}

impl fmt::Display for ArgsParsingError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Args in wront format")
    }
}

impl Error for ArgsParsingError {}

#[derive(Debug)]
struct KeyParsingError {
    original_string: String,
}

impl KeyParsingError {
    fn new(key: &str) -> Self {
        Self { original_string: key.to_owned() }
    }
}

impl fmt::Display for KeyParsingError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "trouble parsing '{}'", self.original_string)
    }
}

impl Error for KeyParsingError {}