use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Input {
    a: u32,
    b: u32,
    c: u32,

    inst: Vec<u32>,
}

impl Input {
    fn new(a: u32, b: u32, c: u32, inst: Vec<u32>) -> Self {
        Self {a, b, c, inst}
    }
}

pub fn parse(input: &str) -> Input {
    let (reg, inst) = input.split_once("\n\n").unwrap();
    let regs = reg.lines().map(|line| line[12..].parse().unwrap()).collect::<Vec<u32>>();
    
    Input {
        a: regs[0],
        b: regs[1],
        c: regs[2],

        inst: inst[9..].split(',').map(|chunk| chunk.parse().unwrap()).collect()
    }
}

pub fn solve(input: &Input) -> (String, u32) {
    let part1 = input.clone();
    let mut a = 0;
    while compute(Input::new(a, 0, 0, input.inst.clone())) != input.inst {
        a += 1;
    }


    (compute(part1).iter().join(","), a)
}

fn compute(mut data: Input) -> Vec<u32> {
    let mut output = Vec::new();
    let mut ptr = 0;
    while ptr < data.inst.len() {
        let operand = data.inst[ptr + 1];
        match data.inst[ptr] {
            0 => {
                data.a >>= to_combo(operand, &data);
            },
            1 => {
                data.b ^= operand;
            },
            2 => {
                data.b = to_combo(operand, &data) % 8;
            },
            3 => {
                match data.a {
                    0 => (),
                    _ => {
                        ptr = operand as usize;
                        continue;
                    }
                }
            },
            4 => {
                data.b ^=  data.c
            },
            5 => {
                output.push(to_combo(operand, &data) % 8);
            },
            6 => {
                data.b = data.a >> to_combo(operand, &data);
            },
            7 => {
                data.c = data.a >> to_combo(operand, &data);
            },
            _ => unimplemented!()
        }

        ptr += 2;
    }

    output
}

fn to_combo(operand: u32, data: &Input) -> u32 {
    match operand {
        0..=3 => operand,
        4 => data.a,
        5 => data.b,
        6 => data.c,
        _ => unimplemented!()
    }
} 
