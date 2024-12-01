use std::ops::{BitAnd, BitOr, Not, BitAndAssign};

#[derive(Clone, Copy, Default)]
pub struct BitSet256 {
    left: u128,
    right: u128,
}

impl BitSet256 {
    fn bit_set(&mut self, offset: usize) {
        if offset < 128 {
            self.left |= 1 << (127 - offset);
        } else {
            self.right |= 1 << (255 - offset);
        }
    }

    fn count_ones(&self) -> u32 {
        self.left.count_ones() + self.right.count_ones()
    }

    fn non_zero(&self) -> bool {
        self.left != 0 || self.right != 0
    }

    fn min_set(&self) -> Option<u32> {
        if self.left != 0 {
            Some(self.left.leading_zeros())
        } else if self.right != 0 {
            Some(128 + self.right.leading_zeros())
        } else {
            None
        }
    }

    fn max_set(&self) -> Option<u32> {
        if self.right != 0 {
            Some(255 - self.right.trailing_zeros())
        } else if self.left != 0 {
            Some(127 - self.left.trailing_zeros())
        } else {
            None
        }
    }

    fn left_shift(&self) -> BitSet256 {
        BitSet256 { left: (self.left << 1) | (self.right >> 127), right: (self.right << 1) }
    }

    fn right_shift(&self) -> BitSet256 {
        BitSet256 { left: (self.left >> 1), right: (self.left << 127) | (self.right >> 1) }
    }
}

impl BitAnd for BitSet256 {
    type Output = BitSet256;

    fn bitand(self, rhs: BitSet256) -> BitSet256 {
        BitSet256 { left: self.left & rhs.left, right: self.right & rhs.right }
    }
}

impl BitOr for BitSet256 {
    type Output = BitSet256;

    fn bitor(self, rhs: BitSet256) -> BitSet256 {
        BitSet256 { left: self.left | rhs.left, right: self.right | rhs.right }
    }
}

impl Not for BitSet256 {
    type Output = BitSet256;

    fn not(self) -> BitSet256 {
        BitSet256 { left: !self.left, right: !self.right }
    }
}

impl BitAndAssign for BitSet256 {
    fn bitand_assign(&mut self, rhs: BitSet256) {
        self.left &= rhs.left;
        self.right &= rhs.right;
    }
}

const N: usize = 75 * 3;
#[derive(Clone, Copy)]
pub struct Input {
    grid: [BitSet256; N],

    north: [BitSet256; N],
    south: [BitSet256; N],
    west: [BitSet256; N],
    east: [BitSet256; N],

    start: usize,
    end: usize,
}

enum Direction {
    North,
    South,
    West,
    East,
}

pub fn parse(input: &str) -> Input {
    let offset = N / 3;
    let maps = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let default = [BitSet256::default(); N];
    let mut grid = default;

    let mut start = usize::MAX;
    let mut end = usize::MIN;
    for i in 0..maps.len() {
        for j in 0..maps[i].len() {
            if maps[i][j] == b'#' {
                start = start.min(offset + i);
                end = end.max(offset + i);
                grid[offset + i].bit_set(offset + j);
            }
        }
    }

    Input { grid, north: default, south: default, west: default, east: default, start, end }
}

fn simulate(input: &mut Input, order: &mut [Direction]) -> bool {
    let Input { grid, north, south, west, east, start, end } = input;

    let mut moved = false;
    let start = start.clone() - 3;
    let end = end.clone() + 3;
    let mut prev;
    let mut curr = !(grid[0].right_shift() | grid[0] | grid[0].left_shift());
    let mut next = !(grid[1].right_shift() | grid[1] | grid[1].left_shift());

    for i in start..end {
        prev = curr;
        curr = next;
        next = !(grid[i + 1].right_shift() | grid[i + 1] | grid[i + 1].left_shift());

        let verti = !(grid[i - 1] | grid[i] | grid[i + 1]);

        let mut up = prev;
        let mut down = next;
        let mut left = verti.right_shift();
        let mut right = verti.left_shift();

        let mut can = grid[i] & !(up & down & left & right);

        for direction in &*order {
            match direction {
                Direction::North => {
                    up &= can;
                    can &= !up;
                }
                Direction::South => {
                    down &= can;
                    can &= !down;
                }
                Direction::West => {
                    left &= can;
                    can &= !left;
                }
                Direction::East => {
                    right &= can;
                    can &= !right;
                }
            }
        }

        north[i - 1] = up;
        south[i + 1] = down;
        west[i] = left.left_shift();
        east[i] = right.right_shift();
    }

    for i in start..end {
        let up = north[i];
        let down = south[i];
        let left = west[i];
        let right = east[i];

        north[i] &= !down;
        south[i] &= !up;
        west[i] &= !right;
        east[i] &= !left;
    }

    let mut new_start = input.start;
    let mut new_end = input.end;
    for i in start..end {
        let same = grid[i] & !(north[i - 1] | south[i + 1] | west[i].right_shift() | east[i].left_shift());
        let change = north[i] | south[i] | west[i] | east[i];
        grid[i] = same | change;
        moved |= change.non_zero();


        if grid[i].non_zero() {
            new_start = new_start.min(i);
            new_end = new_end.max(i);
        }
    }

    input.start = new_start;
    input.end = new_end;

    order.rotate_left(1);
    moved
}


pub fn solve(input: &Input) -> (u32, u32) {
    let mut input = *input;
    let mut order = [Direction::North, Direction::South, Direction::West, Direction::East];

    for _ in 0..10 {
        simulate(&mut input, &mut order);
    }

    let grid = input.grid;
    let elves = grid.iter().map(BitSet256::count_ones).sum::<u32>();
    let min_y = grid.iter().filter_map(BitSet256::min_set).min().unwrap();
    let max_y = grid.iter().filter_map(BitSet256::max_set).max().unwrap();
    let min_x = input.start as u32;
    let max_x = input.end as u32;
    let part1 = (max_y - min_y + 1) * (max_x - min_x + 1) - elves;
    let mut part2 = 0;

    let mut moved = true;
    while moved {
        moved = simulate(&mut input, &mut order);
        part2 += 1;
    }

    

    (part1, part2 + 10)
}
