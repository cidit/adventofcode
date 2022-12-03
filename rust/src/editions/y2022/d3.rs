pub fn p1(input: &str) -> String {
    input.lines()
        .map(|s| s.split_at(s.len()/2))
        .flat_map(|(left, right)| left.chars().find(|&c| right.contains(c)))
        .flat_map(item_code_to_priority)
        .sum::<u32>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|elves| elves[0].chars().find(|&c| elves[1].contains(c) && elves[2].contains(c)))
        .flat_map(item_code_to_priority)
        .sum::<u32>()
        .to_string()
}

fn item_code_to_priority(item_code: char) -> Option<u32> {
    if !item_code.is_alphabetic() { None }
    else if item_code.is_lowercase() { Some(item_code as u32 - 97 + 1) }
    else { Some(item_code as u32 - 65 + 27) }
}