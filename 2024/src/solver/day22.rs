use fxhash::{FxHashMap, FxHashSet};

use itertools::Itertools;

pub fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve(input: &[i64]) -> (i64, i64) {
    (buyers_2000th_secrets(input), max_bananas(input))
}

fn buyers_2000th_secrets(input: &[i64]) -> i64 {
    input
        .iter()
        .map(|secret| Secret::new(*secret).iter().nth(1999).unwrap())
        .sum()
}

fn max_bananas(input: &[i64]) -> i64 {
    let mut seen = FxHashSet::default();
    let mut memo = FxHashMap::default();

    for secret in input.iter() {
        for (mut a, mut b, mut c, mut d, mut e) in
            Secret::new(*secret).iter().take(2000).tuple_windows()
        {
            a %= 10;
            b %= 10;
            c %= 10;
            d %= 10;
            e %= 10;
            if seen.insert((b - a, c - b, d - c, e - d)) {
                memo.entry((b - a, c - b, d - c, e - d))
                    .and_modify(|sum| *sum += e)
                    .or_insert(e);
            }
        }
        seen.clear();
    }

    *memo.values().max().unwrap()
}

struct Secret {
    secret: i64,
}

impl Secret {
    fn new(secret: i64) -> Self {
        Self { secret }
    }

    fn iter(self) -> SecretIterator {
        SecretIterator {
            s: self.secret,
            m: 16777216,
        }
    }
}

struct SecretIterator {
    s: i64,
    m: i64,
}

impl Iterator for SecretIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = (self.s ^ (self.s * 64)) % self.m;
        self.s = (self.s ^ (self.s / 32)) % self.m;
        self.s = (self.s ^ (self.s * 2048)) % self.m;

        Some(self.s)
    }
}
