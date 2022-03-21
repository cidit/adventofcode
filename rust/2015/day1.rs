use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/2015/day1.data")
                .expect("unable to read file");
    let iter = input.trim().split("");

    let mut floor = 0;
    let mut basement_instruction_number: Option<i32> = None;

    for (instruction_number, instruction) in iter.enumerate() {
        floor += match instruction {
            "(" => 1,
            ")" => -1,
            _ => 0,
        };
        if floor == -1 && basement_instruction_number == None {
            basement_instruction_number = Some(instruction_number as i32);
        };
    }

    let convert_to_string = |input: Option<i32>| {
        match input {
            Some(x) => x.to_string(),
            None => "Santa never goes to the basement".to_string(),
        }
    };

    println!("part 1 answer: {}", floor);
    println!("part 2 answer: {}", convert_to_string(basement_instruction_number))
}
