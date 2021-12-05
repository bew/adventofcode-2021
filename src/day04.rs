// day 04

use chumsky as c;
use chumsky::prelude::*;

type BingoNum = u8;

#[derive(Debug, Eq, PartialEq, Clone)]
struct BingoCell {
    num: BingoNum,
    marked: bool,
}
impl BingoCell {
    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct BingoBoard {
    lines: Vec<Vec<BingoCell>>,
}

impl BingoBoard {
    fn from_input(input_lines: Vec<Vec<BingoNum>>) -> Self {
        assert_eq!(input_lines.len(), 5);
        assert!(input_lines.iter().all(|line| line.len() == 5));

        let lines = input_lines
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|num| BingoCell { num, marked: false })
                    .collect()
            })
            .collect();
        BingoBoard { lines }
    }
    fn is_winning(&self) -> bool {
        let has_win_line = (0..5).any(|line| (0..5).all(|col| self.is_marked_at(line, col)));
        let has_win_col = (0..5).any(|col| (0..5).all(|line| self.is_marked_at(line, col)));
        has_win_line || has_win_col
    }
    fn is_marked_at(&self, line: u8, col: u8) -> bool {
        // println!("Checking cell is marked at (L{}, C{})", line, col);
        unsafe {
            self.lines
                .get_unchecked(line as usize)
                .get_unchecked(col as usize)
                .marked
        }
    }
    fn mark_with(&mut self, number: BingoNum) {
        for line in self.lines.iter_mut() {
            for cell in line.iter_mut() {
                if cell.num == number {
                    cell.mark();
                }
            }
        }
    }
    fn unmarked_nums(&self) -> Vec<BingoNum> {
        self.lines
            .iter()
            .flat_map(|line| {
                line.iter()
                    .filter_map(|cell| (!cell.marked).then(|| cell.num))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for cell in line {
                if cell.marked {
                    write!(f, "\x1b[34m{:2}\x1b[0m ", cell.num)?;
                } else {
                    write!(f, "{:2} ", cell.num)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// Format:
//   42,43,1,8,5         -- random numbers
//
//   22 13 17 11  0      -- board 1
//    8  2 23  4 24
//   21  9 14 16  7
//    6 10  3 18  5
//   11 12 20 15 19
//
//    3 27  1 93 42      -- board 2
//   21  0  3 18  5
//   ...
fn input_parser() -> impl Parser<char, (Vec<BingoNum>, Vec<BingoBoard>), Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let newline = c::text::newline();

    let random_numbers = number.separated_by(just(','));

    let board_line = just(' ')
        .ignore_then(number)
        .or(number)
        .separated_by(just(' '))
        // NOTE: at_least is necessary to not match the final newline followed by the end.
        .at_least(1);
    // .exactly(5); // <-------- MISSING METHOD, cf: https://github.com/zesterer/chumsky/issues/41

    // let board = board_line
    //     .separated_by(c::text::newline())
    //     .exactly(5) // <--------- MISSING METHOD, cf: https://github.com/zesterer/chumsky/issues/41
    //     .map(BingoBoard::from_input);
    let board = (newline.ignore_then(board_line))
        .repeated()
        .at_least(5)
        .at_most(5)
        .map(BingoBoard::from_input);

    let boards = board.separated_by(newline);

    random_numbers.then_ignore(newline).then(boards)
}

/// Returns an iterator over the results of winning boards.
/// The first result is for the first winning board.
/// The last result is for the last winning board.
///
/// NOTE: it works for the challenge but it's not perfect: if multiple boards end at the same
/// random number, it will _always_ give the first one as the ending board. It can't return the
/// 'true' last board if that happens at the very end.
fn get_boards_win_results(
    random_numbers: Vec<BingoNum>,
    mut boards: Vec<BingoBoard>,
) -> impl std::iter::Iterator<Item = (BingoNum, BingoBoard)> {
    random_numbers.into_iter().filter_map(move |rand_num| {
        // println!("== Marking remaining boards with rand number: {} ==", rand_num);
        boards.iter_mut().for_each(|board| {
            board.mark_with(rand_num);
            // println!("{}", board);
        });

        // Find a winning board
        let maybe_winning_board_idx = boards.iter().position(|board| board.is_winning());
        if let Some(winning_board_idx) = maybe_winning_board_idx {
            let winning_board = boards.remove(winning_board_idx);
            // Remove all other winning boards
            boards.retain(|board| !board.is_winning());
            // Return the first winning board for that random number
            Some((rand_num, winning_board))
        } else {
            // No winner for that random number
            None
        }
    })
}

fn calc_final_score(unmarked_nums: &[BingoNum], last_rand_num: BingoNum) -> usize {
    let sum_unmarked: usize = unmarked_nums.iter().map(|&n| n as usize).sum();
    sum_unmarked * (last_rand_num as usize)
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let (random_numbers, boards) = input_parser().parse(raw_input).expect("parsing error");

    let mut board_win_results = get_boards_win_results(random_numbers, boards);

    let (last_rand_num, win_board) = board_win_results.next().unwrap();
    let score = calc_final_score(&win_board.unmarked_nums(), last_rand_num);
    (score, Some(38594))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let (random_numbers, boards) = input_parser().parse(raw_input).expect("parsing error");

    let board_win_results = get_boards_win_results(random_numbers, boards);

    let (last_rand_num, win_board) = board_win_results.last().unwrap();
    let score = calc_final_score(&win_board.unmarked_nums(), last_rand_num);
    (score, Some(21184))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let input = r#"
42,43,1,8,5

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#;
        let (random_numbers, boards) = input_parser().parse(input.trim()).expect("parse error");
        assert_eq!(random_numbers, &[42, 43, 1, 8, 5]);
        fn num_line_to_bingocell_line(num_line: Vec<BingoNum>) -> Vec<BingoCell> {
            num_line
                .into_iter()
                .map(|num| BingoCell { num, marked: false })
                .collect()
        }
        let expected_board = BingoBoard {
            lines: vec![
                num_line_to_bingocell_line(vec![22, 13, 17, 11, 0]),
                num_line_to_bingocell_line(vec![8, 2, 23, 4, 24]),
                num_line_to_bingocell_line(vec![21, 9, 14, 16, 7]),
                num_line_to_bingocell_line(vec![6, 10, 3, 18, 5]),
                num_line_to_bingocell_line(vec![1, 12, 20, 15, 19]),
            ],
        };
        assert_eq!(boards, vec![expected_board]);
    }

    static EXAMPLE_INPUT: &str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn test_parse_example_input() {
        let (rand_nums, boards) = input_parser()
            .parse(EXAMPLE_INPUT.trim())
            .expect("parse error");
        assert_eq!(rand_nums.len(), 27);
        assert_eq!(boards.len(), 3);
    }

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT.trim());
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT.trim());
        assert_eq!(result, 1924);
    }
}
