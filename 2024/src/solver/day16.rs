use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct Input {
    maps: Vec<Vec<u8>>,
    start: (usize, usize),
    dir: Direction,
    end: (usize, usize),
}

pub fn parse(input: &str) -> Input {
    let maps = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut start, mut end) = ((0, 0), (0, 0));
    for i in 0..maps.len() {
        for j in 0..maps[i].len() {
            if maps[i][j] == b'S' {
                start = (i, j);
            } else if maps[i][j] == b'E' {
                end = (i, j);
            }
        }
    }

    return Input {
        maps,
        start,
        dir: Direction::East,
        end,
    };
}

pub fn solve(input: &Input) -> (usize, usize) {
    dijkstra(input.maps.clone(), input.start, input.dir, input.end)
}

fn dijkstra(
    maps: Vec<Vec<u8>>,
    start: (usize, usize),
    dir: Direction,
    end: (usize, usize),
) -> (usize, usize) {
    let mut dis = vec![vec![vec![usize::MAX; 4]; maps[0].len()]; maps.len()];
    let mut before = HashMap::new();
    let mut pq = BinaryHeap::new();
    let mut spot = HashSet::new();
    let mut distance = usize::max_value();

    dis[start.0][start.1][1] = 0;
    before.insert((start, dir), vec![((0, 0), Direction::North)]);
    pq.push(Reverse((0, (start, dir, vec![start]))));

    while !pq.is_empty() {
        let Reverse((d, ((x, y), dir, path))) = pq.pop().unwrap();

        if d > dis[x][y][dir.as_index()] {
            continue;
        }

        if (x, y) == end {
            distance = d;
            for (xx, yy) in path {
                spot.insert((xx, yy));
            }
            continue;
        }

        for dd in dir.neighbours() {
            let (nx, ny) = (x as i32 + dd.to_offset().0, y as i32 + dd.to_offset().1);
            if maps[nx as usize][ny as usize] == b'#' {
                continue;
            }

            let weight = if dd == dir { 1 } else { 1001 };
            if dis[nx as usize][ny as usize][dd.as_index()] >= d + weight {
                dis[nx as usize][ny as usize][dd.as_index()] = d + weight;
                before
                    .entry(((nx as usize, ny as usize), dd))
                    .and_modify(|v| v.push(((x, y), dir)))
                    .or_insert(vec![((x, y), dir)]);
                pq.push(Reverse((
                    d + weight,
                    (
                        (nx as usize, ny as usize),
                        dd,
                        path.iter()
                            .chain(&[(ny as usize, nx as usize)])
                            .cloned()
                            .collect(),
                    ),
                )));
            }
        }
    }

    (distance, 1 + spot.len())
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }

    fn neighbours(&self) -> Vec<Direction> {
        match self {
            Direction::North => vec![Direction::North, Direction::West, Direction::East],
            Direction::South => vec![Direction::South, Direction::West, Direction::East],
            Direction::West => vec![Direction::North, Direction::South, Direction::West],
            Direction::East => vec![Direction::North, Direction::South, Direction::East],
        }
    }

    fn as_index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}
