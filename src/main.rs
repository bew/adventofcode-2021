use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

use anyhow::Result as AnyResult;

// These lines DECLARE the modules of my app
mod day01;
mod day02;

// NOTE: We can't pass generic type that impl Read, so we pass a trait object, which will use
//       dynamic dispatch on use.
type PartFn = fn(&str) -> (usize, Option<usize>);

// We define a lifetime in 'Day', to be able to store references in the struct.
struct Day<'a> {
    name: &'a str,
    part1: PartFn,
    part2: PartFn,
    default_input: &'a str,
}

macro_rules! def_day {
    ($d: ident) => {
        Day {
            name: stringify!($d),
            part1: $d::solve_part1,
            part2: $d::solve_part2,
            default_input: concat!("./inputs/", stringify!($d), ".txt"),
        }
    };
}

// NOTE: We use a slice of the array to not have to specify its size in the type.
//       See: https://stackoverflow.com/questions/23810032/how-to-specify-const-array-in-global-scope-in-rust
//
// NOTE: I tried to create a `Day::new` function to simplify creation of `Day` but the compiler
//       yelled at me about the fact that only constant things can be called for a static
//       variable, and if I change the constructor to be a const function, it tells me that
//       passing functions (like `day01::solve`) to a const function is unstable and not well
//       supported.. So struct constructor it is! :D
static DAYS: &[Day] = &[
    // Day {
    //     name: "day01",
    //     part1: day01::solve_part1,
    //     part2: day01::solve_part2,
    //     default_input: "./inputs/day01.txt",
    // },
    // same as:
    def_day!(day01),
    def_day!(day02),
];

fn print_usage() {
    let prog_name = env::args().next().unwrap_or("prog".to_string());
    let day_names: Vec<_> = DAYS.iter().map(|d| d.name).collect();
    println!("Usage: {} <day> [<custom_input_path>]", prog_name);
    println!(
        "Where <day> is either 'all', 'last' or one of: {}",
        day_names.join(", ")
    );
    exit(1);
}

fn run_part(part: &str, part_func: PartFn, input: &str) {
    let (result, maybe_expected) = (part_func)(input);
    match (result, maybe_expected) {
        (value, Some(expected)) => {
            if value == expected {
                println!("✅ {}: {} (same as expected)", part, value)
            } else {
                println!("❌ {}: Expected {} but got {} !!", part, expected, value);
            }
        }
        (value, None) => eprintln!("-- {}: {} ?", part, value),
    };
}

fn run_day_with_input_path(day: &Day, input_path: &Path) -> AnyResult<()> {
    println!("=>> {} <<=", day.name);
    let buf = std::fs::read_to_string(input_path)?;
    run_part("Part1", day.part1, &buf);
    run_part("Part2", day.part2, &buf);
    Ok(())
}

fn run_day(day: &Day) -> AnyResult<()> {
    run_day_with_input_path(day, &PathBuf::from(day.default_input))
}

fn main() -> anyhow::Result<()> {
    // TODO: Use clap to parse params to structured opts!
    let prog_args: Vec<String> = env::args().collect();
    let first_arg = prog_args.get(1);
    // Converts Option<String> to Option<&str> so I can match on `Some("all")`
    // (necessary because matching on `Some("all".to_string())` does not work
    // and matching on `Some(xyz) if xyz == "all"` is ugly..).
    match first_arg.and_then(|s| Some(s.as_str())) {
        Some("all") => {
            for day in DAYS {
                run_day(day)?;
            }
        }
        Some("last") => {
            run_day(&DAYS.last().unwrap())?;
        }
        Some(wanted_day) => {
            let matching_day = DAYS.iter().find(|day| day.name == wanted_day);
            match matching_day {
                Some(day) => {
                    let input_path = match prog_args.get(2) {
                        Some(input_path) => PathBuf::from(input_path),
                        None => PathBuf::from(day.default_input),
                    };
                    run_day_with_input_path(day, &input_path)?;
                }
                None => {
                    println!("Unknown day '{}'", wanted_day);
                    exit(1);
                }
            };
        }
        None => print_usage(),
    };
    Ok(())
}
