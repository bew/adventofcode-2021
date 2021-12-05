// day 01

use chumsky as c;
use chumsky::prelude::*;

// Format:
//   123
//   4567
//   ...
fn input_parser() -> impl Parser<char, Vec<usize>, Error = Simple<char>> {
    c::text::int(10)
        // Q: Why do I need to specify `: String` for map?
        // Answer from author: (ref: https://github.com/zesterer/chumsky/discussions/40#discussioncomment-1750744)
        //   Many of the parsers in the text module are generic across Unicode (via char) and
        //   ASCII (via u8) characters, including ident. The String is required to tell text::int
        //   that it should be parsing chars and not u8s. You could also swap out String with Vec<u8>
        //   if you wanted a parser that parses integers using ASCII bytes.
        .map(|s: String| s.parse().unwrap())
        .separated_by(c::text::newline())
}

#[derive(Debug)]
enum IncDec {
    Increase,
    Decrease,
}

// NOTE instead of multiple zip iterator to make a sliding window, I could have used
// https://doc.rust-lang.org/std/primitive.slice.html#method.windows (which exists on Vec),
// main difference in usage is that it gives an iterator of slices, not of tuples, so I think I
// can't destructure it as easily in closures.

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
