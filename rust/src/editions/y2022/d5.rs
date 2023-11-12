use std::{str::FromStr, num::ParseIntError, collections::{LinkedList, HashMap}, slice::Chunks};

use itertools::Itertools;
use thiserror::Error;

pub fn p1(input: &str) -> String {
    let (mut crates, instructions) = parse(input);
    dbg!(&crates);
    instructions.iter().for_each(|i| {
        crates.mv(i);
        dbg!(&crates);
    });

    let biggest_stack_size = crates.stacks.iter()
    .map(|(_,l)| l)
    .map(LinkedList::len)
    .max()
    .unwrap();

    // dbg!(&crates.stacks);

    let tmp: Vec<_> = crates.stacks.into_iter()
        .filter(|(_, s)| s.len() == biggest_stack_size)
        .collect();

    "".to_string()
}

pub fn p2(input: &str) -> String {
    todo!()
}

fn parse(input: &str) -> (Crates, Vec<Instruction>) {

    let binding = input.lines()
        .take_while(|l| !l.is_empty())
        .filter(|l| !l.starts_with(" 1 "))
        .fold(String::new(), |a, b| format!("{a}{b}\n"));
    

    let crates = binding
        .parse()
        .unwrap();


    let instructions: Vec<_> = input.lines()
        .skip_while(|l| !l.is_empty()).skip(1)
        .map(str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect();

    (crates, instructions)
}

#[derive(Debug)]
struct Crates {
    stacks: HashMap<usize, LinkedList<char>>

}

impl Crates {
    fn insert(&mut self, stack: usize, crate_to_insert: char) {
        self.stacks.entry(stack + 1) // FIXME: + 1 shouldnt be here
            .or_insert_with(|| LinkedList::new())
            .push_back(crate_to_insert);

    }

    fn _mv(&mut self, from: usize, to: usize) {
        let item = self.stacks.get_mut(&from).unwrap().pop_front().unwrap();
        self.stacks.get_mut(&to).unwrap().push_front(item);

    }

    fn mv(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.quantity {
            self._mv(instruction.from, instruction.to);
        }
    }

}


impl FromStr for Crates {
    type Err = ParseCrateError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = Self { stacks: HashMap::new() };
        let crate_identifiers_lines = s.lines()
            .map(|l| {
                l.chars()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .chunks(4)
                    .into_iter()
                    .flat_map(|mut c| c.next())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
            
        for lines in crate_identifiers_lines {
            for (idx, identifier) in lines.into_iter().enumerate() { 
                if identifier == ' ' { continue }
                else {
                    crates.insert(idx, identifier);
                }
            }
        }

        Ok(crates)
    }
}

impl std::fmt::Display for Crates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Error, Debug)]
enum ParseCrateError {

}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}


impl FromStr for Instruction {
    type Err = ParseInstructionError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(' ')
            .collect::<Vec<_>>()
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|mut s| s.nth(1))
            .collect::<Option<Vec<_>>>()
            .ok_or(Self::Err::Missing {instruction: s.to_owned()})?
            .into_iter()
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()?;
        
        let e = ParseInstructionError::Missing {instruction: s.to_owned()};
        Ok(Self { 
            quantity: tokens.get(0).ok_or(e.clone())?.to_owned(), 
            from: tokens.get(1).ok_or(e.clone())?.to_owned(), 
            to: tokens.get(2).ok_or(e)?.to_owned(),
        })
    }
}


#[derive(Error, Debug, Clone)]
enum ParseInstructionError {
    // TODO: missing isnt very explanatory
    #[error("unparseable instruction: '{instruction}'.")]
    Missing { instruction: String},
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = indoc::indoc! {"
        [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
"};

    #[test]
    fn print_test() {
        println!("{TEST_INPUT}");
    }
    #[test]
    fn p1() {
        let out = super::p1(TEST_INPUT);
        dbg!(out);
    }
}