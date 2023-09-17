mod d1;
mod d2;
mod d3;
use crate::util::model::Solution;
use std::collections::HashMap;

pub fn solutions() -> HashMap<String, Solution> {
    HashMap::from([
        ("2022/1/1".to_owned(), d1::p1 as Solution),
        ("2022/1/2".to_owned(), d1::p2 as Solution),
        ("2022/2/1".to_owned(), d2::p1 as Solution),
        ("2022/2/2".to_owned(), d2::p2 as Solution),
        ("2022/3/1".to_owned(), d3::p1 as Solution),
        ("2022/3/2".to_owned(), d3::p2 as Solution),
    ])
}
