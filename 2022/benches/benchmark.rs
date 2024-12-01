#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($day:tt) => {
        mod $day {
            use aoc::solver::$day::*;
            use std::fs;
            use std::path::PathBuf;
            use std::sync::OnceLock;
            use test::Bencher;        
            

            fn load_once() -> &'static str {
                static DATA: OnceLock<String> = OnceLock::new();
                DATA.get_or_init(|| {
                    let day = &format!("{}.txt", stringify!($day));
                    let path: PathBuf = ["input", day].iter().collect();
                    dbg!(&path);
                    fs::read_to_string(path).unwrap()
                })
            }

            #[bench]
            fn parse_bench(b: &mut Bencher) {
                let data = load_once();
                b.iter(|| parse(&data));
            }

            #[bench]
            fn solve_bench(b: &mut Bencher) {
                let data = load_once();
                let input = parse(&data);
                b.iter(|| solve(&input));
            }
        }
    };
}

mod aoc2022 {
    benchmark!(day01);
    benchmark!(day02);
    benchmark!(day03);
    benchmark!(day04);
    benchmark!(day05);
    benchmark!(day06);
    benchmark!(day07);
    benchmark!(day08);
    benchmark!(day09);
    benchmark!(day10);
    benchmark!(day11);
    benchmark!(day12);
    benchmark!(day13);
    benchmark!(day14);
    benchmark!(day15);
    benchmark!(day16);
    benchmark!(day17);
    benchmark!(day18);
    benchmark!(day19);
    benchmark!(day20);
    benchmark!(day21);
    benchmark!(day22);
    benchmark!(day23);
    benchmark!(day24);
    benchmark!(day25);
}

