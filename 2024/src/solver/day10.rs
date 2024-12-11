use std::collections::{HashSet, VecDeque};

pub struct Input {
    pub maps: Vec<Vec<i32>>,
    pub trailheads: Vec<(usize, usize)>,
}

pub fn parse(input: &str) -> Input {
    let maps = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c as u8 - b'0') as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut trailheads = Vec::new();
    for i in 0..maps.len() {
        for j in 0..maps[i].len() {
            if maps[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    Input { maps, trailheads }
}

pub fn solve(input: &Input) -> (u32, u32) {
    (
        input
            .trailheads
            .iter()
            .map(|pos| bfs(*pos, &input.maps))
            .sum(),
        input
            .trailheads
            .iter()
            .map(|pos| dfs(*pos, &input.maps))
            .sum(),
    )
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn bfs(start: (usize, usize), maps: &[Vec<i32>]) -> u32 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(start);
    seen.insert(start);

    let mut scores = 0;
    while !queue.is_empty() {
        let now = queue.pop_front().unwrap();

        if maps[now.0][now.1] == 9 {
            scores += 1;
            continue;
        }

        for i in 0..4 {
            let x = now.0 as i32 + DX[i];
            let y = now.1 as i32 + DY[i];

            if x < 0 || x >= maps.len() as i32 || y < 0 || y >= maps[0].len() as i32 {
                continue;
            }

            if (maps[x as usize][y as usize] - maps[now.0][now.1]) == 1
                && !seen.contains(&(x as usize, y as usize))
            {
                seen.insert((x as usize, y as usize));
                queue.push_back((x as usize, y as usize));
            }
        }
    }

    scores
}

fn dfs(start: (usize, usize), maps: &[Vec<i32>]) -> u32 {
    if maps[start.0][start.1] == 9 {
        return 1;
    }

    let mut scores = 0;
    for i in 0..4 {
        let x = start.0 as i32 + DX[i];
        let y = start.1 as i32 + DY[i];

        if x < 0 || x >= maps.len() as i32 || y < 0 || y >= maps[0].len() as i32 {
            continue;
        }

        if (maps[x as usize][y as usize] - maps[start.0][start.1]) == 1 {
            scores += dfs((x as usize, y as usize), maps);
        }
    }

    return scores;
}
