// edition/day/part

use std::{iter::Map, collections::HashMap};

use anyhow::Result;

pub type Solution = fn(&str) -> Result<String>;

pub struct Day {
    pub p1: Solution,
    pub p2: Solution,
}

impl Day {
    fn get_part(&self, part_num:usize) -> Option<Solution> {
        match part_num {
            1 => Some(self.p1),
            2 => Some(self.p2),
            _ => None,
        }
    }
}

pub struct Edition {
    days: [Option<Day>; 25],
}

pub struct AdventOfCode {
    editions: HashMap<usize, Edition>,
}

pub struct Key {
    editions: usize,
    day: usize,
    part: usize,
}

// impl AdventOfCode {
//     fn find(&self, key: Key) -> Option<Solution> {
//         let days = self.editions.get(&key.editions)?.days;
//         days[key.day]?.get_part(key.part)
//     }
// }

#[cfg(test)]
mod tests {

    // use super::*;

    // #[test]
    // fn can_build() {
    //     let solutions = SolutionRouter::Router(Router(Vec::from([
    //         ("2015", SolutionRouter::Router(Router(Vec::from([
    //             ("1", SolutionRouter::Router(Router(Vec::from([
    //                 ("1", SolutionRouter::Solution(Solution(str::to_string))),
    //                 ("2", SolutionRouter::Solution(Solution(str::to_string))),
    //             ])))),
    //             ("2", SolutionRouter::Router(Router(Vec::from([
    //                 ("1", SolutionRouter::Solution(Solution(str::to_string))),
    //                 ("2", SolutionRouter::Solution(Solution(str::to_string))),
    //             ])))),
    //         ])))),
    //     ])));
    // }
}
