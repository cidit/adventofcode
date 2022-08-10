// pub struct

use crate::util::errors::KeyParsingError;

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
        let keys: Vec<_> = s.split('/').collect();
        let expected = 3;
        if keys.len() != 3 {
            return Err(Self::Err::IncorrectNumKeys {
                expected,
                found: keys.len(),
                original_string: s.to_owned(),
            })
        }

        Ok(Self {
            edition: keys[0].parse()?,
            day: keys[1].parse()?,
            part: keys[2].parse()?,
        })
    }
}

impl std::str::FromStr for KeyType {
    type Err = KeyParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Self::Wildcard)
        } else if let Ok(n) = s.parse::<i32>() {
            Ok(Self::Num(n))
        } else {
            Err(Self::Err::CantParse(s.to_owned()))
        }
    }
}
