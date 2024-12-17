use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Input {
    a: u64,
    b: u64,
    c: u64,

    inst: Vec<u64>,
}

pub fn parse(input: &str) -> Input {
    let (reg, inst) = input.split_once("\n\n").unwrap();
    let regs = reg
        .lines()
        .map(|line| line[12..].parse().unwrap())
        .collect::<Vec<u64>>();

    Input {
        a: regs[0],
        b: regs[1],
        c: regs[2],

        inst: inst[9..]
            .split(',')
            .map(|chunk| chunk.parse().unwrap())
            .collect(),
    }
}

pub fn solve(input: &Input) -> (String, u64) {
    (
        compute(input.a, input.b, input.c, &input.inst)
            .iter()
            .join(","),
        find_quine(&input.inst, input.inst.len(), 0),
    )
}

fn find_quine(inst: &Vec<u64>, idx: usize, value: u64) -> u64 {
    if idx == 0 {
        return if compute(value, 0, 0, inst) == *inst {
            value
        } else {
            0
        };
    }

    for a in 0..8 {
        let new_value = value * 8 + a;
        if compute(new_value, 0, 0, inst)[0] == inst[idx - 1] {
            if let result @ 1.. = find_quine(inst, idx - 1, new_value) {
                return result;
            }
        }
    }

    0
}

fn compute(mut a: u64, mut b: u64, mut c: u64, inst: &Vec<u64>) -> Vec<u64> {
    let mut output = Vec::new();
    let mut ptr = 0;
    while ptr < inst.len() {
        let operand = inst[ptr + 1];
        match inst[ptr] {
            0 => a >>= to_combo(operand, a, b, c),
            1 => b ^= operand,
            2 => b = to_combo(operand, a, b, c) % 8,
            3 => {
                if a != 0 {
                    ptr = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(to_combo(operand, a, b, c) % 8),
            6 => b = a >> to_combo(operand, a, b, c),
            7 => c = a >> to_combo(operand, a, b, c),
            _ => unimplemented!(),
        }

        ptr += 2;
    }

    output
}

fn to_combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => unimplemented!(),
    }
}
