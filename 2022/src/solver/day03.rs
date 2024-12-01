pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve(rucksacks: &Vec<&str>) -> (u32, u32) {
    let part1 = rucksacks
        .iter()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            (mask(first) & mask(second)).trailing_zeros()
        })
        .sum::<u32>();

    let part2 = rucksacks
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            (mask(chunks[0]) & mask(chunks[1]) & mask(chunks[2])).trailing_zeros()
        })
        .sum();

    (part1, part2)
}

fn mask(s: &str) -> u64 {
    s.bytes().fold(0, |acc, b| {
        let sub = if b >= b'a' { b'a' - 1 } else { b'A' - 27 };
        acc | 1 << (b - sub)
    })
}

