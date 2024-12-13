mod solver;

use std::{env, path::PathBuf, time::Instant};

fn main() {
    let day = match env::args().nth(1) {
        Some(day) => Some(day.parse::<u8>().unwrap()),
        None => None,
    };

    let mut elapsed = 0;
    let solver = aoc()
        .into_iter()
        .filter(|solution| day == Some(solution.day) || day.is_none())
        .collect::<Vec<_>>();

    for Solver { day, wrapper } in &solver {
        let path = ["input", &format!("day{day:02}.txt")]
            .iter()
            .collect::<PathBuf>();
        let input = std::fs::read_to_string(path).expect("Unable to load input file");
        let time = Instant::now();
        let (answer1, answer2) = wrapper(&input);
        let duration = time.elapsed().as_micros();
        elapsed += duration;

        println!("Day {day:02}");
        println!("    Part 1: {answer1}");
        println!("    Part 2: {answer2}");
        println!("    Duration: {duration} μs");
    }

    println!("Solutions: {}", solver.len());
    println!("Elapsed: {} ms", elapsed / 1000);
}

struct Solver {
    day: u8,
    wrapper: fn(&str) -> (String, String),
}

macro_rules! solve {
    ($day:tt) => {
        Solver {
            day: (&stringify!($day)[3..]).parse::<u8>().unwrap(),
            wrapper: |data: &str| {
                use solver::$day::*;

                let input = parse(&data);
                let (part1, part2) = solve(&input);

                (part1.to_string(), part2.to_string())
            },
        }
    };
}

fn aoc() -> Vec<Solver> {
    vec![
        solve!(day01),
        solve!(day02),
        solve!(day03),
        solve!(day04),
        solve!(day05),
        solve!(day06),
        solve!(day07),
        solve!(day08),
        solve!(day09),
        solve!(day10),
        solve!(day11),
        solve!(day12),
        solve!(day13),
    ]
}
