// Day 10

use chumsky as c;
use chumsky::prelude::*;

enum Symbol {
    Open(char),
    Close(char),
}
impl Symbol {
    const OPENS: [char; 4] = ['(', '[', '{', '<'];
    const CLOSES: [char; 4] = [')', ']', '}', '>'];

    pub fn from_char(chr: char) -> Self {
        match chr {
            '(' | '[' | '{' | '<' => Self::Open(chr),
            ')' | ']' | '}' | '>' => Self::Close(chr),
            _ => unreachable!(),
        }
    }

    pub fn matching_pair(chr: &char) -> char {
        if let Some(pos) = Self::OPENS.iter().position(|c| c == chr) {
            Self::CLOSES[pos]
        } else if let Some(pos) = Self::CLOSES.iter().position(|c| c == chr) {
            Self::OPENS[pos]
        } else {
            unreachable!()
        }
    }

    #[allow(dead_code)] // because it's used in commented debug lines
    pub fn to_char(&self) -> char {
        let (Symbol::Open(chr) | Symbol::Close(chr)) = self;
        *chr
    }
}
impl std::fmt::Debug for Symbol {
    // Debug repr on a single line.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Open(chr) => write!(f, "Symbol::Open({:?})", chr),
            Symbol::Close(chr) => write!(f, "Symbol::Close({:?})", chr),
        }
    }
}

// Format:
//   [({(<(())[]>[[{[]{<()<>>
//   [(()[<>])]({[<{<<[]>>(
//   ...
fn input_parser() -> impl Parser<char, Vec<Vec<Symbol>>, Error = Simple<char>> {
    let token = one_of("([{<>}])").map(Symbol::from_char);
    let line = token.repeated().at_least(1);
    line.separated_by(c::text::newline())
}

#[derive(Debug)]
enum Report {
    Corrupted(char),
    Incomplete { missing_chars: Vec<char> },
    Valid, // TODO: add chuncks?
}

fn analyze_line(line: &[Symbol]) -> Report {
    let mut opener_stack: Vec<char> = vec![];
    for sym in line {
        match sym {
            Symbol::Open(chr) => opener_stack.push(*chr),
            Symbol::Close(close_chr) => {
                if let Some(last_open_chr) = opener_stack.pop() {
                    if Symbol::matching_pair(&close_chr) == last_open_chr {
                        // continue
                    } else {
                        return Report::Corrupted(*close_chr);
                    }
                } else {
                    return Report::Corrupted(*close_chr);
                }
            }
        }
    }
    if !opener_stack.is_empty() {
        let missing_chars = opener_stack
            .iter()
            .rev()
            .map(Symbol::matching_pair)
            .collect();
        Report::Incomplete { missing_chars }
    } else {
        Report::Valid
    }
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let lines = input_parser().parse(raw_input).unwrap();

    // for line in &lines {
    //     let line_str = line.iter().map(Symbol::to_char).collect::<String>();
    //     println!("Line '{}' is {:?}", line_str, analyze_line(&line));
    // }

    let corruption_score: usize = lines
        .iter()
        .map(|line| analyze_line(line))
        // keep only Corrupted entires
        .filter(|rep| matches!(rep, Report::Corrupted(..)))
        .map(|rep| {
            let syntax_error_points = match rep {
                Report::Corrupted(')') => 3,
                Report::Corrupted(']') => 57,
                Report::Corrupted('}') => 1197,
                Report::Corrupted('>') => 25137,
                _ => unreachable!(), // We kept only Corrupted variants
            };
            syntax_error_points
        })
        .sum();

    (corruption_score, Some(168417))
}

fn closing_char_to_completion_score(chr: char) -> usize {
    match chr {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let lines = input_parser().parse(raw_input).unwrap();

    let mut lines_completion_scores = lines
        .iter()
        .map(|line| analyze_line(line))
        .filter_map(|rep| match rep {
            Report::Incomplete { missing_chars } => {
                let completion_score: usize = missing_chars.iter().fold(0, |acc, chr| {
                    acc * 5 + closing_char_to_completion_score(*chr)
                });
                Some((missing_chars, completion_score))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    lines_completion_scores.sort_by_key(|(_, score)| *score);

    // for (missing_chars, compl_score) in &lines_completion_scores {
    //     let chars_str = missing_chars.iter().collect::<String>();
    //     println!(
    //         "Missing chars '{}', completion score: {}",
    //         chars_str, compl_score
    //     );
    // }

    // Keep only the scores
    let lines_completion_scores = lines_completion_scores
        .iter()
        .map(|(_, score)| score)
        .collect::<Vec<_>>();

    let mid_index = ((lines_completion_scores.len() - 1) as f64 / 2_f64).ceil() as usize;
    (*lines_completion_scores[mid_index], Some(2802519786))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT.trim());
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT.trim());
        assert_eq!(result, 288957);
    }
}
