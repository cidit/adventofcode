use core::panic;

pub fn p1(input: &str) -> String {
    let instructions = parse(input);
    let mut direction = Direction::North;
    let mut position = Point { x: 0, y: 0 };

    for instruction in instructions {
        println!("processing instruction: {instruction:?}");
        direction = direction.apply(&instruction);
        let variance: Point = direction
            .variance()
            .multiply_by_scalar(instruction.distance);
        println!("variance: {variance:?}");
        position = Point {
            x: position.x + variance.x,
            y: position.y + variance.y,
        };
    }

    return format!(
        "the answer is: {} blocks away",
        position.x.abs() + position.y.abs()
    );
}

pub fn p2(input: &str) -> String {
    let instructions = parse(input);
    let mut direction = Direction::North;
    let mut position = Point { x: 0, y: 0 };
    let mut visited: Vec<Point> = vec![];
    let mut first_revisited_location = None;

    'instructions_loop: for instruction in instructions {
        direction = direction.apply(&instruction);
        let variance: Point = direction
            .variance()
            .multiply_by_scalar(instruction.distance);

        let new_pos = Point {
            x: position.x + variance.x,
            y: position.y + variance.y,
        };

        let visiting: Vec<Point> = {
            let mut visiting = vec![];

            for x in position.x.min(new_pos.x)..=position.x.max(new_pos.x) {
                for y in position.y.min(new_pos.y)..=position.y.max(new_pos.y) {
                    visiting.push(Point { x, y });
                }
            }
            // we want to excludre the starting square from the list because it will always overlap with the end of the last list
            visiting.into_iter().filter(|v| v != &position).collect()
        };

        for visit in visiting.iter().skip(1) {
            if visited.contains(&visit) {
                first_revisited_location = Some(visit.clone());
                break 'instructions_loop;
            } else {
                visited.push(visit.clone());
            }
        }

        position = new_pos;
    }

    return format!(
        "the answer is: {}",
        first_revisited_location
            .map(|p| format!("{} blocks away", p.x.abs() + p.y.abs()))
            .unwrap_or("unexistant".into())
    );
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .split(", ")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[derive(Debug)]
struct Instruction {
    rotation: Rotation,
    distance: usize,
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, number) = s.trim().split_at(1);
        let number: usize = number.parse().unwrap();
        match instruction {
            "L" => Ok(Instruction {
                rotation: Rotation::Left,
                distance: number,
            }),
            "R" => Ok(Instruction {
                rotation: Rotation::Right,
                distance: number,
            }),
            _other => panic!("Unexpected direction"),
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn variance(&self) -> Point {
        use Direction::*;
        match self {
            North => Point { x: 0, y: 1 },
            East => Point { x: 1, y: 0 },
            South => Point { x: 0, y: -1 },
            West => Point { x: -1, y: 0 },
        }
    }

    fn apply(&self, instruction: &Instruction) -> Self {
        use Rotation::*;
        match instruction.rotation {
            Left => self.left(),
            Right => self.right(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn multiply_by_scalar(&self, scalar: usize) -> Self {
        Self {
            x: self.x * scalar as i32,
            y: self.y * scalar as i32,
        }
    }
}
