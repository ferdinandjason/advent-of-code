use std::collections::HashMap;
use std::iter::Cycle;
use std::slice::Iter;

/*
ROCKS 
....... 0x00
....... 0x00
..####. 0x3C

...#.... 0x10
..###... 0x38
...#.... 0x10

....#... 0x08
....#... 0x08
..###... 0x38

..#..... 0x20
..#..... 0x20
..#..... 0x20
..#..... 0x20

........ 0x00
..##.... 0x30
..##.... 0x30
 */

const FLOOR: u8 = 0xff;
const WALLS: u32 = 0x01010101;
const ROCKS: [Rock; 5] = [
    Rock { size: 1, shape: 0x0000003C }, // -
    Rock { size: 3, shape: 0x00103810 }, // +
    Rock { size: 3, shape: 0x00080838 }, // L
    Rock { size: 4, shape: 0x20202020 }, // |
    Rock { size: 2, shape: 0x00003030 }, // .
];

#[derive(Copy, Clone)]
struct Rock {
    size: usize,
    shape: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum JetPattern {
    Left,
    Right
}

impl JetPattern {
    fn new(char: u8) -> JetPattern {
        match char {
            b'<' => JetPattern::Left,
            b'>' => JetPattern::Right,
            _ => unreachable!(),
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            JetPattern::Left => 0,
            JetPattern::Right => 1,
        }
    }
}

struct State<'a> {
    jets: Cycle<Iter<'a, JetPattern>>,
    rocks: Cycle<Iter<'a, Rock>>,

    tower: Vec<u8>,
    height: u64,

    seen: HashMap<(u64, u64, u64), (u64, u64)>
}

impl State<'_> {
    fn new(jet: &Vec<JetPattern>) -> State<'_> {
        let mut tower = vec![0; 3300];
        tower[0] = FLOOR;

        State {
            jets: jet.iter().cycle(),
            rocks: ROCKS.iter().cycle(),
            tower,
            height: 0,
            seen: HashMap::<(u64, u64, u64), (u64, u64)>::new(),
        }
    }

    fn simulate(&mut self) -> (u64, u64, u64) {
        let Rock {size, mut shape} = self.rocks.next().unwrap();
        let mut wall = WALLS;
        let mut index = self.height + 3;
        let current_rocks = shape;

        loop {
            let jet = self.jets.next().unwrap();
            let rock = match jet {
                JetPattern::Left => shape.rotate_left(1),
                JetPattern::Right => shape.rotate_right(1)
            };
            
            if rock & wall == 0 {
                shape = rock
            }

            wall = (wall << 8) | WALLS | (self.tower[index as usize] as u32);

            if shape & wall == 0 {
                index -= 1
            } else {
                let bytes = shape.to_le_bytes();
                self.tower[index as usize + 1] |= bytes[0];
                self.tower[index as usize + 2] |= bytes[1];
                self.tower[index as usize + 3] |= bytes[2];
                self.tower[index as usize + 4] |= bytes[3];
                self.height = self.height.max(index + *size as u64);

                break (self.to_seen_state(current_rocks, jet.to_usize()));
            }
        }
    }

    fn to_seen_state(&self, current_rocks: u32, jet_usize: usize) -> (u64, u64, u64) {
        let index = (self.height as i32 - 20).max(0) as usize;

        (
            ((jet_usize as u64) << 63) |
            ((current_rocks as u64) << 32) |
            ((self.tower[index + 19] as u64) << 24) | 
            ((self.tower[index + 18] as u64) << 16) |
            ((self.tower[index + 17] as u64) << 8) |
              self.tower[index + 16] as u64,

            ((self.tower[index + 15] as u64) << 56) | 
            ((self.tower[index + 14] as u64) << 48) | 
            ((self.tower[index + 13] as u64) << 40) | 
            ((self.tower[index + 12] as u64) << 32) | 
            ((self.tower[index + 11] as u64) << 24) | 
            ((self.tower[index + 10] as u64) << 16) |
            ((self.tower[index + 9] as u64) << 8) |
              self.tower[index + 8] as u64,

            ((self.tower[index + 7] as u64) << 56) | 
            ((self.tower[index + 6] as u64) << 48) | 
            ((self.tower[index + 5] as u64) << 40) | 
            ((self.tower[index + 4] as u64) << 32) | 
            ((self.tower[index + 3] as u64) << 24) | 
            ((self.tower[index + 2] as u64) << 16) |
            ((self.tower[index + 1] as u64) << 8) |
              self.tower[index] as u64,
        )
    }

    fn simulate_until(&mut self, n: u64) -> u64 {
        let mut current_rock = 0;
        let mut height_add = 0;
        let find_cycle = n > 3300;
        
        while current_rock < n { current_rock += 1;
            let state = self.simulate();

            if find_cycle {
                if let Some((cycle_height, cycle_start)) = self.seen.get(&state) {
                    let left = n - current_rock;
                    if left == 0 {
                        break;
                    }
                    
                    let count = left / (current_rock - *cycle_start);
                    current_rock += count * (current_rock - cycle_start);
                    height_add += count * (self.height - *cycle_height);
    
                } else {
                    self.seen.insert(state, (self.height, current_rock));
                }
            }
        }

        self.height + height_add
    }
}

pub fn parse(input: &str) -> Vec<JetPattern> {
    input.bytes().map(JetPattern::new).collect()
}

pub fn solve(input: &Vec<JetPattern>) -> (u64, u64) {
    (
        part1(input),
        part2(input),
    )
}

fn part1(input: &Vec<JetPattern>) -> u64 {
    let mut state = State::new(input);
    state.simulate_until(2022)
}

fn part2(input: &Vec<JetPattern>) -> u64 {
    let mut state = State::new(input);
    state.simulate_until(1_000_000_000_000)
}
