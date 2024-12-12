pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

pub fn solve(maps: &[Vec<u8>]) -> (u32, u32) {
    let mut seen = vec![vec![false; maps[0].len()]; maps.len()];
    let (mut part1, mut part2) = (0, 0);

    for i in 0..maps.len() {
        for j in 0..maps[i].len() {
            if !seen[i][j] {
                let (area, perimeter, corners) = flood_fill((i as i32, j as i32), maps, &mut seen);
                part1 += area * perimeter;
                part2 += area * corners;
            }
        }
    }

    (part1, part2)
}

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [-1, 0, 1, 0];

fn flood_fill(start: (i32, i32), maps: &[Vec<u8>], seen: &mut Vec<Vec<bool>>) -> (u32, u32, u32) {
    let (mut area, mut perimeter, mut corners) = (1, 0, 0);
    seen[start.0 as usize][start.1 as usize] = true;

    let up = is_same_region(start, (start.0, start.1 - 1), maps);
    let right = is_same_region(start, (start.0 + 1, start.1), maps);
    let down = is_same_region(start, (start.0, start.1 + 1), maps);
    let left = is_same_region(start, (start.0 - 1, start.1), maps);
    let up_right = is_same_region(start, (start.0 + 1, start.1 - 1), maps);
    let down_right = is_same_region(start, (start.0 + 1, start.1 + 1), maps);
    let down_left = is_same_region(start, (start.0 - 1, start.1 + 1), maps);
    let up_left = is_same_region(start, (start.0 - 1, start.1 - 1), maps);

    corners += ((up + right == 2) || ((up + right) == 0 && up_right == 1)) as u32;
    corners += ((up + left == 2) || ((up + left) == 0 && up_left == 1)) as u32;
    corners += ((down + right == 2) || ((down + right) == 0 && down_right == 1)) as u32;
    corners += ((down + left == 2) || ((down + left) == 0 && down_left == 1)) as u32;
    perimeter += (up + down + right + left) as u32;

    for i in 0..4 {
        let x = start.0 as i32 + DX[i];
        let y = start.1 as i32 + DY[i];

        if x < 0 || y < 0 || x >= maps[0].len() as i32 || y >= maps.len() as i32 {
            continue;
        }

        if maps[x as usize][y as usize] != maps[start.0 as usize][start.1 as usize] {
            continue;
        }

        if maps[x as usize][y as usize] == maps[start.0 as usize][start.1 as usize]
            && !seen[x as usize][y as usize]
        {
            let (sub_area, sub_perimeter, sub_corners) =
                flood_fill((x as i32, y as i32), maps, seen);
            area += sub_area;
            perimeter += sub_perimeter;
            corners += sub_corners;
        }
    }

    return (area, perimeter, corners);
}

fn is_same_region((x1, y1): (i32, i32), (x2, y2): (i32, i32), maps: &[Vec<u8>]) -> u8 {
    if x2 < 0 || y2 < 0 || x2 >= maps[0].len() as i32 || y2 >= maps.len() as i32 {
        return 1;
    }

    (maps[x1 as usize][y1 as usize] != maps[x2 as usize][y2 as usize]) as u8
}
