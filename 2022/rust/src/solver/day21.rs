use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

impl Operator {
    fn new(op: u8) -> Operator {
        match op {
            b'+' => Operator::Add,
            b'-' => Operator::Sub,
            b'*' => Operator::Mul,
            b'/' => Operator::Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Monkey {
    Number(i64),
    Calculate(usize, Operator, usize),
}

struct State {
    root: usize,
    humn: usize,
    monkeys: Vec<Monkey>,
    result: Vec<i64>,
    unknown: Vec<bool>
}

pub fn parse(input: &str) -> (usize, usize, usize, Vec<Monkey>) {
    let input = input.lines().map(|line| line.split_once(": ").unwrap()).collect::<Vec<_>>();
    let indices = input.iter().enumerate().map(|(index, monkey)| (monkey.0, index)).collect::<HashMap<_, _>>();
    let monkeys = input.iter().map(|(_, job)| {
        if job.len() < 5 {
            Monkey::Number(job.parse::<i64>().unwrap())
        } else {
            Monkey::Calculate(
                indices[&job[0..4]],
                Operator::new(job.as_bytes()[5]),
                indices[&job[7..11]],
            )
        }
    }).collect::<Vec<_>>();
    
    let root = indices["root"];
    let humn = indices["humn"];

    (root, humn, input.len(), monkeys)
}

pub fn solve(state: &(usize, usize, usize, Vec<Monkey>)) -> (i64, i64) {
    let mut state = State {
        root: state.0,
        humn: state.1,
        monkeys: state.3.clone(),
        result: vec![0; state.2],
        unknown: vec![false; state.2],
    };

    let root = state.root;
    calculate_result(&mut state, root);

    (state.result[state.root], find_humn(&mut state))
}

fn calculate_result(state: &mut State, i: usize) -> i64 {
    let res = match state.monkeys[i] {
        Monkey::Number(n) => n,
        Monkey::Calculate(a, op, b) => match op {
            Operator::Add => calculate_result(state, a) + calculate_result(state, b),
            Operator::Sub => calculate_result(state, a) - calculate_result(state, b),
            Operator::Mul => calculate_result(state, a) * calculate_result(state, b),
            Operator::Div => calculate_result(state, a) / calculate_result(state, b),
        },
    };

    state.result[i] = res;

    res
}

fn calculate_unknown(state: &mut State, i: usize, value: i64) -> i64 {
    match state.monkeys[i] {
        Monkey::Number(_) => value,
        Monkey::Calculate(a, _, b) if i == state.root => {
            if state.unknown[a] {
                calculate_unknown(state, a, state.result[b])
            } else {
                calculate_unknown(state, b, state.result[a])
            }
        },
        Monkey::Calculate(a, op, b) => {
            if state.unknown[a] {
                match op {
                    Operator::Add => calculate_unknown(state, a, value - state.result[b]),
                    Operator::Sub => calculate_unknown(state, a, value + state.result[b]),
                    Operator::Mul => calculate_unknown(state, a, value / state.result[b]),
                    Operator::Div => calculate_unknown(state, a, value * state.result[b]),
                }
            } else {
                match op {
                    Operator::Add => calculate_unknown(state, b, value - state.result[a]),
                    Operator::Sub => calculate_unknown(state, b, state.result[a] - value),
                    Operator::Mul => calculate_unknown(state, b, value / state.result[a]),
                    Operator::Div => calculate_unknown(state, b, state.result[a] / value),
                }
            }
        }
    }
}

fn find_unknown(state: &mut State, i: usize, humn: usize) -> bool {
    let unknown = match state.monkeys[i] {
        Monkey::Number(_) => i == humn,
        Monkey::Calculate(a, _, b) => find_unknown(state, a, humn) || find_unknown(state, b, humn)
    };
    
    state.unknown[i] = unknown;

    unknown
}

fn find_humn(state: &mut State) -> i64 {
    find_unknown(state, state.root, state.humn);
    calculate_unknown(state, state.root, 0)
}

