pub struct Input {
    pub interval: Vec<[u64; 2]>,
    pub ids: Vec<u64>,
}

pub fn parse(input: &str) -> Input {
    let (a, b) = input.split_once("\n\n").unwrap();

    Input {
        interval: a
            .lines()
            .map(|s| {
                let (a, b) = s.split_once("-").unwrap();
                [a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()]
            })
            .collect(),
        ids: b.lines().map(|s| s.parse::<u64>().unwrap()).collect(),
    }
}

pub fn solve(input: &Input) -> (u64, u64) {
    let merged = merge_intervals(&input.interval);
    let p1 = input
        .ids
        .iter()
        .filter(|id| {
            let idx = merged.partition_point(|&[start, _]| start <= **id);
            idx > 0 && **id <= merged[idx - 1][1]
        })
        .count() as u64;

    let p2 = merged.iter().map(|s| s[1] - s[0] + 1).sum::<u64>();

    (p1, p2)
}

fn merge_intervals(intervals: &[[u64; 2]]) -> Vec<[u64; 2]> {
    let mut sorted = intervals.to_vec();
    sorted.sort_unstable();

    let mut merged = Vec::with_capacity(sorted.len());
    merged.push(sorted[0]);

    for &[start, end] in &sorted[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last[1].saturating_add(1) {
            last[1] = last[1].max(end);
        } else {
            merged.push([start, end]);
        }
    }

    merged
}
