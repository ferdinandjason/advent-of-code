use itertools::Itertools;

#[derive(Debug)]
pub struct Input {
    a: (i64, i64),
    b: (i64, i64),
    z: (i64, i64),
}

impl Input {
    fn token_to_win(&self, delta: i64) -> i64 {
        let num_b = (self.z.0 + delta) * self.a.1 - (self.z.1 + delta) * self.a.0;
        let denom_b = self.b.0 * self.a.1 - self.b.1 * self.a.0;

        if num_b % denom_b != 0 {
            return 0;
        }

        let b_token = num_b / denom_b;
        let num_a = (self.z.0 + delta) - self.b.0 * b_token;
        let denom_a = self.a.0;

        if num_a % denom_a != 0 {
            return 0;
        }

        let a_token = num_a / denom_a;

        a_token * 3 + b_token
    }
}

pub fn parse(input: &str) -> Vec<Input> {
    input
        .split("\n\n")
        .map(|batch| {
            let lines = batch.lines().collect::<Vec<_>>();
            let button_a = lines[0][12..]
                .split(", Y+")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let button_b = lines[1][12..]
                .split(", Y+")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let prize = lines[2][9..]
                .split(", Y=")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Input {
                a: button_a,
                b: button_b,
                z: prize,
            }
        })
        .collect()
}

pub fn solve(input: &[Input]) -> (i64, i64) {
    (
        input.iter().map(|i| i.token_to_win(0)).sum(),
        input.iter().map(|i| i.token_to_win(10000000000000)).sum(),
    )
}
