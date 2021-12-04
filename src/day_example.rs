// Example day :)

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
