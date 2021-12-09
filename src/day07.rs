// day 06

use chumsky as c;
use chumsky::prelude::*;

// Format:
//   1,2,3,4 ...
fn input_parser() -> impl Parser<char, Vec<i32>, Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    number.separated_by(just(','))
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let input = input_parser().parse(raw_input).unwrap();
    let min = input.iter().min().cloned().unwrap();
    let max = input.iter().max().cloned().unwrap();

    // Brute force impl..
    // For each horiz position (hpos), sum the distances of that hpos to all crabs
    // Then find the smallest sum of distances.
    let res = (min..=max)
        .map(|hpos| input.iter().map(|x| (x - hpos).abs()).sum::<i32>())
        .min()
        .unwrap();

    (res as usize, Some(356179))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let input = input_parser().parse(raw_input).unwrap();
    let min = input.iter().min().cloned().unwrap();
    let max = input.iter().max().cloned().unwrap();

    // Brute force impl..
    // For each horiz position (hpos), sum the fuel used for distance (by 1+2+3...) of that hpos to all crabs
    // Then find the smallest sum of distances.
    let res = (min..=max)
        .map(|hpos| {
            input
                .iter()
                .map(|x| (0..=(x - hpos).abs()).sum::<i32>())
                .sum::<i32>()
        })
        .min()
        .unwrap();

    (res as usize, Some(99788435))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT);
        assert_eq!(result, 168);
    }
}
