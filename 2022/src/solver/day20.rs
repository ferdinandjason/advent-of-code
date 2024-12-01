use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse::<i64>().unwrap()).collect()
}

pub fn solve(encrypted: &[i64]) -> (i64, i64) {
    (
        decrypt(encrypted, 1, 1),
        decrypt(encrypted, 811_589_153, 10)
    )
}

fn decrypt(encrypted: &[i64], key: i64, round: i64) -> i64 {
    let mut input = encrypted.iter().map(|v| v * key).enumerate().collect::<VecDeque<_>>();

    for _ in 0..round {
        for i in 0..input.len() {
            let idx = input.iter().position(|(j, _)| i == *j).unwrap();
            
            input.rotate_left(idx);
            let (j, v) = input.pop_front().unwrap();
            let d = v.rem_euclid(input.len() as i64);
            input.rotate_left(d as usize);
            input.push_front((j, v));
        }
    }

    let idx = input.iter().position(|(_, v)| *v == 0_i64).unwrap();

    [1000, 2000, 3000].iter().map(|i| input[(idx + i) % input.len()].1).sum()
}
