use std::collections::{HashMap, HashSet};

pub struct Input {
    maps: Vec<Vec<u8>>,
    antennas: HashMap<u8, Vec<(usize, usize)>>,
}

pub fn parse(input: &str) -> Input {
    let maps = input
        .split('\n')
        .map(|s| s.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let mut antennas = HashMap::new();

    for i in 0..maps.len() {
        for j in 0..maps[0].len() {
            if maps[i][j] != '.' as u8 {
                let entry = antennas.entry(maps[i][j]).or_insert(Vec::new());
                entry.push((i, j));
            }
        }
    }

    Input { maps, antennas }
}

pub fn solve(input: &Input) -> (usize, usize) {
    let mut antinode1 = HashSet::<(usize, usize)>::new();
    let mut antinode2 = HashSet::<(usize, usize)>::new();
    for (_antenna, coords) in &input.antennas {
        for i in 0..coords.len() {
            for j in 0..i {
                let (mut x1, mut y1, mut x2, mut y2) = (coords[i].0 as isize, coords[i].1 as isize, coords[j].0 as isize, coords[j].1 as isize);
                let dx = x2 - x1;
                let dy = y2 - y1;

                if (x1 - dx) >= 0 && (x1 - dx) < input.maps.len() as isize && (y1 - dy) >= 0 && (y1 - dy) < input.maps[0].len() as isize {
                    antinode1.insert(((x1 - dx) as usize, (y1 - dy) as usize));
                    antinode2.insert(((x1 - dx) as usize, (y1 - dy) as usize));
                }

                if (x2 + dx) >= 0 && (x2 + dx) < input.maps.len() as isize && (y2 + dy) >= 0 && (y2 + dy) < input.maps[0].len() as isize {
                    antinode1.insert(((x2 + dx) as usize, (y2 + dy) as usize));
                    antinode2.insert(((x2 + dx) as usize, (y2 + dy) as usize));
                }
               
                while x1 >= 0 && x1 < input.maps.len() as isize && y1 >= 0 && y1 < input.maps[0].len() as isize {
                    antinode2.insert((x1 as usize, y1 as usize));
                    x1 -= dx;
                    y1 -= dy;
                }

                while x2 >= 0 && x2 < input.maps.len() as isize && y2 >= 0 && y2 < input.maps[0].len() as isize {
                    antinode2.insert((x2 as usize, y2 as usize));
                    x2 += dx;
                    y2 += dy;
                }
            }
        }
    }

    (antinode1.len(), antinode2.len())
}

