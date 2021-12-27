use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

use anyhow::Result as AnyResult;

// These lines DECLARE the modules of my app
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

// NOTE: We can't pass generic type that impl Read, so we pass a trait object, which will use
//       dynamic dispatch on use.
type PartFn = fn(&str) -> (usize, Option<usize>);

// We define a lifetime in 'Day', to be able to store references in the struct.
struct Day {
    name: &'static str,
    // description: &'static str,
    part1: PartFn,
    part2: PartFn,
    default_input: &'static str,
}

macro_rules! def_day {
    // ($d: ident, $desc: tt) => {
    ($d: ident) => {
        Day {
            name: stringify!($d),
            // description: stringify!($desc),
            part1: $d::solve_part1,
            part2: $d::solve_part2,
            default_input: concat!("./inputs/", stringify!($d), ".txt"),
        }
    };
}

static DAYS: &[Day] = &[
    // Day {
    //     name: "day01",
    //     part1: day01::solve_part1,
    //     part2: day01::solve_part2,
    //     default_input: "./inputs/day01.txt",
    // },
    // same as:
    def_day!(day01), // Sonar Sweep
    // ---
    def_day!(day02), // Dive!
    def_day!(day03), // Binary Diagnostic
    def_day!(day04), // Giant Squid (bingo simulation)
    def_day!(day05), // Hydrothermal Venture (crossing lines)
    def_day!(day06), // Lanternfish (recursive fish colony)
    def_day!(day07), // The Treachery of Whales (efficient crab movements)
    def_day!(day08), // Seven Segment Search
    def_day!(day09), // Smoke Basin (find low points & basins in a heightmap) FOR NOW...
];

fn print_usage() {
    let prog_name = env::args().next().unwrap_or("prog".to_string());
    let day_names: Vec<_> = DAYS.iter().map(|d| d.name).collect();
    println!("Usage:");
    println!("  {} <cmd>", prog_name);
    println!("  {} <day> [<custom_input_path>]", prog_name);
    println!("");

    println!("<cmd> can be:");
    println!("  all   - run all available days");
    println!("  last  - run the last available day (used while dev)");
    println!("  list  - list available days");
    println!("");

    let joined_days = day_names.join(", ");
    println!("<day> can be one of: {}", joined_days);
    println!("");
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

    // NOTE: We convert Option<String> to Option<&str> to be able to match on `Some("all")`,
    //       instead of matching `Some(xyz) if xyz == "all"` many times, which is quite ugly.
    match first_arg.map(String::as_str) {
        Some("all") => {
            for day in DAYS {
                run_day(day)?;
            }
        }
        Some("last") => {
            run_day(&DAYS.last().unwrap())?;
        }
        Some("list") => {
            println!("Available days:");
            for day in DAYS {
                println!("- {}", day.name);
                // TODO: Display how finish that day it, by running it without displaying anything
                //       (need to return proper enum with all statuses)
            }
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
