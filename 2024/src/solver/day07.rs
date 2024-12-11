pub fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();

            (
                target.parse().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn solve(input: &[(i64, Vec<i64>)]) -> (i64, i64) {
    (
        input
            .iter()
            .map(|(target, eqs)| {
                let n = eqs.len() - 1;
                for x in 0..(1 << n) {
                    let mut number = eqs[0];
                    for i in 0..n {
                        if (x >> i) & 1 == 0 {
                            number += eqs[i + 1];
                        } else {
                            number *= eqs[i + 1];
                        }
                    }

                    if *target == number {
                        return number;
                    }
                }

                0
            })
            .sum::<i64>(),
        input
            .iter()
            .map(|(target, eqs)| {
                let n = eqs.len() - 1;
                for mut x in 0..POW3[n] {
                    let mut number = eqs[0];
                    for i in 0..n {
                        match x % 3 {
                            0 => number += eqs[i + 1],
                            1 => number *= eqs[i + 1],
                            2 => number = concat(number, eqs[i + 1]),
                            _ => (),
                        }

                        x /= 3;
                    }

                    if *target == number {
                        return number;
                    }
                }

                0
            })
            .sum::<i64>(),
    )
}

const POW3: [i64; 12] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147];

fn concat(a: i64, b: i64) -> i64 {
    let mut temp = b;
    let mut pow10 = 1;
    while temp > 0 {
        pow10 *= 10;
        temp /= 10;
    }
    a * pow10 + b
}
