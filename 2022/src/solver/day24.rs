pub struct BlizzardMap {
    n: usize,
    m: usize,
    verti: Vec<Vec<u128>>,
    horiz: Vec<Vec<u128>>,
}

pub fn parse(input: &str) -> BlizzardMap {
    let maps = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let n = maps.len() - 2;
    let m = maps[0].len() - 2;

    let build_blizzard = |kind| {
        (1..=n).map(|i| {
            maps[i][1..(maps[i].len() - 1)].iter().fold(0_u128, |acc, a| {
                (acc << 1) | (*a != kind) as u128
            })
        }).collect::<Vec<_>>()
    };

    let left = build_blizzard(b'<');
    let right = build_blizzard(b'>');
    let top = build_blizzard(b'^');
    let bottom = build_blizzard(b'v');

    let mut verti = vec![vec![0; n]; n];
    for time in 0..n {
        for i in 0..n {
            let top = top[(i + time) % n];
            let bottom = bottom[(n + i - time % n) % n];

            verti[time][i] = top & bottom;
        }
    }

    let mut horiz = vec![vec![0; n]; m];
    for time in 0..m {
        for i in 0..n {
            let left = left[i] << time | left[i] >> (m - time);
            let right = right[i] >> time | right[i] << (m - time);

            horiz[time][i] = left & right;
        }
    }

    BlizzardMap { n, m, verti, horiz }
}

pub fn solve(map: &BlizzardMap) -> (usize, usize) {
    let part1 = simulate(map, 0, true);
    let back = simulate(map, part1, false);
    let part2 = simulate(map, back, true);

    (part1, part2)
}

fn simulate(map: &BlizzardMap, start: usize, forward: bool) -> usize {
    let BlizzardMap { n, m, verti, horiz } = map;
    let mut time = start;
    let mut state = vec![0; n + 1];
    

    loop {
        time += 1;

        if forward {
            state[0] |= 1 << (m - 1);
        } else {
            state[n - 1] |= 1;
        }

        let mut prev;
        let mut cur = 0;
        let mut next = state[0];

        for i in 0..*n {
            prev = cur;
            cur = next;
            next = state[i + 1];

            state[i] = (cur | cur << 1 | cur >> 1 | prev | next)
                & verti[time % n][i]
                & horiz[time % m][i];
        }

        if forward && state[n - 1] & 1 != 0 {
            break time + 1;
        }

        if !forward && state[0] & 1 << (m - 1) != 0 {
            break time + 1;
        }
        
    }
}