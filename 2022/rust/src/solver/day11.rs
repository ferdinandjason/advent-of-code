use std::collections::VecDeque;

pub enum Operation {
    Sqr,
    Add(u64),
    Mul(u64),
}

pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

pub fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|monkey| {
        let row = monkey.split("\n").collect::<Vec<_>>();

        let items = row[1][18..].split(", ").map(|num| num.parse::<u64>().unwrap()).collect();
        let operation = match row[2][19..].split_ascii_whitespace().take(3).collect::<Vec<_>>()[..] {
            ["old", "*", "old"] => Operation::Sqr,
            [_, "+", value] => Operation::Add(value.parse::<u64>().unwrap()),
            [_, "*", value] => Operation::Mul(value.parse::<u64>().unwrap()),
            _ => unreachable!(),
        };
        let test = row[3][21..].parse::<u64>().unwrap();
        let yes = row[4][29..].parse::<usize>().unwrap();
        let no = row[5][30..].parse::<usize>().unwrap();

        Monkey{items, operation, test, yes, no}
    }).collect::<Vec<_>>()
}

pub fn solve(monkeys: &Vec<Monkey>) -> (u64, u64) {
    (
        part1(monkeys),
        part2(monkeys),
    )
}

fn part1(monkeys: &Vec<Monkey>) -> u64 {
    play(monkeys, 20, |worry| worry / 3)
}

fn part2(monkeys: &Vec<Monkey>) -> u64 {
    let modulo = monkeys.iter().map(|monkey| monkey.test).product::<u64>();
    play(monkeys, 10_000, |worry| worry % modulo)
}

fn play(monkeys: &Vec<Monkey>, rounds: u32, adjust: impl Fn(u64) -> u64) -> u64 {
    let mut counter = [0; 8];
    let mut q = VecDeque::<(usize, u64)>::new();
    monkeys.iter().enumerate().for_each(|(i, monkey)| {
        monkey.items.iter().for_each(|item| q.push_back((i, *item)));
    });

    for _ in 0..rounds {
        let mut nq = VecDeque::<(usize, u64)>::new();

        while let Some((from, worry)) = q.pop_front() {
            let worry = match monkeys[from].operation {
                Operation::Sqr => worry * worry,
                Operation::Add(val) => worry + val,
                Operation::Mul(val) => worry * val,
            };

            let worry = adjust(worry);
            let to = if worry % monkeys[from].test == 0 { monkeys[from].yes } else { monkeys[from].no };

            if to >= from {
                q.push_back((to, worry));
            } else {
                nq.push_back((to, worry));
            }
            counter[from] += 1;
        }

        q = nq;
    }

    counter.sort_unstable();
    counter.iter().rev().take(2).product()
}
