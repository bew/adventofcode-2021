// day 02

use chumsky as c;
use chumsky::prelude::*;

#[derive(Debug)]
enum Cmd {
    Forward(usize),
    Up(usize),
    Down(usize),
}

// Format:
//   forward 5
//   down 5
//   forward 8
//   up 3
//   down 8
//   forward 2
//   ...
fn input_parser() -> impl Parser<char, Vec<Cmd>, Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let cmd_line = c::text::ident()
        .then_ignore(just(' '))
        .then(number)
        .map(|(cmd, by_count)| match cmd.as_str() {
            "forward" => Cmd::Forward(by_count),
            "up" => Cmd::Up(by_count),
            "down" => Cmd::Down(by_count),
            _ => unreachable!(),
        });
    cmd_line.separated_by(c::text::newline())
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let cmds = input_parser().parse(raw_input).unwrap();

    let (mut hpos, mut depth) = (0, 0);
    for cmd in cmds {
        match cmd {
            Cmd::Forward(by_count) => hpos += by_count,
            Cmd::Down(by_count) => depth += by_count,
            Cmd::Up(by_count) => depth -= by_count,
        }
    }

    (hpos * depth, Some(1813801))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let cmds = input_parser().parse(raw_input).unwrap();

    let (mut hpos, mut depth, mut aim) = (0, 0, 0);
    for cmd in cmds {
        match cmd {
            Cmd::Forward(by_count) => {
                hpos += by_count;
                depth += by_count * aim;
            }
            Cmd::Down(by_count) => aim += by_count,
            Cmd::Up(by_count) => aim -= by_count,
        }
    }

    (hpos * depth, Some(1960569556))
}
