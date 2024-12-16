pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Input {
    maps: Vec<Vec<u8>>,
    maps2: Vec<Vec<u8>>,
    inst: Vec<Direction>,
}

pub fn parse(input: &str) -> Input {
    let (maps, inst) = input.split_once("\n\n").unwrap();

    Input {
        maps: maps.lines().map(|line| line.bytes().collect()).collect(),
        maps2: maps
            .lines()
            .map(|line| {
                line.bytes()
                    .flat_map(|c| match c {
                        b'#' => vec![b'#', b'#'],
                        b'.' => vec![b'.', b'.'],
                        b'O' => vec![b'[', b']'],
                        b'@' => vec![b'@', b'.'],
                        _ => unimplemented!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        inst: inst
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '^' => Direction::North,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    '>' => Direction::East,
                    _ => unreachable!(),
                })
            })
            .collect(),
    }
}

pub fn solve(input: &Input) -> (usize, usize) {
    let mut maps = input.maps.clone();
    let mut start1 = find_start(&maps);
    input.inst.iter().for_each(|d| {
        start1 = simulate_move_part1(&mut maps, start1, d);
    });

    let mut maps2 = input.maps2.clone();
    let mut start2 = find_start(&maps2);
    input.inst.iter().for_each(|d| {
        start2 = simulate_move_part2(&mut maps2, start2, d);
    });

    (calculate_score(&maps, b'O'), calculate_score(&maps2, b'['))
}

fn simulate_move_part1(maps: &mut Vec<Vec<u8>>, (x, y): (i32, i32), d: &Direction) -> (i32, i32) {
    let now = maps[x as usize][y as usize];
    let (dx, dy) = d.offset();
    let (nx, ny) = (x + dx, y + dy);
    if nx < 0 || ny < 0 || nx >= maps.len() as i32 || ny >= maps[0].len() as i32 {
        return (x, y);
    }

    match maps[nx as usize][ny as usize] {
        b'.' => {
            maps[nx as usize][ny as usize] = now;
            maps[x as usize][y as usize] = b'.';

            (nx, ny)
        }
        b'#' => (x, y),
        b'O' => {
            simulate_move_part1(maps, (nx, ny), d);
            if maps[nx as usize][ny as usize] == b'.' {
                maps[nx as usize][ny as usize] = now;
                maps[x as usize][y as usize] = b'.';

                (nx, ny)
            } else {
                (x, y)
            }
        }
        _ => unimplemented!(),
    }
}

fn simulate_move_part2(maps: &mut Vec<Vec<u8>>, (x, y): (i32, i32), d: &Direction) -> (i32, i32) {
    let now = maps[x as usize][y as usize];
    let (dx, dy) = d.offset();
    let (nx, ny) = (x + dx, y + dy);
    if nx < 0 || ny < 0 || nx >= maps.len() as i32 || ny >= maps[0].len() as i32 {
        return (x, y);
    }

    match maps[nx as usize][ny as usize] {
        b'.' => {
            maps[nx as usize][ny as usize] = now;
            maps[x as usize][y as usize] = b'.';
            (nx, ny)
        }
        b'#' => (x, y),
        b'[' | b']' => match *d {
            Direction::East | Direction::West => {
                simulate_move_part2(maps, (nx, ny), d);
                if maps[nx as usize][ny as usize] == b'.' {
                    maps[nx as usize][ny as usize] = now;
                    maps[x as usize][y as usize] = b'.';

                    (nx, ny)
                } else {
                    (x, y)
                }
            }
            Direction::North | Direction::South => {
                let (pnx, pny) = boxes_part(maps, (nx, ny));
                let backup = maps.clone();

                let pos_a = simulate_move_part2(maps, (nx, ny), d);
                let pos_b = simulate_move_part2(maps, (pnx, pny), d);

                if pos_a != (nx, ny) && pos_b != (pnx, pny) {
                    maps[nx as usize][ny as usize] = now;
                    maps[x as usize][y as usize] = b'.';
                    maps[pnx as usize][pny as usize] = b'.';
                    (nx, ny)
                } else {
                    *maps = backup;
                    (x, y)
                }
            }
        },
        _ => unimplemented!(),
    }
}

fn boxes_part(maps: &Vec<Vec<u8>>, (nx, ny): (i32, i32)) -> (i32, i32) {
    if maps[nx as usize][(ny - 1) as usize] == b'[' && maps[nx as usize][ny as usize] == b']' {
        (nx, ny - 1)
    } else if maps[nx as usize][(ny + 1) as usize] == b']' && maps[nx as usize][ny as usize] == b'['
    {
        (nx, ny + 1)
    } else {
        unimplemented!()
    }
}

fn find_start(maps: &Vec<Vec<u8>>) -> (i32, i32) {
    maps.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == b'@' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap_or((0, 0))
}

fn calculate_score(maps: &Vec<Vec<u8>>, corner: u8) -> usize {
    maps.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == corner {
                    Some(i * 100 + j)
                } else {
                    None
                }
            })
        })
        .sum()
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}
