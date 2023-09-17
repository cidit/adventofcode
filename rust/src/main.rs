mod editions;
mod util;
mod solution;

use std::io::Read;

use util::errors::KeyMissingError;
use util::model::SolutionKey;

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1).expect("expected one argument");
    let keys: SolutionKey = arg.parse().expect("couldnt parse keys from argument");

    if keys.any_wild() {
        unimplemented!("doesnt support wildcards yet")
    }

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let result = match editions::solutions().get(&arg) {
        Some(f) => Ok(f(&input)),
        None => Err(KeyMissingError::new(keys.clone())),
    };

    println!(
        "{}",
        result.unwrap_or_else(|_| format!("no entry for {}", keys))
    );
    Ok(())
}
