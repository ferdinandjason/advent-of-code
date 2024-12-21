use std::collections::VecDeque;

use fxhash::FxHashMap;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|code| code.bytes().collect::<Vec<u8>>())
        .collect()
}

pub fn solve(input: &[Vec<u8>]) -> (u64, u64) {
    let mut numeric_map = FxHashMap::default();
    for i in 0..NUMERIC_KEYPAD.len() {
        for j in 0..NUMERIC_KEYPAD[i].len() {
            if NUMERIC_KEYPAD[i][j] == b' ' {
                continue;
            }
            numeric_map.insert(NUMERIC_KEYPAD[i][j], (i, j));
        }
    }

    let mut directional_map = FxHashMap::default();
    for i in 0..DIRECTIONAL_KEYPARD.len() {
        for j in 0..DIRECTIONAL_KEYPARD[i].len() {
            if DIRECTIONAL_KEYPARD[i][j] == b' ' {
                continue;
            }
            directional_map.insert(DIRECTIONAL_KEYPARD[i][j], (i, j));
        }
    }

    let mut memo = FxHashMap::default();

    (
        shortest_sequence_complexity(input, &numeric_map, &directional_map, &mut memo, 2),
        shortest_sequence_complexity(input, &numeric_map, &directional_map, &mut memo, 25),
    )
}

const NUMERIC_KEYPAD: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

fn numeric_keypad_bfs(from: (usize, usize), to: (usize, usize)) -> Vec<Vec<u8>> {
    let mut queue = VecDeque::new();

    queue.push_back((from, vec![]));

    let mut result = Vec::new();
    let mut minlen = usize::max_value();

    while !queue.is_empty() {
        let (current, mut path) = queue.pop_front().unwrap();

        if path.len() > minlen {
            continue;
        }

        if current == to {
            if path.len() <= minlen {
                minlen = path.len();
                path.push(b'A');
                result.push(path);
            }
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = current.0 as i32 + dx;
            let ny = current.1 as i32 + dy;

            if nx < 0 || nx >= 4 || ny < 0 || ny >= 3 {
                continue;
            }

            if NUMERIC_KEYPAD[nx as usize][ny as usize] == b' ' {
                continue;
            }

            queue.push_back((
                (nx as usize, ny as usize),
                path.iter().chain([&to_ascii((dx, dy))]).cloned().collect(),
            ));
        }
    }

    result
}

const DIRECTIONAL_KEYPARD: [[u8; 3]; 2] = [[b' ', b'^', b'A'], [b'<', b'v', b'>']];

fn directional_keypad_bfs(from: (usize, usize), to: (usize, usize)) -> Vec<String> {
    let mut queue = VecDeque::new();

    queue.push_back((from, vec![]));

    let mut result = Vec::new();
    let mut minlen = usize::max_value();

    while !queue.is_empty() {
        let (current, mut path) = queue.pop_front().unwrap();

        if path.len() > minlen {
            continue;
        }

        if current == to {
            if path.len() <= minlen {
                minlen = path.len();
                path.push(b'A');
                result.push(String::from_utf8(path).unwrap());
            }
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = current.0 as i32 + dx;
            let ny = current.1 as i32 + dy;

            if nx < 0 || nx >= 2 || ny < 0 || ny >= 3 {
                continue;
            }

            if DIRECTIONAL_KEYPARD[nx as usize][ny as usize] == b' ' {
                continue;
            }

            queue.push_back((
                (nx as usize, ny as usize),
                path.iter().chain([&to_ascii((dx, dy))]).cloned().collect(),
            ));
        }
    }

    result
}

fn to_ascii(dir: (i32, i32)) -> u8 {
    match dir {
        (0, -1) => b'<',
        (0, 1) => b'>',
        (-1, 0) => b'^',
        (1, 0) => b'v',
        _ => unreachable!(),
    }
}

fn encode_num_to_dir(
    codes: &[u8],
    numeric_map: &FxHashMap<u8, (usize, usize)>,
    direction_map: &FxHashMap<u8, (usize, usize)>,
    memo: &mut FxHashMap<(String, usize), usize>,
    depth: usize,
) -> usize {
    let mut start = (3, 2);
    let mut total = 0;
    for to in codes {
        let paths = numeric_keypad_bfs(start, numeric_map[&to]);
        let pathlen = paths
            .iter()
            .map(|path| {
                encode_dir_to_dir(
                    String::from_utf8(path.clone()).unwrap(),
                    direction_map,
                    memo,
                    depth,
                )
            })
            .min()
            .unwrap();
        total += pathlen;
        start = numeric_map[&to];
    }
    total
}

fn encode_dir_to_dir(
    dirs: String,
    direction_map: &FxHashMap<u8, (usize, usize)>,
    memo: &mut FxHashMap<(String, usize), usize>,
    depth: usize,
) -> usize {
    if let Some(&len) = memo.get(&(dirs.to_string(), depth)) {
        return len;
    }

    if depth == 0 {
        return dirs.len();
    }

    let mut start = (0, 2);
    let mut total = 0;
    for to in dirs.chars() {
        let dirss = directional_keypad_bfs(start, direction_map[&(to as u8)]);
        let pathlen = dirss
            .iter()
            .map(|dirs| encode_dir_to_dir(dirs.clone(), direction_map, memo, depth - 1))
            .min()
            .unwrap();
        total += pathlen;
        start = direction_map[&(to as u8)];
    }

    memo.insert((dirs.to_string(), depth), total);

    total
}

fn extract_numeric_from_code(codes: &[u8]) -> u64 {
    let mut number = 0u64;
    for i in 0..(codes.len() - 1) {
        number = number * 10 + (codes[i] - b'0') as u64;
    }

    number
}

fn shortest_sequence_complexity(
    codes: &[Vec<u8>],
    numeric_map: &FxHashMap<u8, (usize, usize)>,
    direction_map: &FxHashMap<u8, (usize, usize)>,
    memo: &mut FxHashMap<(String, usize), usize>,
    depth: usize,
) -> u64 {
    codes
        .iter()
        .map(|code| {
            let num = extract_numeric_from_code(&code);
            let pathlen = encode_num_to_dir(&code, numeric_map, direction_map, memo, depth);

            num * pathlen as u64
        })
        .sum()
}
