use crate::util::errors::KeyParsingError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SolutionKey {
    edition: KeyType,
    day: KeyType,
    part: KeyType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    Wildcard,
    Num(i32),
}

impl SolutionKey {
    // pub fn edition(&self) -> &KeyType {
    //     &self.edition
    // }
    // pub fn day(&self) -> &KeyType {
    //     &self.day
    // }
    // pub fn part(&self) -> &KeyType {
    //     &self.part
    // }
    pub fn any_wild(&self) -> bool {
        use KeyType::Wildcard;
        self.edition == Wildcard || self.day == Wildcard || self.part == Wildcard
    }
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
            });
        }

        Ok(Self {
            edition: keys[0].parse()?,
            day: keys[1].parse()?,
            part: keys[2].parse()?,
        })
    }
}

impl fmt::Display for SolutionKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}/{}", self.edition, self.day, self.part)
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

impl fmt::Display for KeyType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Wildcard => "*".to_owned(),
                Self::Num(n) => n.to_string(),
            }
        )
    }
}
