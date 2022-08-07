// pub struct

use util::model::{KeyParsingError}

#[derive(Debug)]
pub struct SolutionKey {
    edition: KeyType,
    day: KeyType,
    part: KeyType,
}

#[derive(Debug)]
pub enum KeyType {
    Wildcard,
    Num(i32),
}

impl std::str::FromStr for SolutionKey {
    type Err = KeyParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}