// day 03

use chumsky as c;
use chumsky::prelude::*;

// Format:
//   00100...
//   11110...
//   10110...
//   10111...
//   10101...
//   01111...
//   00111...
//   11100...
//   10000...
//   11001...
//   00010...
//   01010...
//   ...
fn input_parser() -> impl Parser<char, Vec<(u8, usize)>, Error = Simple<char>> {
    let bits = just('0').or(just('1')).repeated().at_least(1);
    let bit_based_number = bits.collect::<String>().map(|s: String| {
        let num_from_bits = usize::from_str_radix(&s, 2).unwrap();
        (s.len() as u8, num_from_bits)
    });
    bit_based_number.separated_by(c::text::newline())
}

fn most_common_bit_in_column(numbers: &[usize], column: u8) -> u8 {
    let mut is1_diff = 0;
    for num in numbers {
        let is1 = num & (1 << column) != 0;
        if is1 {
            is1_diff += 1;
        } else {
            is1_diff -= 1;
        }
    }
    if is1_diff >= 0 { 1 } else { 0 }
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let diag_report = input_parser().parse(raw_input).unwrap();
    let max_len = diag_report.iter().cloned().map(|(len, _)| len).max().unwrap();
    let numbers: Vec<usize> = diag_report.iter().cloned().map(|(_, number)| number).collect();
    // dbg!(&numbers);
    // dbg!(max_len);

    let mut gamma_rate = 0 as usize;
    for bit_idx in (0u8..max_len).rev() {
        if most_common_bit_in_column(&numbers, bit_idx) == 1 {
            // println!("most common is 1 for bit idx: {}", bit_idx);
            gamma_rate += 1 << bit_idx;
        }
    }
    // dbg!(gamma_rate);

    // Q: Is there a better way to get a number with all-1 bits of size `max_len` ?
    // (e.g for size 5: 0b11111)
    let max_len_mask_of_1 = (0..max_len).fold(0, |acc, idx| acc | (1 << idx));
    // println!("mask of 1 (size {}): {:#012b}", max_len, max_len_mask_of_1);

    let epsilon_rate = max_len_mask_of_1 ^ gamma_rate; // invert all `max_len` bits
    // dbg!((gamma_rate, epsilon_rate));

    let power_consumption = gamma_rate * epsilon_rate;
    (power_consumption, Some(841526))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let _diag_report = input_parser().parse(raw_input).unwrap();

    (0, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let res = input_parser().parse("0110\n01010\n").unwrap();
        dbg!(&res);
        assert!(res == &[(4, 0x6), (5, 0xA)]);
    }

    #[test]
    fn example_part1() {
        // NOTE: didn't find a simple way to unindent a set of indented lines.. for the
        // original input_parser to work.
        let raw_input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;
        dbg!(solve_part1(raw_input));
        // NOTE: no useful asserts, I use that test to see debug output only..
        // assert!(false); // make it fail to see output..
    }
}
