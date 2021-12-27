// day 09

use std::collections::HashSet;

use chumsky as c;
use chumsky::prelude::*;

// Format:
//   2199943210
//   3987894921
//   ...
fn input_parser() -> impl Parser<char, HeightMap, Error = Simple<char>> {
    let digit = one_of("0123456789").map(|chr: char| chr.to_digit(10).unwrap_or(0));
    let heights_line = digit.repeated().at_least(1);
    heights_line
        .separated_by(c::text::newline())
        .map(HeightMap::new)
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct PosXY {
    pub x: i32,
    pub y: i32,
}
impl PosXY {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
// impl a oneline Debug representation of the position :)
impl std::fmt::Debug for PosXY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: to escape braces in format strings, it must be repeated.
        // ref: https://doc.rust-lang.org/std/fmt/index.html#escaping
        write!(f, "PosXY {{ x: {}, y: {} }}", &self.x, &self.y)
    }
}

#[derive(Debug)]
struct HeightMap {
    grid: Vec<Vec<u32>>,
}
impl HeightMap {
    pub fn new(grid: Vec<Vec<u32>>) -> Self {
        Self { grid }
    }

    pub fn get_at(&self, pos: &PosXY) -> Option<&u32> {
        self.grid
            .get(pos.y as usize)
            .and_then(|grid_line| grid_line.get(pos.x as usize))
    }

    // NOTE: `+ '_` on the return value declares that the return type captures data from `self` and
    // thus they must have the same lifetime.
    // Basically means `and it has the same lifetime as self` (necessary when not obvious)
    pub fn iter_heights(&self) -> impl Iterator<Item = (PosXY, u32)> + '_ {
        self.grid.iter().enumerate().flat_map(|(pos_y, line)| {
            line.iter()
                .enumerate()
                // NOTE: move is necessary to force the closure to take ownership of `pos_y`
                // (and `line`, but it's a ref so no problem)
                .map(move |(pos_x, height)| {
                    let pos = PosXY::new(pos_x as i32, pos_y as i32);
                    (pos, *height)
                })
        })
    }

    pub fn iter_lowest_heights_points(&self) -> impl Iterator<Item = (PosXY, u32)> + '_ {
        self.iter_heights().filter(|(pos, current_height)| {
            self.iter_neighbours_heights_points(pos)
                .all(|(_pos, height)| height > *current_height)
        })
    }

    pub fn iter_neighbours_heights_points(
        &self,
        pos: &PosXY,
    ) -> impl Iterator<Item = (PosXY, u32)> + '_ {
        let neighbours_positions = [
            PosXY::new(pos.x - 1, pos.y), // left
            PosXY::new(pos.x + 1, pos.y), // right
            PosXY::new(pos.x, pos.y - 1), // up
            PosXY::new(pos.x, pos.y + 1), // down
        ];
        // keep only this item if all its neighbours are greater than 'current_height'
        neighbours_positions
            .into_iter()
            .filter_map(|pos| self.get_at(&pos).and_then(|height| Some((pos, *height))))
    }

    pub fn get_basin_size_from(&self, from_pos: &PosXY) -> usize {
        let mut points_in_basin = HashSet::new();
        points_in_basin.insert(*from_pos);
        self.accumulate_points_in_basin(from_pos, &mut points_in_basin);
        points_in_basin.len()
    }

    fn accumulate_points_in_basin(&self, from_pos: &PosXY, points_in_basin: &mut HashSet<PosXY>) {
        for (neigh_pos, height) in self.iter_neighbours_heights_points(from_pos) {
            if height < 9 {
                if points_in_basin.insert(neigh_pos) {
                    // `neigh_pos` was inserted, so it wasn't known to be in the basin yet and we
                    // need to recurse to check its neighbours.
                    self.accumulate_points_in_basin(&neigh_pos, points_in_basin);
                }
            }
        }
    }
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let heightmap = input_parser().parse(raw_input).unwrap();

    let lowest_points = heightmap.iter_lowest_heights_points();
    let risk_levels = lowest_points.map(|(_pos, height)| 1 + height);

    (risk_levels.sum::<u32>() as usize, Some(541))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    // NOTE: This implementation is enough for this AoC, but it's not perfect:
    //   => If 2 lowest points are part of the same basin, there will be 2 basins counted
    //      instead of 1.

    let heightmap = input_parser().parse(raw_input).unwrap();

    let lowest_points = heightmap.iter_lowest_heights_points();
    let basin_sizes = lowest_points.map(|(pos, _height)| heightmap.get_basin_size_from(&pos));

    let basin_sizes_sorted = {
        let mut buffer = basin_sizes.collect::<Vec<_>>();
        buffer.sort();
        buffer.reverse(); // to have biggest first
        buffer
    };
    // dbg!(&basin_sizes_sorted);

    let result = basin_sizes_sorted
        .into_iter()
        .take(3)
        .reduce(|a, b| a * b)
        // necessary as reduce can theorically return None (but not here) if not enough values.
        .unwrap();
    (result, Some(847504))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT.trim());
        assert_eq!(result, 15);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT.trim());
        assert_eq!(result, 1134);
    }
}
