use itertools::Itertools;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let a = iter.next().unwrap().parse().unwrap();
            let b = iter.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect()
}

pub fn solve(input: &[(usize, usize)]) -> (u32, String) {
    (
        bfs((0, 0), (70, 70), (71, 71), &input[0..1024]),
        find_first_bit(input).iter().join(","),
    )
}

fn find_first_bit(input: &[(usize, usize)]) -> [usize; 2] {
    let mut left = 1024;
    let mut right = input.len();
    while left < right {
        let mid = (left + right) / 2;
        if bfs((0, 0), (70, 70), (71, 71), &input[0..mid]) == 0 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    [input[left - 1].0, input[left - 1].1]
}

fn bfs(
    start: (usize, usize),
    end: (usize, usize),
    (n, m): (usize, usize),
    corrupted: &[(usize, usize)],
) -> u32 {
    let mut seen = vec![vec![false; m]; n];
    let mut corrupt = vec![vec![false; m]; n];
    let mut queue = VecDeque::new();

    for (x, y) in corrupted {
        corrupt[*y][*x] = true;
    }

    queue.push_back((0, start));
    seen[start.0][start.1] = true;

    while !queue.is_empty() {
        let (step, (x, y)) = queue.pop_front().unwrap();
        if (x, y) == end {
            return step;
        }

        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                continue;
            }

            if corrupt[nx as usize][ny as usize] {
                continue;
            }

            if seen[nx as usize][ny as usize] {
                continue;
            }

            seen[nx as usize][ny as usize] = true;
            queue.push_back((step + 1, (nx as usize, ny as usize)));
        }
    }

    0
}
