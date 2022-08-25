pub fn p1(input: &str) -> String {
    let input = parse(input);

    let floor: i32 = input
        .iter()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    floor.to_string()
}

pub fn p2(input: &str) -> String {
    let input = parse(input);

    let mut floor = 0;
    let mut basement_instruction_number: Option<i32> = None;

    for (instruction_number, instruction) in input.iter().enumerate() {
        floor += match instruction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 && basement_instruction_number == None {
            basement_instruction_number = Some(instruction_number as i32);
        };
    }

    match basement_instruction_number {
        Some(x) => x.to_string(),
        None => "Santa never goes to the basement".to_string(),
    }
}

fn parse(input: &str) -> Vec<char> {
    input.trim().chars().collect::<Vec<_>>()
}
