pub mod y2015;
use crate::util::model::Solution;
use std::collections::HashMap;

pub fn solutions() -> HashMap<String, Solution> {
    HashMap::new()
        .into_iter()
        .chain(y2015::solutions())
        // ...chain()
        .collect()
}
