use std::{ops::{Neg, Add, AddAssign, Mul, Sub, SubAssign}, collections::{VecDeque, HashMap}, str};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Open,
    Wall,
}

enum Move {
    Left,
    Right,
    Forward(u32),
}

impl Move {
    fn parse_list(s: &str) -> Vec<Move> {
        let mut i = 0;
        let buffer = s.as_bytes();
        let mut moves = Vec::<Move>::new();

        while i < buffer.len() {
            if buffer[i] == b'L' {
                moves.push(Move::Left);
                i += 1;
                continue;
            }

            if buffer[i] == b'R' {
                moves.push(Move::Right);
                i += 1;
                continue;
            }

            let mut j = i;
            while j < buffer.len() && buffer[j] != b'L' && buffer[j] != b'R' { j += 1; }
            let num = str::from_utf8(&buffer[i..j]).unwrap().parse::<usize>().unwrap() as u32;
            moves.push(Move::Forward(num));

            i = j;
        }

        moves
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    start: i32,
    block: i32,
}

impl Grid {
    fn tile(&self, point: Point) -> Tile {
        let x = point.x as usize;
        let y = point.y as usize;
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.tiles[y * self.width + x]
        } else {
            Tile::None
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
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
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);

#[derive(Clone, Copy)]
struct Face {
    corner: Point,
    i: Vector,
    j: Vector,
    k: Vector,
}

pub struct Input {
    grid: Grid,
    moves: Vec<Move>,
}

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.rsplit_once("\n\n").unwrap();
    let grid = parse_grid(prefix);
    let moves = Move::parse_list(suffix);
    Input { grid, moves }
}

pub fn solve(input: &Input) -> (i32, i32) {
    (
        part1(input),
        part2(input),
    )
}

fn part1(input: &Input) -> i32 {
    let grid = &input.grid;
    let block = grid.block;

    let handle_none = |position, direction| {
        let reverse = direction * -block;
        let mut next = position + reverse;

        while grid.tile(next) != Tile::None {
            next += reverse;
        }

        next += direction;
        (next, direction)
    };

    password(input, handle_none)
}

fn part2(input: &Input) -> i32 {
    let grid = &input.grid;
    let block = grid.block;
    let edge = block - 1;

    let start = Face {
        corner: Point::new(grid.start - grid.start % block, 0),
        i: Vector { x: 1, y: 0, z: 0 },
        j: Vector { x: 0, y: 1, z: 0 },
        k: Vector { x: 0, y: 0, z: 1 },
    };
    let mut todo = VecDeque::from([start]);
    let mut faces = HashMap::new();
    let mut corners = HashMap::new();

    faces.insert(start.k, start);
    corners.insert(start.corner, start);

    while let Some(next) = todo.pop_front() {
        let Face { corner, i, j, k } = next;

        let neighbors = [
            Face { corner: corner + Point::new(-block, 0), i: -k, j, k: i }, // Left
            Face { corner: corner + Point::new(block, 0), i: k, j, k: -i },  // Right
            Face { corner: corner + Point::new(0, -block), i, j: -k, k: j }, // Up
            Face { corner: corner + Point::new(0, block), i, j: k, k: -j },  // Down
        ];

        for next in neighbors {
            if grid.tile(next.corner) != Tile::None && !faces.contains_key(&next.k) {
                todo.push_back(next);
                faces.insert(next.k, next);
                corners.insert(next.corner, next);
            }
        }
    }

    let handle_none = |position: Point, direction| {
        let offset = Point::new(position.x % block, position.y % block);
        let corner = position - offset;
        let Face { i, j, k, .. } = corners[&corner];
        let next_k = match direction {
            LEFT => i,
            RIGHT => -i,
            UP => j,
            DOWN => -j,
            _ => unreachable!(),
        };
        let Face { corner: next_corner, i: next_i, j: next_j, .. } = faces[&next_k];

        let next_direction = if k == next_i {
            RIGHT
        } else if k == -next_i {
            LEFT
        } else if k == next_j {
            DOWN
        } else if k == -next_j {
            UP
        } else {
            unreachable!()
        };
        
        let next_offset = match (direction, next_direction) {
            (LEFT, LEFT) => Point::new(edge, offset.y),
            (LEFT, RIGHT) => Point::new(0, edge - offset.y),
            (LEFT, DOWN) => Point::new(offset.y, 0),
            (LEFT, UP) => Point::new(edge - offset.y, edge),
            (RIGHT, LEFT) => Point::new(edge, edge - offset.y),
            (RIGHT, RIGHT) => Point::new(0, offset.y),
            (RIGHT, DOWN) => Point::new(edge - offset.y, 0),
            (RIGHT, UP) => Point::new(offset.y, edge),
            (DOWN, LEFT) => Point::new(edge, offset.x),
            (DOWN, RIGHT) => Point::new(0, edge - offset.x),
            (DOWN, DOWN) => Point::new(offset.x, 0),
            (DOWN, UP) => Point::new(edge - offset.x, edge),
            (UP, LEFT) => Point::new(edge, edge - offset.x),
            (UP, RIGHT) => Point::new(0, offset.x),
            (UP, DOWN) => Point::new(edge - offset.x, 0),
            (UP, UP) => Point::new(offset.x, edge),
            _ => unreachable!(),
        };
        let next_position = next_corner + next_offset;
        (next_position, next_direction)
    };

    password(input, handle_none)
}

fn parse_grid(input: &str) -> Grid {
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = raw.iter().map(|line| line.len()).max().unwrap();
    let height = raw.len();
    let mut tiles = vec![Tile::None; width * height];

    for (y, row) in raw.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let tile = match col {
                b'.' => Tile::Open,
                b'#' => Tile::Wall,
                _ => Tile::None,
            };
            tiles[y * width + x] = tile;
        }
    }

    let start = tiles.iter().position(|&t| t == Tile::Open).unwrap() as i32;
    let block = gcd(width, height) as i32;
    Grid { width, height, tiles, start, block }
}

fn password(input: &Input, handle_none: impl Fn(Point, Point) -> (Point, Point)) -> i32 {
    let Input { grid, moves } = input;
    let mut position = Point::new(grid.start, 0);
    let mut direction = Point::new(1, 0);

    for command in moves {
        match command {
            Move::Left => direction = direction.counter_clockwise(),
            Move::Right => direction = direction.clockwise(),
            Move::Forward(n) => {
                for _ in 0..*n {
                    let next = position + direction;
                    match grid.tile(next) {
                        Tile::Wall => break,
                        Tile::Open => position = next,
                        Tile::None => {
                            let (next_position, next_direction) = handle_none(position, direction);
                            if grid.tile(next_position) == Tile::Open {
                                position = next_position;
                                direction = next_direction;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    let position_score = 1000 * (position.y + 1) + 4 * (position.x + 1);
    let direction_score = match direction {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    };
    position_score + direction_score
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
  }
