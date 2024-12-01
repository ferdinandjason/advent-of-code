use std::{cmp::Ordering, collections::HashMap};

struct Valve {
    code: u32,
    flow: u32,
    edges: Vec<u32>
}

impl Valve {
    fn new(input: &str) -> Self {
        let (info, to) = input.split_once("; ").unwrap();

        Self {
            code: perfect_hash(&info[6..8]),
            flow: info[23..].parse::<u32>().unwrap(),
            edges: to[22..].trim().split(", ").map(perfect_hash).collect::<Vec<_>>(),
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        other.flow.cmp(&self.flow).then(self.code.cmp(&other.code))
    }
}

fn perfect_hash(code: &str) -> u32 {
    code.bytes().fold(0, |acc, n| { acc * 26 + ((n - b'A') as u32)})
}

pub struct Input {
    start: usize,
    valves: Vec<Valve>,
    distances: Vec<Vec<u32>>,
    all_valve_open_mask: u32,
}

pub fn parse(input: &str) -> Input {
    let mut valves = input.lines().map(Valve::new).collect::<Vec<_>>(); 
    valves.sort_by(Valve::cmp);

    let n = valves.iter().position(| v| v.code == 0).unwrap() + 1;
    let indices = valves.iter().enumerate().map(|(i, v)| (v.code, i)).collect::<HashMap<u32, usize>>();

    let mut distances = vec![vec![u32::MAX; n]; n];
    for i in 0..n {
        distances[i][i] = 0;

        valves[i].edges.iter().for_each(|e| {
            let mut prev = valves[i].code;
            let mut curr = &valves[indices[e]];
            let mut distance = 1;
            while curr.flow == 0 && curr.code != 0 {
                let next = curr.edges.iter().find(|&&n| n != prev).unwrap();
                prev = curr.code;
                curr = &valves[indices[next]];
                distance += 1
            }

            distances[i][indices[&curr.code]] = distance;
        });
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = distances[i][j].min(distances[i][k].saturating_add(distances[k][j]));
            }
        }
    }

    distances.iter_mut().for_each(|v| v.iter_mut().for_each(|d| *d += 1));

    let start = indices[&0];
    let all_valve_open_mask = (1 << (n - 1)) - 1;
    Input {valves, distances, start, all_valve_open_mask}
}

pub fn solve(input: &Input) -> (u32, u32) {
    (
        part1(input),
        part2(input)
    )
}

fn part1(input: &Input) -> u32 {
    find_max_pressure(input, input.start, 30, 0, 0)
}

fn part2(input: &Input) -> u32 {
    let mut mask_memo = HashMap::default();

    find_max_pressure_with_memo(input, input.start, 26, 0, 0, &mut mask_memo);
    
    let mut max_pressure_with_elephant = 0;
    let mut mask_candidate = mask_memo.iter().filter(|(_, &max_pressure)| max_pressure > 0).collect::<Vec<_>>();
    mask_candidate.sort_by_key(|c| c.1);

    for i in 0..mask_candidate.len() {
        let (mask, max_pressure) = mask_candidate[i];
        for j in (0..i).rev() {
            let (elephant_mask, elephant_max_pressure) = mask_candidate[j];
            if mask & elephant_mask == 0 {
                max_pressure_with_elephant = max_pressure_with_elephant.max(max_pressure + elephant_max_pressure);
                break;
            }
        }
    }

    max_pressure_with_elephant
}

fn find_max_pressure(input: &Input, from: usize, time: u32, opened_valved_mask: u32, current_flow: u32) -> u32 {
    let mut state_mask = opened_valved_mask;
    let mut max_pressure = current_flow;

    while state_mask != input.all_valve_open_mask {
        let to = state_mask.trailing_ones() as usize;
        state_mask |= 1 << to;

        let needed = input.distances[from][to];
        if needed > time || from == to {
            continue;
        }

        let time_left = time - needed;
        max_pressure = max_pressure.max(
            find_max_pressure(input, to, time_left, opened_valved_mask | (1 << to), current_flow + input.valves[to].flow * time_left)
        )
    }

    max_pressure
}

fn find_max_pressure_with_memo(input: &Input, from: usize, time: u32, opened_valved_mask: u32, current_flow: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    memo.insert(
        opened_valved_mask,
        *memo.get(&opened_valved_mask).unwrap_or(&0).max(&current_flow)
    );

    let mut state_mask = opened_valved_mask;
    let mut max_pressure = current_flow;

    while state_mask != input.all_valve_open_mask {
        let to = state_mask.trailing_ones() as usize;
        state_mask |= 1 << to;

        let needed = input.distances[from][to];
        if needed > time || from == to {
            continue;
        }

        let time_left = time - needed;
        max_pressure = max_pressure.max(
            find_max_pressure_with_memo(input, to, time_left, opened_valved_mask | (1 << to), current_flow + input.valves[to].flow * time_left, memo)
        )
    }

    max_pressure
}
