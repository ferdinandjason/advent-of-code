use std::{collections::VecDeque, ops::{Add, AddAssign, Sub, SubAssign}};

pub fn parse(input: &str) -> (Point, Point, Vec<&[u8]>) {
    let mut dest = Point::default();
    let mut src = Point::default();
    let map = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();

    'outer: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'S' {
                src = Point{x: i as i32, y: j as i32};
            }

            if map[i][j] == b'E' {
                dest = Point{x: i as i32, y: j as i32};
                break 'outer;
            }
        }
    }

    (src, dest, map)
}

pub fn solve(input: &(Point, Point, Vec<&[u8]>)) -> (u64, u64) {
    (
        part1(input),
        part2(input),
    )
}

fn part1(input: &(Point, Point, Vec<&[u8]>)) -> u64 {
    bfs::<41, 95>(&input.2, input.0, b'E', |diff| diff <= 1_i32)
}

fn part2(input: &(Point, Point, Vec<&[u8]>)) -> u64 {
    bfs::<41, 95>(&input.2, input.1, b'a', |diff| diff >= -1_i32)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0_i32, y: 0_i32 }
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point{ x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

const DELTA: [Point; 4] = [
    Point{ x: -1_i32, y: 0_i32 },
    Point{ x: 0_i32, y: -1_i32 },
    Point{ x: 0_i32, y: 1_i32 },
    Point{ x: 1_i32, y: 0_i32 },
];

fn bfs<const N: usize, const M: usize>(input: &Vec<&[u8]>, start: Point, dest: u8, allow: impl Fn(i32) -> bool) -> u64 {
    let mut seen = [[false; M]; N];
    let mut queue = VecDeque::<(Point, u64)>::new();

    queue.push_back((start, 0));
    seen[start.x as usize][start.y as usize] = true;

    while let Some((now, step)) = queue.pop_front() {
        let noww = input[now.x as usize][now.y as usize];
        if noww == dest {
            return step
        }

        let noww = match noww {
            b'E' => b'z',
            b'S' => b'a',
            _ => noww
        };
        
        for d in DELTA {
            let next = now + d;
            if next.x < 0 || next.x >= N as i32 || next.y < 0 || next.y >= M as i32 {
                continue;
            }

            let nextt = input[next.x as usize][next.y as usize];
            let nextt = match nextt {
                b'E' => b'z',
                b'S' => b'a',
                _ => nextt
            };

            let diff = nextt as i32 - noww as i32;
            if allow(diff) && seen[next.x as usize][next.y as usize] == false {
                queue.push_back((next, step + 1));
                seen[next.x as usize][next.y as usize] = true;
            }
        }
    };


    // let temp = seen.iter().map(|x| x.iter().map(|y| match y {
    //     true => '#',
    //     false => '.',
    // }).collect::<String>()).collect::<Vec<String>>().join("\n");

    // dbg!(temp);

    0
}
