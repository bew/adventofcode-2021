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
fn input_parser() -> impl Parser<char, (u8, Vec<usize>), Error = Simple<char>> {
    let bits = just('0').or(just('1')).repeated().at_least(1);
    let bit_based_number = bits.collect::<String>().map(|s: String| {
        let num_from_bits = usize::from_str_radix(&s, 2).unwrap();
        (s.len() as u8, num_from_bits)
    });
    let report_parser = bit_based_number.separated_by(c::text::newline());
    // extract the numbers and the bit length of the numbers, to be directly usable :)
    report_parser.map(|diag_report: Vec<(u8, usize)>| {
        let max_len = diag_report.iter().cloned().map(|(len, _)| len).max().unwrap();
        let numbers: Vec<usize> = diag_report.iter().cloned().map(|(_, number)| number).collect();
        (max_len, numbers)
    })
}

#[derive(Debug)]
enum BitPopularity {
    Zero,
    One,
    Equal,
}
impl BitPopularity {
    fn value_or(&self, default_value: u8) -> u8 {
        match self {
            BitPopularity::One => 1,
            BitPopularity::Zero => 0,
            BitPopularity::Equal => default_value,
        }
    }
    fn invert_popularity(&self) -> BitPopularity {
        match self {
            BitPopularity::One => BitPopularity::Zero,
            BitPopularity::Zero => BitPopularity::One,
            BitPopularity::Equal => BitPopularity::Equal,
        }
    }
}

fn number_get_bit_at_idx(number: usize, bit_idx: u8) -> usize {
    (number & (1 << bit_idx)) >> bit_idx
}

fn bit_popularity_at_idx(numbers: &[usize], bit_idx: u8) -> BitPopularity {
    let mut is1_diff = 0;
    for num in numbers {
        if number_get_bit_at_idx(*num, bit_idx) == 1 {
            is1_diff += 1;
        } else {
            is1_diff -= 1;
        }
    }
    match is1_diff {
        n if n > 0 => BitPopularity::One,
        n if n == 0 => BitPopularity::Equal,
        n if n < 0 => BitPopularity::Zero,
        _ => unreachable!(),
    }
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let (max_len, numbers) = input_parser().parse(raw_input).unwrap();

    let mut gamma_rate = 0 as usize;
    for bit_idx in (0u8..max_len).rev() {
        if let BitPopularity::One | BitPopularity::Equal = bit_popularity_at_idx(&numbers, bit_idx) {
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
    let (max_len, numbers) = input_parser().parse(raw_input).unwrap();

    let oxygen_generator_rating = {
        let mut numbers = numbers.clone();
        for bit_idx in (0u8..max_len).rev() {
            let bit_popularity = bit_popularity_at_idx(&numbers, bit_idx);
            let bit_criteria = bit_popularity.value_or(1); // criteria when Equal: 1
            numbers.retain(|num| number_get_bit_at_idx(*num, bit_idx) == bit_criteria.into());

            // let numbers_as_bits: Vec<String> = numbers.iter().map(|num| format!("{:#012b}", num)).collect();
            // dbg!(bit_idx, bit_popularity, bit_criteria, &numbers_as_bits);

            if numbers.len() == 1 { break }
        }
        // There should be only one number left, take it out!
        numbers.pop().unwrap()
    };
    // dbg!(oxygen_generator_rating);
    // println!("-------");

    let co2_scrubber_rating = {
        let mut numbers = numbers.clone();
        for bit_idx in (0u8..max_len).rev() {
            let bit_popularity = bit_popularity_at_idx(&numbers, bit_idx);
            let bit_least_popularity = bit_popularity.invert_popularity();
            // In this case we look for the least popularity:
            let bit_criteria = bit_least_popularity.value_or(0); // criteria when Equal: 0
            numbers.retain(|num| number_get_bit_at_idx(*num, bit_idx) == bit_criteria.into());

            // let numbers_as_bits: Vec<String> = numbers.iter().map(|num| format!("{:#012b}", num)).collect();
            // dbg!(bit_idx, bit_popularity, bit_criteria, &numbers_as_bits);

            if numbers.len() == 1 { break }
        }
        // There should be only one number left, take it out!
        numbers.pop().unwrap()
    };
    // dbg!(co2_scrubber_rating);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
    (life_support_rating, Some(4790390))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let res = input_parser().parse("0110\n01010\n").unwrap();
        dbg!(&res);
        assert!(res == (5, vec![0x6, 0xA]));
    }

    #[test]
    fn test_get_bit_at_index() {
        //            idx: >>|76543210|<<
        static NUM: usize = 0b00110100;
        assert!(number_get_bit_at_idx(NUM, 0) == 0);
        assert!(number_get_bit_at_idx(NUM, 1) == 0);
        assert!(number_get_bit_at_idx(NUM, 2) == 1);
        assert!(number_get_bit_at_idx(NUM, 3) == 0);
        assert!(number_get_bit_at_idx(NUM, 4) == 1);
        assert!(number_get_bit_at_idx(NUM, 5) == 1);
        assert!(number_get_bit_at_idx(NUM, 6) == 0);
        assert!(number_get_bit_at_idx(NUM, 7) == 0);
    }

    // NOTE: didn't find a simple way to unindent a set of indented lines.. for the
    // original input_parser to work.
    static EXAMPLE_INPUT: &str = r#"00100
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

    #[test]
    fn example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT);
        assert!(result == 198);
    }

    #[test]
    fn example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT);
        assert!(result == 230);
    }
}
