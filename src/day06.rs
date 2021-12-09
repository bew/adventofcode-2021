// day 06

use std::collections::HashMap;

use chumsky as c;
use chumsky::prelude::*;

type BirthStage = u8;

#[derive(Debug)]
struct Lanternfish {
    pub days_before_childbirth: BirthStage,
}
impl std::fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.days_before_childbirth)
    }
}

// Format:
//   1,2,3,4 ...
fn input_parser() -> impl Parser<char, Vec<Lanternfish>, Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let lanternfish = number.map(|n| Lanternfish {
        days_before_childbirth: n,
    });
    lanternfish.separated_by(just(','))
}

pub fn solve_part1(raw_input: &str) -> (usize, Option<usize>) {
    let mut fishes = input_parser().parse(raw_input).unwrap();
    let run_for_days = 80;

    // impl bete & mechante... (but slow and using lots of memory..)
    #[allow(unused_variables)]
    for day in 1..=run_for_days {
        // Run simulation for that day
        let mut nb_fish_birth = 0;
        fishes.iter_mut().for_each(|fish| {
            if fish.days_before_childbirth == 0 {
                nb_fish_birth += 1;
                fish.days_before_childbirth = 6;
            } else {
                fish.days_before_childbirth -= 1;
            }
        });
        // Let new fishes be born!
        for _ in 0..nb_fish_birth {
            fishes.push(Lanternfish {
                days_before_childbirth: 8,
            });
        }
        // Print
        // println!("After {:2} days, nb fishes: {}", day, fishes.len());
    }

    (fishes.len(), Some(391671))
}

#[derive(Debug)]
struct LanternfishBirthSimulator {
    fish_count_by_birth_stage: HashMap<BirthStage, usize>,
}
impl LanternfishBirthSimulator {
    pub fn from_fishes(input_fishes: &[Lanternfish]) -> Self {
        let mut simu = Self::new();
        for fish in input_fishes {
            simu.add_fish_at_stage(fish.days_before_childbirth);
        }
        simu
    }

    pub fn new() -> Self {
        Self {
            fish_count_by_birth_stage: HashMap::new(),
        }
    }

    fn add_fish_at_stage(&mut self, stage: BirthStage) {
        self.add_many_fishes_at_stage(stage, 1);
    }
    fn add_many_fishes_at_stage(&mut self, stage: BirthStage, fishes_to_add: usize) {
        let fish_count = self.fish_count_by_birth_stage.entry(stage).or_insert(0);
        *fish_count += fishes_to_add;
    }

    pub fn fish_count(&self) -> usize {
        self.fish_count_by_birth_stage.values().sum()
    }

    pub fn simulate_passing_day(&mut self) {
        // new day, reset counters
        let last_state = std::mem::replace(&mut self.fish_count_by_birth_stage, HashMap::new());

        // For all fish waiting to give birth, let them pass the day
        for birth_stage in 1..=8 {
            if let Some(fish_count) = last_state.get(&birth_stage) {
                self.add_many_fishes_at_stage(birth_stage - 1, *fish_count);
            }
        }
        // For all fish ready to give birth, do it!
        if let Some(fish_count) = last_state.get(&0) {
            self.add_many_fishes_at_stage(8, *fish_count); // new fish!
            self.add_many_fishes_at_stage(6, *fish_count); // old fish, they know how to do it, 7 days is enough!
        }
    }
}

#[allow(unused_variables)]
pub fn solve_part2(raw_input: &str) -> (usize, Option<usize>) {
    let input_fishes = input_parser().parse(raw_input).unwrap();
    // NOTE: The dummy impl doesn't work for 256 days, because even around 190 days, there are
    // 7_172_256_393 fishes already, which means 7 Gigabytes of RAM used to store fishes!!!
    // (given 1 fish stored in a byte)
    let run_for_days = 256;
    // We need another data structure for that fish simulation..

    // IDEA: We don't actually care about the order of fishes, or even about the fishes at all.
    // I could simply store the number of fishes at each day-before-birth stage, and simulate
    // progress with it!
    // ==> That's what I implemented in LanternfishBirthSimulator, works flawlessly :)
    let mut simu = LanternfishBirthSimulator::from_fishes(&input_fishes);

    for day in 1..=run_for_days {
        simu.simulate_passing_day();
        // println!("After {:2} days, nb fishes: {}", day, simu.fish_count());
    }

    (simu.fish_count(), Some(1_754_000_560_399)) // that's a lot...
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_example_part1() {
        let (result, _) = solve_part1(EXAMPLE_INPUT);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_example_part2() {
        let (result, _) = solve_part2(EXAMPLE_INPUT);
        assert_eq!(result, 26_984_457_539);
    }
}
