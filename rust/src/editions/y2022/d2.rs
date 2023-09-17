

pub fn p1(input: &str) -> String {
    let parsed = parse(input);

    parsed.iter()
        .map(|(opponent, player)| (
            parse_opponent(*opponent).unwrap(), 
            parse_player(*player).unwrap()
        ))
        .map(|(opponent, player)| player.cmp(&opponent).point_value() + player.point_value())
        .sum::<u32>().to_string()
}

pub fn p2(input: &str) -> String {
    let parsed = parse(input);
    parsed.iter()
        .map(|(opponent, player)| (
            parse_opponent(*opponent).unwrap(), 
            parse_outcome(*player).unwrap()
        ))
        .map(|(opponent, outcome)| (
            opponent.clone(),
            chose_what_to_play(outcome, opponent),
        ))
        .map(|(opponent, player)| player.cmp(&opponent).point_value() + player.point_value())
        .sum::<u32>().to_string()
}

fn parse(input: &str) -> Vec<(char, char)> {
    input.lines()
        .map(|l| (
            l.chars().nth(0).unwrap(), 
            l.chars().nth(2).unwrap(),
        ))
        .collect()
}

#[derive(Clone, Copy)]
enum Choice {
    Rock, Paper, Scisors
}

enum Outcome {
    Win, Draw, Lose
}

impl Choice {
    fn point_value(&self) -> u32 {
        use Choice::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scisors => 3,
        }
    }

    fn cmp(&self, other: &Self) -> Outcome {
        use Outcome::*;
        use Choice::*;

        match self {
            Rock => match other {
                Rock => Draw,
                Paper => Lose,
                Scisors => Win,
            },
            Paper => match other {
                Rock => Win,
                Paper => Draw,
                Scisors => Lose,
            },
            Scisors => match other {
                Rock => Lose,
                Paper => Win,
                Scisors => Draw,
            }
        }
    }
}

impl Outcome {
    fn point_value(&self) -> u32 {
        use Outcome::*;
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

fn parse_opponent(char: char) -> Option<Choice> {
    match char {
        'A' => Some(Choice::Rock),
        'B' => Some(Choice::Paper),
        'C' => Some(Choice::Scisors),
        _ => None,
    }
}

fn parse_player(char: char) -> Option<Choice> {
    match char {
        'X' => Some(Choice::Rock),
        'Y' => Some(Choice::Paper),
        'Z' => Some(Choice::Scisors),
        _ => None,
    }
}

fn parse_outcome(char: char) -> Option<Outcome> {
    match char {
        'X' => Some(Outcome::Lose),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => None,
    }
}

fn chose_what_to_play(outcome: Outcome, opponent: Choice) -> Choice {
    match outcome {
        Outcome::Draw => opponent,
        Outcome::Lose => match opponent {
            Choice::Rock => Choice::Scisors,
            Choice::Paper => Choice::Rock,
            Choice::Scisors => Choice::Paper,
        },
        Outcome::Win => match opponent {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scisors,
            Choice::Scisors => Choice::Rock,
        }
    }
}