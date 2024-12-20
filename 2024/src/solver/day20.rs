use std::collections::VecDeque;

use fxhash::FxHashMap;

pub struct Maps {
    racetrack: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse(input: &str) -> Maps {
    let racetrack = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let (mut start, mut end) = ((0, 0), (0, 0));

    for i in 0..racetrack.len() {
        for j in 0..racetrack[0].len() {
            if racetrack[i][j] == b'S' {
                start = (i, j);
            } else if racetrack[i][j] == b'E' {
                end = (i, j);
            }
        }
    }

    return Maps {
        racetrack,
        start,
        end,
    };
}

pub fn solve(input: &Maps) -> (usize, usize) {
    let (baseline, path) = baseline_bfs(input.start, input.end, &input.racetrack);
    let mut point_scores = FxHashMap::default();

    for (i, (x, y)) in path.iter().enumerate() {
        point_scores.insert((*x, *y), i);
    }

    (
        find_savings(&point_scores, 2, 100, baseline),
        find_savings(&point_scores, 20, 100, baseline),
    )
}

fn find_savings(
    point_scores: &FxHashMap<(usize, usize), usize>,
    cheat_count: usize,
    minimum: usize,
    baseline: usize,
) -> usize {
    let mut result = 0;
    for ((x, y), score) in point_scores {
        for (dx, dy) in taxicable_between(cheat_count as i32).into_iter() {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = *x as i32 + dx;
            let ny = *y as i32 + dy;

            if point_scores.contains_key(&(nx as usize, ny as usize)) {
                let new_scores = (baseline - point_scores[&(nx as usize, ny as usize)])
                    + *score
                    + dx.abs() as usize
                    + dy.abs() as usize;

                if new_scores > baseline {
                    continue;
                }

                if baseline - new_scores >= minimum {
                    result += 1;
                }
            }
        }
    }

    result
}

fn taxicable_between(d: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for y in (-d)..=d {
        for x in (-d - y.abs())..=(d - y.abs()) {
            if x.abs() + y.abs() <= d {
                result.push((x, y));
            }
        }
    }

    result
}

fn baseline_bfs(
    start: (usize, usize),
    end: (usize, usize),
    racetrack: &Vec<Vec<u8>>,
) -> (usize, Vec<(usize, usize)>) {
    let mut queue = VecDeque::new();
    let mut path = Vec::new();

    queue.push_back((0, (-1, 0), start));
    path.push(start);
    while !queue.is_empty() {
        let (step, dir, current) = queue.pop_front().unwrap();
        if current == end {
            return (step, path);
        }

        for (dx, dy) in neighbour_direction(dir) {
            let nx = current.0 as i32 + dx;
            let ny = current.1 as i32 + dy;

            if nx < 0 || ny < 0 || nx >= racetrack.len() as i32 || ny >= racetrack[0].len() as i32 {
                continue;
            }

            if racetrack[nx as usize][ny as usize] == b'#' {
                continue;
            }

            queue.push_back((step + 1, (dx, dy), (nx as usize, ny as usize)));
            path.push((nx as usize, ny as usize));
        }
    }

    (0, path)
}

fn neighbour_direction(current: (i32, i32)) -> [(i32, i32); 3] {
    match current {
        (0, 1) => [(0, 1), (1, 0), (-1, 0)],
        (1, 0) => [(1, 0), (0, 1), (0, -1)],
        (0, -1) => [(0, -1), (1, 0), (-1, 0)],
        (-1, 0) => [(-1, 0), (0, 1), (0, -1)],
        _ => unimplemented!(),
    }
}
