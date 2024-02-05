pub fn parse(input: &str) -> &str {
    input
}

pub fn solve(input: &str) -> (usize, usize) {
    let part1 = find_marker(input, 4);
    let part2 = find_marker(input, 14);

    (part1, part2)
}

struct State {
    seen: [i32; 26],
    mask: i32
}

impl Default for State {
    fn default() -> Self {
        Self {
            seen: [0; 26],
            mask: 0
        }
    }
}

impl State {
    fn push(&mut self, c: u8) {
        let index = (c - b'a') as usize;
        self.seen[index] += 1;
        self.mask |= 1 << index;
    }

    fn pop(&mut self, c: u8) {
        let index = (c - b'a') as usize;
        self.seen[index] -= 1;
        if self.seen[index] == 0 {
            self.mask ^= 1 << index;
        }
    }

    fn is_unique(&self, window: usize) -> bool {
        self.mask.count_ones() as usize == window
    }
}

fn find_marker(input: &str, window: usize) -> usize {
    let input = input.as_bytes();
    let mut state = State::default();

    (0..window).for_each(|i| state.push(input[i]));

    for i in window..input.len() {
        if state.is_unique(window) {
            return i
        }

        state.push(input[i]);
        state.pop(input[i - window]);
    }

    if state.is_unique(window) {
        return input.len() - 1
    }

    unreachable!();
}

