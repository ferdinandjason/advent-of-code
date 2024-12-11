use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|s| s.as_bytes().to_owned()).collect()
}

pub fn solve(map: &[Vec<u8>]) -> (i32, i32) {
    let pos = find_start(map);
    let dir = Dir::new(-1, 0);
    let mut game = Game::new(map, pos, dir);
    let mut state = vec![vec![0; map[0].len()]; map.len()];
    let mut obstacle_pos = HashSet::new();

    loop {
        state[game.pos.x as usize][game.pos.y as usize] = 1;
        match game.next() {
            Result::Continue => continue,
            Result::Done => break,
        }
    }

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if state[i][j] == 1 && map[i][j] == '.' as u8 {
                if obstacle_pos.contains(&(i, j)) {
                    continue;
                }

                let mut map_clone = map.iter().map(|v| (*v).to_owned()).collect::<Vec<_>>();
                map_clone[i][j] = '#' as u8;

                let mut turtle_game = Game::new(&map_clone, pos.clone(), dir.clone());
                let mut hare_game = Game::new(&map_clone, pos.clone(), dir.clone());

                loop {
                    match turtle_game.next() {
                        Result::Continue => {
                            match hare_game.next() {
                                Result::Continue => (),
                                Result::Done => break,
                            }

                            match hare_game.next() {
                                Result::Continue => (),
                                Result::Done => break,
                            }
                        }
                        Result::Done => break,
                    }

                    if turtle_game.pos.x == hare_game.pos.x
                        && turtle_game.pos.y == hare_game.pos.y
                        && turtle_game.dir.x == hare_game.dir.x
                        && turtle_game.dir.y == hare_game.dir.y
                    {
                        obstacle_pos.insert((i, j));
                        break;
                    }
                }
            }
        }
    }

    (
        state.iter().map(|v| v.iter().sum::<i32>()).sum::<i32>(),
        obstacle_pos.len() as i32,
    )
}

fn find_start(input: &[Vec<u8>]) -> Position {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '^' as u8 {
                return Position::new(i as i32, j as i32);
            }
        }
    }

    Position::new(0, 0)
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, dir: &Dir) {
        self.x += dir.x;
        self.y += dir.y;
    }
}

#[derive(Clone, Copy)]
struct Dir {
    x: i32,
    y: i32,
}

impl Dir {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn rotate_right(&mut self) {
        let x = self.x;
        self.x = self.y;
        self.y = -x;
    }
}

enum Result {
    Continue,
    Done,
}

struct Game<'a> {
    map: &'a [Vec<u8>],
    pos: Position,
    dir: Dir,
}

impl<'a> Game<'a> {
    fn new(map: &'a [Vec<u8>], pos: Position, dir: Dir) -> Self {
        Self { map, pos, dir }
    }

    fn next(&mut self) -> Result {
        if self.pos.x + self.dir.x < 0
            || self.pos.x + self.dir.x >= self.map.len() as i32
            || self.pos.y + self.dir.y < 0
            || self.pos.y + self.dir.y >= self.map[0].len() as i32
        {
            return Result::Done;
        }

        while self.map[(self.pos.x + self.dir.x) as usize][(self.pos.y + self.dir.y) as usize]
            == '#' as u8
        {
            self.dir.rotate_right();
        }

        self.pos.move_to(&self.dir);
        Result::Continue
    }
}
