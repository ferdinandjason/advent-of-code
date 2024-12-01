use itertools::Itertools;

pub fn parse(input: &str) -> [[[u16; 22]; 22]; 22] {
    let mut cube_grid = [[[0_u16; 22]; 22]; 22];

    input
        .lines()
        .map(|line| line.split(',').map(|chunk| chunk.parse::<usize>().unwrap()).collect_tuple().unwrap())
        .for_each(|(x, y, z)| {
            cube_grid[x + 1][y + 1][z + 1] = 1;
        });

    cube_grid
}

pub fn solve(cube_grid: &[[[u16; 22]; 22]; 22]) -> (u16, u16) {
    let mut cube_grid = cube_grid.to_owned();
    (
        part1(&cube_grid),
        part2(&mut cube_grid),
    )
}

fn part1(cube_grid: &[[[u16; 22]; 22]; 22]) -> u16 {
    let mut surface_area = 0;

    for i in 1..22 {
        for j in 1..22 {
            for k in 1..22 {
                if cube_grid[i][j][k] == 1 {
                    surface_area += 6 - (
                        cube_grid[i + 1][j][k] + 
                        cube_grid[i - 1][j][k] + 
                        cube_grid[i][j + 1][k] + 
                        cube_grid[i][j - 1][k] + 
                        cube_grid[i][j][k + 1] + 
                        cube_grid[i][j][k - 1]
                    )
                }
            }
        }
    }

    surface_area
}

fn part2(cube_grid: &mut [[[u16; 22]; 22]; 22]) -> u16 {
    paint(cube_grid, 0, 0, 0);

    let mut surface_area = 0;

    for i in 1..22 {
        for j in 1..22 {
            for k in 1..22 {
                if cube_grid[i][j][k] == 1 {
                    surface_area += (
                        cube_grid[i + 1][j][k] + 
                        cube_grid[i - 1][j][k] + 
                        cube_grid[i][j + 1][k] + 
                        cube_grid[i][j - 1][k] + 
                        cube_grid[i][j][k + 1] + 
                        cube_grid[i][j][k - 1]
                    ) >> 3;
                }
            }
        }
    }

    surface_area
}

fn paint(cube_grid: &mut [[[u16; 22]; 22]; 22], i: i8, j: i8, k: i8) {
    if cube_grid[i as usize][j as usize][k as usize] == 0 {
        cube_grid[i as usize][j as usize][k as usize] = 8;

        if i + 1 < 22 { paint(cube_grid, i + 1, j, k); }
        if i - 1 >= 0 { paint(cube_grid, i - 1, j, k); }
        if j + 1 < 22 { paint(cube_grid, i, j + 1, k); }
        if j - 1 >= 0 { paint(cube_grid, i, j - 1, k); }
        if k + 1 < 22 { paint(cube_grid, i, j, k + 1); }
        if k - 1 >= 0 { paint(cube_grid, i, j, k - 1); }
    }
}
