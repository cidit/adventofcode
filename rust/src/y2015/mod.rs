pub mod d1;
use crate::util::errors::KeyMissingError;
use std::error::Error;

pub fn solutions<'a>(key: &'a str, input: &str) -> Result<Vec<String>, Box<dyn Error + 'a>> {
    match key {
        "1" => d1::solution(input),
        _ => Result::Err(Box::from(KeyMissingError::new(key))),
    }
}
