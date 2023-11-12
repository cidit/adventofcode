#[rustfmt::skip]
mod unformatted {

    pub fn p1_pattern() -> Vec<String> {
        [
            "123",
            "456",
            "789",
        ].into_iter().map(str::to_string).collect()
    }    

    pub fn p2_pattern() -> Vec<String> {
        [
            "..1..",
            ".234.",
            "56789",
            ".ABC.",
            "..D..",
        ].into_iter().map(str::to_string).collect()
    }
}

pub fn p1(input: &str) -> String {
    let instructions = parse(input);
    let keypad = Keypad::from_pattern(&unformatted::p1_pattern());
    let mut cursor = Point(1, 1);
    let mut code = String::new();

    for instruction in instructions {
        for direction in instruction {
            let new_pos = cursor.add(&direction.variance());
            if let Some(_) = keypad.spot_at(&new_pos) {
                cursor = new_pos;
            } else {
                continue;
            }
        }
        let codepoint = keypad.spot_at(&cursor).unwrap();
        code.push(codepoint);
    }

    code
}

pub fn p2(input: &str) -> String {
    let instructions = parse(input);
    let keypad = Keypad::from_pattern(&unformatted::p2_pattern());
    let mut cursor = Point(0, 2);
    let mut code = String::new();

    for instruction in instructions {
        for direction in instruction {
            let new_pos = cursor.add(&direction.variance());
            if let Some(_) = keypad.spot_at(&new_pos) {
                cursor = new_pos;
            } else {
                continue;
            }
        }
        let codepoint = keypad.spot_at(&cursor).unwrap();
        code.push(codepoint);
    }

    code
}

fn parse(input: &str) -> Instructions {
    input
        .trim()
        .lines()
        .map(|l| l.trim().chars().map(Direction::try_from).collect())
        .collect::<Result<_, _>>()
        .unwrap()
}

type Instructions = Vec<Vec<Direction>>;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn variance(&self) -> Point {
        match self {
            Self::Up => Point(0, -1),
            Self::Down => Point(0, 1),
            Self::Left => Point(-1, 0),
            Self::Right => Point(1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            other => Err(format!(
                "Unexpected character when parsing direction: '{other}'."
            )),
        }
    }
}

struct Keypad {
    w: usize,
    h: usize,
    spots: Vec<Option<char>>,
}

impl Keypad {
    fn spot_at(&self, p: &Point) -> Option<char> {
        let Point(x, y) = p;
        if &0 > x || x > &(self.w as i32) || &0 > y || y > &(self.h as i32) {
            return None;
        }
        let idx = x + y * (self.w as i32);
        self.spots.get(idx as usize).cloned().flatten()
    }

    fn from_pattern(pattern: &[String]) -> Self {
        if let Some(line) = pattern.first() {
            Self {
                w: line.len(),
                h: pattern.len(),
                spots: pattern
                    .into_iter()
                    .flat_map(|it| it.chars())
                    .map(|c| match c {
                        '.' => None,
                        other => Some(other),
                    })
                    .collect(),
            }
        } else {
            Self {
                w: 0,
                h: 0,
                spots: Vec::new(),
            }
        }
    }
}

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn add(&self, p: &Point) -> Self {
        let Point(ax, ay) = self;
        let Point(bx, by) = p;
        Point(ax + bx, ay + by)
    }
}
