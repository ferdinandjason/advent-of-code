use regex::Regex;

pub fn parse(input: &str) -> &str {
    input
}

pub fn solve(input: &str) -> (u64, u64) {
    (part1(input), part2(input))
}

pub fn part1(input: &str) -> u64 {
    let re_mul: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re_mul
        .captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let re_do_mul = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enable = true;
    re_do_mul
        .captures_iter(input)
        .map(|caps| {
            match caps.get(0).unwrap().as_str() {
                "do()" => {
                    enable = true;
                    0
                }
                "don't()" => {
                    enable = false;
                    0
                }
                _ if enable => {
                    if let (Some(a), Some(b)) = (caps.get(1), caps.get(2)) {
                        a.as_str().parse::<u64>().unwrap()
                            * b.as_str().parse::<u64>().unwrap()
                    } else {
                        0
                    }
                },
                _ => 0,
            }
        })
        .sum::<u64>()
}
