use fxhash::FxHashMap;
use itertools::Itertools;
use std::iter::zip;

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
pub enum Op {
    And,
    Or,
    Xor,
}

pub struct Input {
    init: FxHashMap<String, usize>,
    gates: FxHashMap<String, (String, Op, String)>,
    revgates: FxHashMap<(String, Op, String), String>,
}

pub fn parse(input: &str) -> Input {
    let (init, gates) = input.split_once("\n\n").unwrap();
    let gates: FxHashMap<_, _> = gates
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let (a, rop, b) = lhs
                .split_whitespace()
                .collect_tuple::<(&str, &str, &str)>()
                .unwrap();

            let op = match rop {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => unimplemented!(),
            };

            (rhs.to_string(), (a.to_string(), op, b.to_string()))
        })
        .collect();

    let revgates: FxHashMap<_, _> = gates
        .iter()
        .map(|(&ref key, &ref value)| {
            (
                (value.0.clone(), value.1.clone(), value.2.clone()),
                key.clone(),
            )
        })
        .collect();

    Input {
        init: init
            .lines()
            .map(|line| {
                let (value, key) = line.split_once(": ").unwrap();
                (value.to_string(), key.parse().unwrap())
            })
            .collect::<FxHashMap<_, _>>(),
        gates,
        revgates,
    }
}

pub fn solve(input: &Input) -> (u64, String) {
    (simulate(input), find_error(input))
}

fn simulate(input: &Input) -> u64 {
    let mut values = input.init.clone();
    input
        .gates
        .keys()
        .filter(|x| x.starts_with('z'))
        .sorted()
        .rev()
        .map(|x| resolve(&mut values, input, x.clone()))
        .fold(0u64, |bin, b| (bin << 1) + b as u64)
}

fn resolve(values: &mut FxHashMap<String, usize>, input: &Input, target: String) -> usize {
    if let Some(value) = values.get(&target) {
        return *value;
    }

    let (a, op, b) = input.gates.get(&target).unwrap();
    let a = match values.get(a) {
        Some(value) => *value,
        None => resolve(values, input, a.clone()),
    };
    let b = match values.get(b) {
        Some(value) => *value,
        None => resolve(values, input, b.clone()),
    };

    let value = match op {
        Op::And => a & b,
        Op::Or => a | b,
        Op::Xor => a ^ b,
    };

    values.insert(target, value);
    value
}

fn find_error(input: &Input) -> String {
    let x = input.init.keys().filter(|x| x.starts_with('x')).sorted();
    let y = input.init.keys().filter(|x| x.starts_with('y')).sorted();
    let mut c0 = None;
    let mut errors = Vec::with_capacity(8);

    for (x, y) in zip(x, y) {
        let (mut z1, c1) = full_adder(
            x.to_string(),
            y.to_string(),
            c0.clone(),
            &input.revgates,
            &mut errors,
        );

        if let Some(mut c) = c1 {
            if c.starts_with('z') && c != "z45" {
                // if the carry is output and not the last one
                std::mem::swap(&mut c, &mut z1);
                errors.push(c.clone());
                errors.push(z1.clone());
            }
            c0 = Some(c);
        } else {
            c0 = find_output(x.clone(), y.clone(), Op::And, &input.revgates);
        }
    }

    errors.iter().sorted().join(",")
}

fn full_adder(
    x: String,
    y: String,
    c0: Option<String>,
    reverse_lookup: &FxHashMap<(String, Op, String), String>,
    errors: &mut Vec<String>,
) -> (String, Option<String>) {
    // x1 xor y1 = m1 -- intermediate sum
    // x1 and y1 = n1 -- intermediate carry
    // c0 and m1 = r1 -- carry for intermediate sum
    // c0 xor m1 = z1 -- final sum
    // r1  or n1 = c1 -- final cary

    let mut m1 = find_output(x.clone(), y.clone(), Op::Xor, reverse_lookup).unwrap(); // intermediate sum
    let mut n1 = find_output(x, y, Op::And, reverse_lookup).unwrap(); // intermediate carry

    if let Some(c) = c0 {
        let mut r1 = find_output(c.clone(), m1.clone(), Op::And, reverse_lookup);
        if r1 == None {
            std::mem::swap(&mut m1, &mut n1); // then m1 and n1 must be swapped;
            errors.push(m1.clone());
            errors.push(n1.clone());
            r1 = find_output(c.clone(), m1.clone(), Op::And, reverse_lookup);
        }

        let mut z1 = find_output(c, m1.clone(), Op::Xor, reverse_lookup).unwrap();

        if m1.starts_with('z') {
            // if m1 is output layer
            std::mem::swap(&mut m1, &mut z1);
            errors.push(m1.clone());
            errors.push(z1.clone());
        }

        if n1.starts_with('z') {
            // if n1 is output layer
            std::mem::swap(&mut n1, &mut z1);
            errors.push(n1.clone());
            errors.push(z1.clone());
        }

        let mut r = r1.unwrap();

        if r.starts_with('z') {
            // if r1 is output layer
            std::mem::swap(&mut r, &mut z1);
            errors.push(r.clone());
            errors.push(z1.clone());
        }

        (z1, find_output(r, n1, Op::Or, reverse_lookup))
    } else {
        (m1, Some(n1))
    }
}

fn find_output(
    x: String,
    y: String,
    op: Op,
    reverse_lookup: &FxHashMap<(String, Op, String), String>,
) -> Option<String> {
    if let Some(&ref value) = reverse_lookup.get(&(x.clone(), op.clone(), y.clone())) {
        return Some(value.to_string());
    }

    if let Some(&ref value) = reverse_lookup.get(&(y.clone(), op.clone(), x.clone())) {
        return Some(value.to_string());
    }

    None
}
