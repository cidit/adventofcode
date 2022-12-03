
// edition/day/part

enum SolutionRouter {
   Solution(Solution),
   Router(Router),
}

struct Solution(fn(&str) -> String);

struct Router(Vec<(&'static str, SolutionRouter)>);

#[cfg(test)]
mod tests {

    use super::*;

    fn can_build() {
        let solutions = SolutionRouter::Router(Router(Vec::from([
            ("2015", SolutionRouter::Router(Router(Vec::from([
                ("1", SolutionRouter::Router(Router(Vec::from([
                    ("1", SolutionRouter::Solution(Solution(String::from))),
                    ("2", SolutionRouter::Solution(Solution(String::from))),
                ])))),
                ("2", SolutionRouter::Router(Router(Vec::from([
                    ("1", SolutionRouter::Solution(Solution(String::from))),
                    ("2", SolutionRouter::Solution(Solution(String::from))),
                ])))),
            ])))),
        ])));

    }

}
