#[derive(Clone)]
pub struct Cave {
    n: usize,
    map: Vec<Vec<u8>>,

    count: u32
}

impl Cave {
    fn simulate_fall(&mut self, start: (u32, u32), overflow: u8) -> u8 {
        let can = self.can_stop_to((start.0 + 1, start.1), overflow) &&
                        self.can_stop_to((start.0 + 1, start.1 - 1), overflow) &&
                        self.can_stop_to((start.0 + 1, start.1 + 1), overflow);

        if can {
            self.count += 1;
            self.map[start.0 as usize][start.1 as usize] = b'o';
            b'o'
        } else {
            self.map[start.0 as usize][start.1 as usize] = b'~';
            b'~'
        }
    }

    fn can_stop_to(&mut self, coord: (u32, u32), overflow: u8) -> bool {
        let fall_to = if coord.0 >= self.n as u32 {
            overflow
        } else if self.map[coord.0 as usize][coord.1 as usize] == b'.' {
            self.simulate_fall(coord, overflow)
        } else {
            self.map[coord.0 as usize][coord.1 as usize]
        };

        fall_to == b'#' || fall_to == b'o'
    }

    // fn print(&mut self) -> String {
    //     self.map.iter().map(|row| String::from_utf8(row[390..612].to_vec()).unwrap()).collect::<Vec<_>>().join("\n")
    // }
}

pub fn parse(input: &str) -> Cave {
    let paths = input.lines().map(|line| {
        line.split(" -> ").map(|chunk| {
            let (x, y) = chunk.split_once(",").unwrap();

            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let x = paths.iter().flat_map(|x| x.iter().map(|x| x.0)).max().unwrap() as usize + 2_usize;
    let y = paths.iter().flat_map(|x| x.iter().map(|x| x.1)).max().unwrap() as usize + 2_usize;

    let n = y;
    let m = x * 2;

    let mut map = vec![vec![b'.'; m]; n];    

    paths.iter().for_each(|path| {
        for i in 1..path.len() {
            let (x1, y1, x2, y2) = (
                path[i].0.min(path[i - 1].0), path[i].1.min(path[i - 1].1), 
                path[i].0.max(path[i - 1].0), path[i].1.max(path[i - 1].1)
            );

            let dx = x2 - x1;
            let dy = y2 - y1;

            if dx != 0 {
                for x in x1..=x2 {
                    map[y1 as usize][x as usize] = b'#';
                }
            }

            if dy != 0 {
                for y in y1..=y2 {
                    map[y as usize][x1 as usize] = b'#';
                }
            }
        }
    });

    Cave {n, map, count: 0}
}

pub fn solve(input: &Cave) -> (u32, u32) {
    (
        simulate(input, b'~'),
        simulate(input, b'#')
    )
}

fn simulate(input: &Cave, overflow: u8) -> u32 {
    let mut cave = input.clone();
    cave.simulate_fall((0, 500), overflow);

    cave.count
}
