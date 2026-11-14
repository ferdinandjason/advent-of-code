pub struct Input {
    pub numbers: Vec<Vec<u64>>,
    pub numbers_rtl: Vec<Vec<u64>>,
    pub op: Vec<u8>,
}

pub fn parse(input: &str) -> Input {
    let (nums, ops) = input.trim_end().rsplit_once('\n').unwrap();

    Input {
        numbers: nums
            .lines()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect(),
        numbers_rtl: as_rtl_numbers(nums.lines().map(str::as_bytes).collect()),
        op: ops
            .split_ascii_whitespace()
            .map(|s| s.as_bytes()[0])
            .collect(),
    }
}

pub fn solve(input: &Input) -> (u64, u64) {
    let p1 = (0..input.op.len())
        .into_iter()
        .map(|i| {
            let mut ans = if input.op[i] == b'+' { 0u64 } else { 1u64 };

            for nums in &input.numbers {
                match input.op[i] {
                    b'+' => ans += nums[i],
                    b'*' => ans *= nums[i],
                    _ => unimplemented!(),
                }
            }

            ans
        })
        .sum::<u64>();

    let p2 = (0..input.op.len())
        .into_iter()
        .map(|i| {
            let mut ans = if input.op[i] == b'+' { 0u64 } else { 1u64 };

            for nums in &input.numbers_rtl[i] {
                match input.op[i] {
                    b'+' => ans += nums,
                    b'*' => ans *= nums,
                    _ => unimplemented!(),
                }
            }

            ans
        })
        .sum::<u64>();

    (p1, p2)
}

fn as_rtl_numbers(grid: Vec<&[u8]>) -> Vec<Vec<u64>> {
    let m = grid.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut blocks = Vec::new();
    let mut current = Vec::new();

    for col in 0..m {
        let mut num: u64 = 0;
        let mut has_digit = false;

        for row in &grid {
            let b = row.get(col).copied().unwrap_or(b' ');
            if b.is_ascii_digit() {
                num = num * 10 + (b - b'0') as u64;
                has_digit = true;
            }
        }

        if has_digit {
            current.push(num);
        } else if !current.is_empty() {
            blocks.push(std::mem::take(&mut current));
        }
    }

    if !current.is_empty() {
        blocks.push(current);
    }

    blocks
}
