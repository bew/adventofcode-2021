// day 05

use std::collections::HashMap;

use chumsky as c;
use chumsky::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point2D {
    pub x: i32,
    pub y: i32,
}
impl Point2D {
    fn from_coords((x, y): (i32, i32)) -> Self {
        Point2D { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct VentLine {
    start: Point2D,
    end: Point2D,
}
impl VentLine {
    fn between_points((p1, p2): (Point2D, Point2D)) -> Self {
        VentLine { start: p1, end: p2 }
    }

    fn is_horiz_or_vert(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    // NOTE: I don't like this 'imperative' way of doing this... But I can't wrap my head around
    //       an iterator-friendly way for now...
    fn points(&self) -> Vec<Point2D> {
        let mut points_on_vent = vec![];

        use std::cmp::Ordering;
        let increment_between = |start: i32, end: i32| match start.cmp(&end) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let x_increment = increment_between(self.start.x, self.end.x);
        let y_increment = increment_between(self.start.y, self.end.y);

        // println!("Points from: ({}, {}) -> ({}, {}) [x incr: {} | y incr: {}]", self.start.x, self.start.y, self.end.x, self.end.y, x_increment, y_increment);

        let (mut x, mut y) = (self.start.x, self.start.y);
        while x != self.end.x || y != self.end.y {
            // println!(" * ({}, {})", x, y);
            points_on_vent.push(Point2D { x, y });
            x += x_increment;
            y += y_increment;
        }
        // println!(" * ({}, {})", x, y);
        points_on_vent.push(Point2D { x, y }); // and last point!

        points_on_vent
    }
}

#[derive(Debug)]
struct OceanMap {
    known_vent_points: HashMap<Point2D, i32>,
}
impl OceanMap {
    fn new() -> Self {
        Self {
            known_vent_points: HashMap::new(),
        }
    }

    fn register_hydrothermal_vent(&mut self, vent: &VentLine) {
        for vent_point in vent.points() {
            let nb_vents = self.known_vent_points.entry(vent_point).or_insert(0);
            *nb_vents += 1;
        }
    }

    fn count_dangerous_vent_points(&self) -> usize {
        self.known_vent_points
            .iter()
            .filter(|(_, &nb_vents)| nb_vents >= 2)
            .count()
    }
}

impl std::fmt::Display for OceanMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.known_vent_points.keys().map(|p| p.x).max().unwrap();
        let max_y = self.known_vent_points.keys().map(|p| p.y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                match self.known_vent_points.get(&Point2D { x, y }) {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Format:
//   1,2 -> 3,4
//   ...
fn input_parser() -> impl Parser<char, Vec<VentLine>, Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let point = (number.then_ignore(just(',')).then(number)).map(Point2D::from_coords);
    let arrow = (just(' ').then(just('-')).then(just('>')).then(just(' '))).ignored(); // TODO: replace with 'keyword'
    let ventline = (point.then_ignore(arrow).then(point)).map(VentLine::between_points);
    ventline.separated_by(c::text::newline())
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let known_vents = input_parser().parse(raw_input).unwrap();

    let mut map = OceanMap::new();
    for vent in known_vents.into_iter().filter(|v| v.is_horiz_or_vert()) {
        map.register_hydrothermal_vent(&vent);
    }
    // println!("{}", map);
    (map.count_dangerous_vent_points(), Some(6283))
}

pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let known_vents = input_parser().parse(raw_input).unwrap();

    let mut map = OceanMap::new();
    for vent in known_vents {
        map.register_hydrothermal_vent(&vent);
    }
    // println!("{}", map);
    (map.count_dangerous_vent_points(), Some(18864))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
    "#;

    // Part1 map:
    //
    // .......1..
    // ..1....1..
    // ..1....1..
    // .......1..
    // .112111211
    // ..........
    // ..........
    // ..........
    // ..........
    // 222111....

    // Part2 map:
    //
    // 1.1....11.
    // .111...2..
    // ..2.1.111.
    // ...1.2.2..
    // .112313211
    // ...1.2....
    // ..1...1...
    // .1.....1..
    // 1.......1.
    // 222111....

    #[test]
    fn test_parsing() {
        let vent_lines = input_parser().parse(EXAMPLE_INPUT.trim()).unwrap();
        assert_eq!(
            vent_lines.first(),
            Some(&VentLine {
                start: Point2D { x: 0, y: 9 },
                end: Point2D { x: 5, y: 9 }
            })
        );
    }

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT.trim());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT.trim());
        assert_eq!(result, 12);
    }
}
