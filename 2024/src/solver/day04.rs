use std::iter::zip;

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|s| s.as_bytes()).collect()
}

pub fn solve(input: &[&[u8]]) -> (usize, usize) {
    let (n, m) = (input.len(), input[0].len());
    let (mut xmas_count, mut x_mas_count) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            match input[i][j] {
                b'X' => xmas_count += find_xmas_neighbors(input, i as i32, j as i32),
                b'A' => x_mas_count += find_x_mas_neighbors(input, i as i32, j as i32),
                _ => (),
            }
        }
    }

    (xmas_count, x_mas_count)
}

const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

fn find_xmas_neighbors(input: &[&[u8]], x: i32, y: i32) -> usize {
    zip(DX, DY)
        .filter(|(dx, dy)| {
            let (mut nx, mut ny) = (x, y);
            let mut neighbors: [u8; 3] = [0; 3];
            for i in 0..3 {
                nx += dx;
                ny += dy;
                if nx < 0 || nx >= input.len() as i32 || ny < 0 || ny >= input[0].len() as i32 {
                    break;
                }
                neighbors[i] = input[nx as usize][ny as usize];
            }

            neighbors == *b"MAS"
        })
        .count()
}

fn find_x_mas_neighbors(input: &[&[u8]], x: i32, y: i32) -> usize {
    if x - 1 < 0 || x + 1 >= input.len() as i32 || y - 1 < 0 || y + 1 >= input[0].len() as i32 {
        return 0;
    }

    (check_cross_1(input, x as usize, y as usize) && check_cross_2(input, x as usize, y as usize))
        as usize
}

fn check_cross_1(input: &[&[u8]], x: usize, y: usize) -> bool {
    (input[x - 1][y - 1] == 'M' as u8 && input[x + 1][y + 1] == 'S' as u8)
        || (input[x - 1][y - 1] == 'S' as u8 && input[x + 1][y + 1] == 'M' as u8)
}

fn check_cross_2(input: &[&[u8]], x: usize, y: usize) -> bool {
    (input[x - 1][y + 1] == 'M' as u8 && input[x + 1][y - 1] == 'S' as u8)
        || (input[x - 1][y + 1] == 'S' as u8 && input[x + 1][y - 1] == 'M' as u8)
}
