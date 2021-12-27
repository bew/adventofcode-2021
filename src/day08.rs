// day 06

use std::collections::{HashMap, HashSet};

use chumsky as c;
use chumsky::prelude::*;
use lazy_static::lazy_static;

type SignalPattern = HashSet<char>;

#[derive(Debug, Clone)]
struct InputLine {
    init: Vec<SignalPattern>,
    output: Vec<SignalPattern>,
}

fn fmt_signals(sigs: &SignalPattern) -> String {
    format!("Signals[{}]", String::from_iter(sigs))
}

// Format: (<init numbers> | <output numbers>)
//   ceb bgfdea febgc ec eadcgfb eagbcd fcdebg dcef gafbc egdbf | fdbgec fedbg gdabefc gefbd
//   af cegdabf cfdge ecdbfg dcfga edafgc cfa cabedf gdbac afge | cgdab bcagd badecgf fa
//   ...
fn input_parser() -> impl Parser<char, Vec<InputLine>, Error = Simple<char>> {
    let signal_pattern = one_of("abcdefg")
        .repeated()
        .at_least(1)
        .map(|chars| HashSet::from_iter(chars));
    let patterns = signal_pattern.separated_by(just(' ')).at_least(1);
    let input_line = patterns
        // NOTE: clone() is necessary because parsers are moved when combined,
        //       and since I use it after (and parsers do not impl Copy),
        //       I'd get a `use after move` compilation error.
        .clone()
        .then_ignore(just(" | "))
        .then(patterns)
        .map(|(init, output)| InputLine { init, output });
    input_line.separated_by(c::text::newline())
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let input_lines = input_parser().parse(raw_input).unwrap();

    let res = input_lines
        .into_iter()
        .map(|l| l.output)
        .flatten()
        .filter(|sig_pattern| [2, 3, 4, 7].contains(&sig_pattern.len()))
        .count();

    (res, Some(349))
}

lazy_static! {
    static ref DIGITS_SEGMENTS: [HashSet<char>; 10] = [
        HashSet::from_iter(['a', 'b', 'c', /**/ 'e', 'f', 'g']), // 0
        HashSet::from_iter([/**/ /**/ 'c', /**/ /**/ 'f' /**/]), // 1  | unique nb segs
        HashSet::from_iter(['a', /**/ 'c', 'd', 'e', /**/ 'g']), // 2
        HashSet::from_iter(['a', /**/ 'c', 'd', /**/ 'f', 'g']), // 3
        HashSet::from_iter([/**/ 'b', 'c', 'd', /**/ 'f' /**/]), // 4  | unique nb segs
        HashSet::from_iter(['a', 'b', /**/ 'd', /**/ 'f', 'g']), // 5
        HashSet::from_iter(['a', 'b', /**/ 'd', 'e', 'f', 'g']), // 6
        HashSet::from_iter(['a', /**/ 'c', /**/ /**/ 'f' /**/]), // 7  | unique nb segs
        HashSet::from_iter(['a', 'b', 'c', 'd', 'e', 'f', 'g']), // 8  | unique nb segs
        HashSet::from_iter(['a', 'b', 'c', 'd', /**/ 'f', 'g']), // 9
    ];
}

#[derive(Debug)]
struct WireMapper {
    digit_to_signal_pattern: HashMap<u32, SignalPattern>,
}
impl WireMapper {
    pub fn new() -> Self {
        WireMapper {
            digit_to_signal_pattern: HashMap::new(),
        }
    }

    /// Search & return the pattern with given length.
    /// NOTE: Crash if it cannot find the pattern.
    fn pop_pattern_with_len(patterns: &mut Vec<SignalPattern>, len: usize) -> SignalPattern {
        let pos = patterns.iter().position(|pat| pat.len() == len);
        patterns.remove(pos.unwrap())
    }

    pub fn guess_from_init(&mut self, init_patterns: &[SignalPattern]) {
        let mut patterns_left_to_guess: Vec<SignalPattern> =
            init_patterns.iter().cloned().collect();

        let pattern_for_1 = Self::pop_pattern_with_len(&mut patterns_left_to_guess, 2);
        let pattern_for_4 = Self::pop_pattern_with_len(&mut patterns_left_to_guess, 4);
        let pattern_for_7 = Self::pop_pattern_with_len(&mut patterns_left_to_guess, 3);
        let pattern_for_8 = Self::pop_pattern_with_len(&mut patterns_left_to_guess, 7);
        // patterns for 1 & 4 are cloned because we need them later for comparisons
        self.digit_to_signal_pattern
            .insert(1, pattern_for_1.clone());
        self.digit_to_signal_pattern
            .insert(4, pattern_for_4.clone());
        self.digit_to_signal_pattern.insert(7, pattern_for_7);
        self.digit_to_signal_pattern.insert(8, pattern_for_8);

        // Now, deduce other digits from common segments...
        // PATTERNS LEFT TO FIND: 0 2 3 5 6 9

        for pat in patterns_left_to_guess {
            if pat.len() == 6 {
                // Patterns with len 6: can be either 0, 6 or 9.
                if pat.is_superset(&pattern_for_4) {
                    self.digit_to_signal_pattern.insert(9, pat);
                } else if pat.is_superset(&pattern_for_1) {
                    self.digit_to_signal_pattern.insert(0, pat);
                } else {
                    self.digit_to_signal_pattern.insert(6, pat);
                }
            } else if pat.len() == 5 {
                // Patterns with len 5: can be either 2, 3 or 5
                if pat.is_superset(&pattern_for_1) {
                    self.digit_to_signal_pattern.insert(3, pat);
                } else if pat.intersection(&pattern_for_4).count() == 3 {
                    // 5 has 3 common segments with 4, but only 2 with 2, so this is 5.
                    self.digit_to_signal_pattern.insert(5, pat);
                } else {
                    self.digit_to_signal_pattern.insert(2, pat);
                }
            }
        }
    }

    pub fn resolve_signal_pattern_to_digit(&self, signal_pattern: &SignalPattern) -> u32 {
        let r#match = self
            .digit_to_signal_pattern
            .iter()
            .find(|(_, pattern)| signal_pattern == *pattern);
        match r#match {
            Some((digit, _)) => *digit,
            None => panic!(
                "Cannot find digit for signal pattern: {}",
                fmt_signals(signal_pattern)
            ),
        }
    }
}

fn guess_and_resolve_output(input_line: InputLine) -> usize {
    let mut sig_mapper = WireMapper::new();
    sig_mapper.guess_from_init(&input_line.init);
    let output_digits: Vec<u32> = input_line
        .output
        .iter()
        .map(|pattern| sig_mapper.resolve_signal_pattern_to_digit(pattern))
        .collect();
    // dbg!(&output_digits);

    let output_number = output_digits
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, item)| {
            acc + (item * 10_u32.pow(3 - (idx as u32)))
        });

    output_number as usize
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let input_lines = input_parser().parse(raw_input).unwrap();

    let sum_all_outputs = input_lines.into_iter().map(guess_and_resolve_output).sum();
    (sum_all_outputs, Some(1070957))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT.trim());
        assert_eq!(result, 26);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT.trim());
        assert_eq!(result, 61229);
    }

    #[test]
    fn test_short_example_part2() {
        let (result, _) = solve_part2(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(result, 5353);
    }
}
