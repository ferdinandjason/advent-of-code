pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|s| s.as_bytes()).collect()
}

const DIR: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

pub fn solve(input: &Vec<&[u8]>) -> (u64, u64) {
    let mut map: Vec<Vec<u8>> = input.iter().map(|s| s.to_vec()).collect();
    let first = compute(&mut map);
    let mut total = first;

    loop {
        let removed = compute(&mut map);
        if removed == 0 {
            break;
        }

        total += removed;
    }

    (first, total)
}

fn compute(input: &mut Vec<Vec<u8>>) -> u64 {
    let n = input.len() as isize;
    let m = input[0].len() as isize;
    let mut removed = 0;

    for i in 0..n {
        for j in 0..m {
            if input[i as usize][j as usize] != b'@' {
                continue;
            }

            let mut cnt = 0;
            for x in 0..DIR.len() {
                let nx = i as isize + DIR[x][0];
                let ny = j as isize + DIR[x][1];

                if nx >= 0 && nx < n && ny >= 0 && ny < m {
                    if input[nx as usize][ny as usize] == b'@' {
                        cnt += 1;
                    }
                }
            }

            if cnt < 4 {
                removed += 1;
                input[i as usize][j as usize] = b'.';
            }
        }
    }

    removed
}
