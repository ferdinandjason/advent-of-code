use std::collections::HashMap;

pub struct Input<'a> {
    pattern: Vec<&'a str>,
    designs: Vec<&'a str>,
}

pub fn parse(input: &str) -> Input {
    let (pattern, design) = input.split_once("\n\n").unwrap();

    Input {
        pattern: pattern.split(", ").collect(),
        designs: design.lines().collect(),
    }
}

pub fn solve(input: &Input) -> (u32, u64) {
    let mut state = HashMap::new();

    input
        .designs
        .iter()
        .map(|design| {
            let ways = find_arrangements(design, &input.pattern, &mut state, 0);
            (ways > 0, ways)
        })
        .fold((0, 0), |answer, value| {
            (answer.0 + value.0 as u32, answer.1 + value.1)
        })
}

fn find_arrangements<'a>(
    target: &'a str,
    pattern: &[&str],
    state: &mut HashMap<(&'a str, usize), u64>,
    idx: usize,
) -> u64 {
    if let Some(&ways) = state.get(&(target, idx)) {
        return ways;
    }

    if idx == target.len() {
        state.insert((target, idx), 1);
        return 1;
    }

    let mut possible_ways = 0;
    for i in 0..pattern.len() {
        if idx + pattern[i].len() <= target.len()
            && &target[idx..(idx + pattern[i].len())] == pattern[i]
        {
            possible_ways += find_arrangements(target, pattern, state, idx + pattern[i].len());
        }
    }

    state.insert((target, idx), possible_ways);
    possible_ways
}
