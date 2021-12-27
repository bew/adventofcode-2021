// Day not_a_day

use chumsky as c;
use chumsky::prelude::*;

// Format:
//   foo
//   barbar
//   ...
fn input_parser() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    c::text::ident()
        .separated_by(c::text::newline())
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let lines = input_parser().parse(raw_input).unwrap();

    (0, None)
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let lines = input_parser().parse(raw_input).unwrap();

    (0, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
"#;

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT.trim());
        assert_eq!(result, 42);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT.trim());
        assert_eq!(result, 42);
    }
}
