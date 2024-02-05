use std::str;
use itertools::Itertools;

pub struct Instruction {
    qty: usize,
    src: usize,
    dst: usize,
}

pub struct Input {
    stack: Vec<Vec<char>>,
    insts: Vec<Instruction>,
}

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let stack_map = prefix.lines().collect::<Vec<_>>();
    let width = (stack_map[0].len() + 1) / 4;

    let mut stack: Vec<Vec<char>> = vec![Vec::new(); width];
    for row in stack_map.iter().rev().skip(1) {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if !c.is_ascii_whitespace() {
                stack[i].push(c);
            }
        }
    }

    let insts = suffix
        .lines()
        .map(|line| {
            let (qty, src, dst) = line
                .as_bytes()
                .split(|c| c.is_ascii_whitespace() || c.is_ascii_alphabetic())
                .filter(|c| !c.is_empty())
                .map(|c| str::from_utf8(c).unwrap().parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _)>().unwrap();

            Instruction {qty, src, dst}
        })
        .collect::<Vec<_>>();

    Input{stack, insts}
}

pub fn solve(input: &Input) -> (String, String) {
    let part1 = simulate(input, false);
    let part2 = simulate(input, true);

    (part1, part2)
}

fn simulate(input: &Input, rev: bool) -> String {
    let Input {stack, insts} = input;
    let mut stack = stack.clone();
    let mut temp = Vec::new();

    for &Instruction {qty, src, dst} in insts {
        let start = stack[src - 1].len() - qty;
        temp.extend(stack[src - 1].drain(start..));
        
        if rev { stack[dst - 1].extend(temp.iter().rev()); } 
        else   { stack[dst - 1].extend(temp.iter()); }

        temp.clear();
    }

    stack.iter().map(|v| v.last().unwrap()).collect()
}
