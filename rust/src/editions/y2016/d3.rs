use std::num::ParseIntError;

pub fn p1(input: &str) -> String {
    let instructions = parse_p1(input).unwrap();
    count_valid_triangles(instructions).to_string()
}

pub fn p2(input: &str) -> String {
    let instructions = parse_p2(input).unwrap();
    count_valid_triangles(instructions).to_string()
}

fn parse_p1(input: &str) -> Result<Vec<Instruction>, ParseIntError> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().split_whitespace().map(str::parse::<u32>).collect())
        .collect()
}

fn parse_p2(input: &str) -> Result<Vec<Instruction>, ParseIntError> {
    let instructions: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.trim().split_whitespace().map(str::parse::<u32>).collect())
        .collect::<Result<_, _>>()?;

    let flat: Vec<_> = instructions.iter().flatten().collect();
    let parsed: Vec<Vec<u32>> = flat
        .chunks(9)
        .map(|s| {
            let s = s.to_owned();
            let mut v: Vec<Vec<u32>> = Vec::new();
            for i in 0..s.len() / 3 {
                v.push(vec![s[i + 0].clone(), s[i + 3].clone(), s[i + 6].clone()])
            }
            v
        })
        .flatten()
        .collect();
    Ok(parsed)
}
type Instruction = Vec<u32>;

fn count_valid_triangles(instructions: Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .filter(|v| {
            let (a, b, c) = (v[0], v[1], v[2]);
            a + b > c && a + c > b && b + c > a
        })
        .count()
        .try_into()
        .unwrap()
}
