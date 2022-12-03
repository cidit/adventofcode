mod d1;
use crate::util::model::Solution;
use std::collections::HashMap;

pub fn solutions() -> HashMap<String, Solution> {
    HashMap::from([
        ("2022/1/1".to_owned(), d1::p1 as Solution),
        ("2022/1/2".to_owned(), d1::p2 as Solution),
//        ("2015/2/1".to_owned(), d2::p1 as Solution),
//        ("2015/2/2".to_owned(), d2::p2 as Solution),
    ])
}
