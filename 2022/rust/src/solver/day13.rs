pub fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

pub fn solve(input: &Vec<&str>) -> (usize, u32) {
    (
        part1(input),
        part2(input)
    )
}

fn part1(input: &Vec<&str>) -> usize {
    input
        .chunks_exact(2)
        .enumerate()
        .map(|(i, chunk)|
            if compare(chunk[0], chunk[1]) {
                i + 1
            } else {
                0
            }
        )
        .sum()
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut index_2 = 1;
    let mut index_6 = 2;

    input.iter().for_each(|now| {
        if compare(now, "[[2]]") {
            index_2 += 1;
        }

        if compare(now, "[[6]]") {
            index_6 += 1
        }
    });

    index_2 * index_6
}

pub struct Packet<'a> {
    signal: &'a [u8],
    index: usize,
    stack: Vec<u8>,
}

impl Packet<'_> {
    fn new(signal: &str) -> Packet<'_> {
        Packet {
            signal: signal.as_bytes(),
            index: 0,
            stack: Vec::new(),
        }
    }
}

fn compare(left: &str, right: &str) -> bool {
    let mut left = Packet::new(left);
    let mut right = Packet::new(right);

    while let (Some(a), Some(b)) = (left.next(), right.next()) {
        match (a, b) {
            (a, b) if a == b => (),
            (b']', _) => return true,
            (_, b']') => return false,
            (b'[', b) => {
                right.stack.push(b']');
                right.stack.push(b);
            },
            (a, b'[') => {
                left.stack.push(b']');
                left.stack.push(a);
            },
            (a, b) => return a < b
        }
    }

    false
}

impl Iterator for Packet<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().or_else(|| {
            if self.signal[self.index] == b'1' && self.signal[self.index + 1] == b'0' {
                self.index += 2;
                Some(b':')
            } else {
                self.index += 1;
                Some(self.signal[self.index - 1])
            }
        })
    }
}

