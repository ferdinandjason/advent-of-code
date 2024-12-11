// use std::collections::HashMap;
use fxhash::FxHashMap;

pub fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn solve(input: &[u64]) -> (usize, usize) {
    let mut state = FxHashMap::default();
    // for i in input.iter() {
    //     state.entry(*i).and_modify(|e| *e += 1).or_insert(1usize);
    // }
    //
    // let mut part1 = 0;
    // for i in 0..75 {
    //     state = blink(&state);
    //     if i == 24 {
    //         part1 = state.values().sum();
    //     }
    // }
    //
    // let part2 = state.values().sum();
    //
    // return (part1, part2);

    (
        input
            .iter()
            .map(|x| magic_stones(&25, 0, *x, &mut state).0)
            .sum(),
        input
            .iter()
            .map(|x| magic_stones(&75, 0, *x, &mut state).0)
            .sum(),
    )
}

fn magic_stones(
    target: &usize,
    level: usize,
    stone: u64,
    state: &mut FxHashMap<u64, Vec<usize>>,
) -> (usize, Vec<usize>) {
    if let Some(vec) = state.get(&stone) {
        let level_diff = target - level;
        if level_diff < vec.len() {
            return (vec[level_diff], vec.to_vec());
        }
    }

    let mut scores = 0;
    let mut chain = Vec::with_capacity(target + 1);
    chain.push(1);

    if level == *target {
        state.insert(stone, chain.clone());
        return (1, chain);
    }

    match stone {
        0 => {
            let (score, vec) = magic_stones(target, level + 1, 1, state);
            scores += score;
            chain.extend(vec.iter());
        }
        _ => {
            let nd = count_digits(stone);
            if nd % 2 == 1 {
                let (score, vec) = magic_stones(target, level + 1, stone * 2024, state);
                scores += score;
                chain.extend(vec.iter());
            } else {
                let n = POW10[nd / 2];

                let (score1, vec1) = magic_stones(target, level + 1, stone / n, state);
                let (score2, vec2) = magic_stones(target, level + 1, stone % n, state);

                for i in 0..vec1.len().min(vec2.len()) {
                    chain.push(vec1[i] + vec2[i]);
                }

                scores += score1;
                scores += score2;
            }
        }
    };

    if let Some(vec) = state.get(&stone) {
        if vec.len() < chain.len() {
            state.insert(stone, chain.clone());
        }
    } else {
        state.insert(stone, chain.clone());
    }

    (scores, chain)
}

fn count_digits(mut n: u64) -> usize {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

const POW10: [u64; 11] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
];

// fn blink(stones: &FxHashMap<u64, usize>) -> FxHashMap<u64, usize> {
//     let mut new_stones = FxHashMap::default();
//     for (&stone, &count) in stones {
//         let nd = count_digits(stone);
//         if stone == 0 {
//             new_stones
//                 .entry(1)
//                 .and_modify(|e| *e += count)
//                 .or_insert(count);
//         } else if nd % 2 == 0 {
//             let n = POW10[nd / 2];
//
//             new_stones
//                 .entry(stone / n)
//                 .and_modify(|e| *e += count)
//                 .or_insert(count);
//             new_stones
//                 .entry(stone % n)
//                 .and_modify(|e| *e += count)
//                 .or_insert(count);
//         } else {
//             new_stones
//                 .entry(stone * 2024)
//                 .and_modify(|e| *e += count)
//                 .or_insert(count);
//         }
//     }
//
//     new_stones
// }
