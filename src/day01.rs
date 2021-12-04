// day 01

use chumsky as c;
use chumsky::prelude::*;

// Format:
//   123
//   4567
//   ...
fn input_parser() -> impl Parser<char, Vec<usize>, Error = Simple<char>> {
    c::text::int(10)
        // Q: why do I need to specify `: String` for map?
        .map(|s: String| s.parse().unwrap())
        .separated_by(c::text::newline())
}

#[derive(Debug)]
enum IncDec {
    Increase,
    Decrease,
}

fn count_increasing_measures(measures: &[usize]) -> usize {
    measures
        .iter()
        .zip(measures.iter().skip(1))
        .map(|(previous, current)| {
            if current > previous {
                IncDec::Increase
            } else {
                IncDec::Decrease
            }
        })
        .filter(|inc_dec| matches!(inc_dec, IncDec::Increase))
        .count()
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let measures = input_parser().parse(raw_input).unwrap();
    // KEEP: ...parse().map_err(|errs| anyhow::anyhow!("parsing errors: {:?}", errs))?;

    let result = count_increasing_measures(&measures);
    // dbg!(result);
    (result, Some(1502))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let measures = input_parser().parse(raw_input).unwrap();
    // Smoothed measures by sum-ing measurements 3 by 3 and using that sum.
    let smoothed_measures: Vec<_> = measures
        .iter()
        .zip(measures.iter().skip(1))
        .zip(measures.iter().skip(2))
        .map(|((item0, item1), item2)| item0 + item1 + item2)
        .collect();

    let result = count_increasing_measures(&smoothed_measures);
    // dbg!(result);
    (result, Some(1538))
}
