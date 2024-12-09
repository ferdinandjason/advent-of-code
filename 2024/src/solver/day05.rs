use std::{cmp::Ordering, collections::HashSet};

pub struct Input {
    rules: Vec<(i32, i32)>,
    query: Vec<Vec<i32>>,
}

pub fn parse(input: &str) -> Input {
    let (rules_raw, query_raw) = input.split_once("\n\n").unwrap();

    Input {
        rules: rules_raw
            .split('\n')
            .map(|s| {
                let (u, v) = s.split_once('|').unwrap();
                (u.parse().unwrap(), v.parse().unwrap())
            })
            .collect::<Vec<_>>(),
        query: query_raw
            .split('\n')
            .map(|s| s.split(',').map(|i| i.parse().unwrap()).collect())
            .collect(),
    }
}

pub fn solve(input: &Input) -> (i32, i32) {
    let mut order_map: HashSet<(i32, i32)> = HashSet::new();
    input.rules.iter().for_each(|(a, b)| {
        order_map.insert((*a, *b));
    });
    let mut part1 = Vec::with_capacity(input.query.len());
    let mut part2 = Vec::with_capacity(input.query.len());

    input.query.clone().iter_mut().for_each(|v| {
        if v.is_sorted_by(|a, b| order_map.contains(&(*a, *b))) {
            part1.push(v[v.len() / 2]);
        } else {
            v.sort_by(|a, b| {
                if a == b {
                    return Ordering::Equal;
                }
                if order_map.contains(&(*a, *b)) {
                    return Ordering::Less;
                }
                return Ordering::Greater
            });

            part2.push(v[v.len() / 2]);
        }
    });

    (part1.iter().sum(), part2.iter().sum())
}