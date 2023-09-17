use thiserror::Error;

pub fn p1(input: &str) -> String {
    let input = parse(input);
    input.iter().map(BoxDim::wrapping).sum::<i32>().to_string()
}

pub fn p2(input: &str) -> String {
    let input = parse(input);
    String::new()
}

fn parse(input: &str) -> Vec<BoxDim> {
    input
        .split('\n')
        .map(str::parse)
        // i want this to fail if i have any errors, hence no flat_map
        .map(Result::unwrap)
        .collect()
}

struct BoxDim(i32, i32, i32);

impl BoxDim {
    fn area(&self) -> i32 {
        self.sides().iter().map(|(&a, &b)| 2 * a * b).sum::<i32>()
    }

    fn sides(&self) -> [(&i32, &i32); 3] {
        let Self(l, w, h) = self;
        [(l, w), (w, h), (h, l)]
    }

    fn extra(&self) -> i32 {
        let Self(l, w, h) = self;
        let sides = [l*w, l*h, w*h];
        sides.into_iter().min().unwrap()
    }

    fn wrapping(&self) -> i32 {
        self.area() + self.extra()
    }
}

impl std::str::FromStr for BoxDim {
    type Err = BoxDimParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<i32> = s.split('x').flat_map(str::parse).collect();
        if dims.len() != 3 {
            return Err(BoxDimParseError::TooManyDimensions(dims.len()));
        }
        Ok(Self(dims[0], dims[1], dims[2]))
    }
}

#[derive(Error, Debug)]
enum BoxDimParseError {
    #[error("expected 3 dimentions, got {0}")]
    TooManyDimensions(usize),
}
