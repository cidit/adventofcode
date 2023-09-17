pub mod y2015;
pub mod y2022;
use crate::util::model::Solution;
use std::collections::HashMap;

pub fn solutions() -> HashMap<String, Solution> {
    HashMap::new()
        .into_iter()
        .chain(y2015::solutions())
        .chain(y2022::solutions())
        // ...chain()
        .collect()
}
