pub fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .split('\n')
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();
            (
                target.parse().unwrap(),
                numbers.split(' ').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

pub fn solve(input: &[(i64, Vec<i64>)]) -> (i64, i64) {
    let (mut part1, mut part2) = (0, 0);

    input.into_iter().for_each(|(target, eqs)| {
        let n = eqs.len() - 1;
        for mut x in 0..POW3[n] {
            let mut number = eqs[0];
            let mut using_concat = false;
            for i in 0..n {
                match x.rem_euclid(3) {
                    0 => number += eqs[i + 1],
                    1 => number *= eqs[i + 1],
                    2 => {
                        number = {
                            using_concat = true;
                            concat(number, eqs[i + 1])
                        }
                    }
                    _ => (),
                }

                x /= 3;
            }

            if *target == number {
                part2 += number;
                if !using_concat {
                    part1 += number;
                }

                break;
            }
        }
    });

    (part1, part2)
}

const POW3: [i64; 12] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147];
const POW10: [i64; 11] = [
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

fn concat(a: i64, b: i64) -> i64 {
    a * POW10[((b as f64).log10()).ceil() as usize] + b
}
