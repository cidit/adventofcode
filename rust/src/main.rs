mod util;
mod y2015;


use util::errors::{KeyMissingError};
use util::model::SolutionKey;


fn main() {
    let arg = std::env::args().nth(1).expect("expected one argument");
    let keys: SolutionKey = arg.parse().expect("couldnt parse keys from argument");

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