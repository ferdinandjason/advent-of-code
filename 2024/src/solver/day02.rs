pub fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .split("\n")
        .map(|s| {
            s.split(" ")
                .map(|i| i.parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn solve(input: &[Vec<i8>]) -> (usize, usize) {
    (part1(input), part2(input))
}

fn is_safe_inc(a: i8, b: i8) -> bool {
    let diff = a - b;
    diff >= 1 && diff <= 3
}

fn is_safe_dec(a: i8, b: i8) -> bool {
    let diff = a - b;
    diff <= -1 && diff >= -3
}

pub fn part1(input: &[Vec<i8>]) -> usize {
    input
        .iter()
        .map(|v| {
            v.windows(2)
                .all(|w| is_safe_inc(w[0], w[1]) || v.windows(2).all(|w| is_safe_dec(w[0], w[1])))
        })
        .filter(|&b| b)
        .count()
}

pub fn part2(input: &[Vec<i8>]) -> usize {
    input
        .iter()
        .map(|v| {
            if v.windows(2).all(|w| is_safe_inc(w[0], w[1]))
                || v.windows(2).all(|w| is_safe_dec(w[0], w[1]))
            {
                return true;
            }

            (0..v.len()).any(|i| {
                let mut tmp = v.clone();
                tmp.remove(i);
                tmp.windows(2).all(|w| is_safe_inc(w[0], w[1]))
                    || tmp.windows(2).all(|w| is_safe_dec(w[0], w[1]))
            })
        })
        .filter(|&b| b)
        .count()
}
