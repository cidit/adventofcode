mod util;
mod y2015;

use core::panic;

use util::errors::{ArgsParsingError, KeyMissingError};
use util::model::SolutionKey;


fn main() {
    let arg = std::env::args().nth(1).expect("expected one argument");
    let keys = parse_keys(&arg);

    // in the following format: year/day/part where n is either a wildcard or a number, identifying one or many aoc challenges
    // let keys = args.split

    let input = "hello world";

    let key = "hello";

    let result = match key {
        "2015" => y2015::solutions(key, input),
        _ => Err(Box::from(KeyMissingError::new(key))),
    };

    println!(
        "{:?}",
        result.unwrap_or_else(|_| Vec::from([format!("no entry for {:?}", key)]))
    );
}

fn parse_keys(arg: &str) -> Vec<SolutionKey> {
    let keys: Vec<_> = arg.split("/").map(str::trim).collect();

    if keys.len() != 3 {
        panic!("expected 3 keys separated by '/'");
    }

    keys.iter()
        .cloned()
        .map(is_valid_key)
        .zip(&keys)
        .for_each(|(valid, key)| {
            if !valid {
                panic!("invalid key {}", key);
            }
        });

    return keys.iter().cloned().map(str::to_owned).collect();
}

fn is_valid_key(key: &str) -> bool {
    if key.chars().all(|c| c.is_numeric()) {
        return true;
    }

    if key.len() == 1 && key.chars().next() == Some('*') {
        return true;
    }

    return false;
}
