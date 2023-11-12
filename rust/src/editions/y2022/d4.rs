use std::ops::{RangeInclusive};

pub fn p1(input: &str) -> String {
    parse(input)
        .iter()
        .filter(|assignments| {
            (assignments[0].start() <= assignments[1].start() && assignments[0].end() >= assignments[1].end()) ||
            (assignments[0].start() >= assignments[1].start() && assignments[0].end() <= assignments[1].end())
        })
        .count()
        .to_string()
}


pub fn p2(input: &str) -> String {
    parse(input)
        .iter()
        .filter(|assignments| {
            assignments[0].clone().any(|section| assignments[1].contains(&section))
        })
        .count()
        .to_string()
}

fn parse(input: &str) -> Vec<Vec<RangeInclusive<i32>>> {
    input.lines()
        .map(|s| {
            s.split(',')
                .map(|s| {
                    s.split('-')
                        .map(str::parse::<i32>)
                        .map(Result::unwrap)
                        .collect::<Vec<_>>()
                })
                .map(|bounds| bounds[0]..=bounds[1])
                .collect::<Vec<_>>()
        })
        .collect()
}