use std::collections::HashMap;

pub struct Input {
    pub a: Vec<i32>,
    pub b: Vec<i32>,
    pub mpb: HashMap<i32, i32>,
}

pub fn parse(input: &str) -> Input {
    let mut i = Input {
        a: Vec::new(),
        b: Vec::new(),
        mpb: HashMap::new(),
    };

    input
        .lines()
        .map(|s| s.split_once("   ").unwrap())
        .for_each(|(a, b)| {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();

            i.a.push(a);
            i.b.push(b);

            *i.mpb.entry(b).or_default() += 1;
        });

    i.a.sort_unstable();
    i.b.sort_unstable();

    i
}

pub fn solve(input: &Input) -> (i32, i32) {
    return input
        .a
        .iter()
        .zip(input.b.iter())
        .fold((0, 0), |(part1, part2), (a, b)| {
            (
                part1 + i32::abs(a - b),
                part2 + input.mpb.get(a).copied().unwrap_or(0) * a,
            )
        });
}
