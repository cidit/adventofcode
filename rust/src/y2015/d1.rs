use std::error::Error;

pub fn solution(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let input = parse(input)?;

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

    let convert_to_string = |input: Option<i32>| match input {
        Some(x) => x.to_string(),
        None => "Santa never goes to the basement".to_string(),
    };

    Ok(vec![
        format!("part 1 answer: {}", floor),
        format!(
            "part 2 answer: {}",
            convert_to_string(basement_instruction_number)
        ),
    ])
}

fn parse(input: &str) -> Result<Vec<char>, Box<dyn Error>> {
    return Ok(input.trim().chars().collect::<Vec<_>>());
}
