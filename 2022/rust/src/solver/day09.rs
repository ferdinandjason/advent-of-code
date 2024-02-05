use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub enum Dir {
    U, R, D, L
}

fn parse_dir(dir: &str) -> Dir {
    match dir {
        "U" => Dir::U,
        "R" => Dir::R,
        "D" => Dir::D,
        "L" => Dir::L,
        _ => unreachable!(),
    }
}

pub fn parse(input: &str) -> Vec<(Dir, u32)> {
    input.lines().map(|line| {
        let (dir, step) = line.split_once(" ").unwrap();

        (parse_dir(dir), step.parse::<u32>().unwrap())
    }).collect::<Vec<_>>()
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.x as u32);
        hasher.write_u32(self.y as u32);
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0_i32, y: 0_i32 }
    }
}

impl Point {
    fn signum(self) -> Self {
        Point { x: self.x.signum(), y: self.y.signum() }
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

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Point{ x: self.x * rhs, y: self.y * rhs}
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

fn dir_to_delta(dir: &Dir) -> Point {
    match dir {
        Dir::U => Point{ x: 0_i32, y: -1_i32 },
        Dir::R => Point{ x: 1_i32, y: 0_i32 },
        Dir::D => Point{ x: 0_i32, y: 1_i32 },
        Dir::L => Point{ x: -1_i32, y: 0_i32 },
    }
}

pub fn solve(moves: &Vec<(Dir, u32)>) -> (usize, usize) {
    (
        simulate::<2>(moves),
        simulate::<10>(moves),
    )
}

fn simulate<const N: usize>(moves: &Vec<(Dir, u32)>) -> usize {
    let mut rope = [Point::default(); N];

    let mut origin = Point::default();
    let mut xmin = i32::MAX;
    let mut ymin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymax = i32::MIN;
    moves.iter().for_each(|(dir, step)| {
        origin += dir_to_delta(dir) * (*step as i32);

        xmin = xmin.min(origin.x);
        ymin = ymin.min(origin.y);
        xmax = xmax.max(origin.x);
        ymax = ymax.max(origin.y);
    });

    let n = xmax - xmin + 1;
    let m = ymax - ymin + 1;
    let mut seen = vec![false; (n * m) as usize];
    // let mut seen = HashSet::<Point>::new();

    let mut distinct = 0;
    moves.iter().for_each(|(dir, step)| {
        for _ in 0..*step {
            rope[0] += dir_to_delta(dir);
            for i in 1..rope.len() {
                let d = rope[i - 1] - rope[i];
                if d.x.abs() > 1 || d.y.abs() > 1 {
                    rope[i] += d.signum();
                }
            }

            let last = rope.last().unwrap();
            let index = ((last.x - xmin) * m + (last.y - ymin)) as usize;

            if !seen[index] {
                seen[index] = true;
                distinct += 1;
            }
        }
    });

    distinct
}


