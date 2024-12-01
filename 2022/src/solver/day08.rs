pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

pub fn solve(tree: &Vec<Vec<u8>>) -> (u32, u64) {
    (
        part1(tree),
        part2(tree),
    )
}

fn part1(tree: &Vec<Vec<u8>>) -> u32 {
    let n = tree.len();
    let mut visible = vec![0_u128; n];

    for i in 0..n {
        let mut left = -1_i8;
        let mut right = -1_i8;
        let mut top = -1_i8;
        let mut bottom = -1_i8;

        for j in 0..n {
            if tree[i][j] as i8 > left {
                visible[i] |= 1 << j;
                left = tree[i][j] as i8;
            }

            if tree[i][n - j - 1] as i8 > right {
                visible[i] |= 1 << (n - j - 1);
                right = tree[i][n - j - 1] as i8;
            }

            if tree[j][i] as i8 > top {
                visible[j] |= 1 << i;
                top = tree[j][i] as i8;
            }

            if tree[n - j - 1][i] as i8 > bottom {
                visible[n - j - 1] |= 1 << i;
                bottom = tree[n - j - 1][i] as i8;
            }
        }
    }

    visible.iter().map(|mask| mask.count_ones()).sum::<u32>()
}

fn part2(tree: &Vec<Vec<u8>>) -> u64 {
    let n = tree.len();
    let temp = (0, n - 1);
    let mut scenic_score = vec![vec![1_u64; n]; n];

    for i in 0..n {
        let mut left = vec![(0_u8, 0_usize)];
        let mut right = vec![(0_u8, n - 1)];
        let mut top = vec![(0_u8, 0_usize)];
        let mut bottom = vec![(0_u8, n - 1)];

        for j in 0..n {
            // ------------------------- left ------------------------- \\
            if tree[i][j] < left.last().unwrap().0 {
                scenic_score[i][j] *= (j - left.last().unwrap().1) as u64;
                left.push((tree[i][j], j));
            } else {
                while !left.is_empty() && tree[i][j] > left.last().unwrap().0 {
                    left.pop();
                }
    
                scenic_score[i][j] *= (j - left.last().unwrap_or_else(|| &(0, 0)).1) as u64;
                left.push((tree[i][j], j));
            }


            // ------------------------- right ------------------------- \\
            if tree[n - i - 1][n - j - 1] < right.last().unwrap().0 {
                scenic_score[n - i - 1][n - j - 1] *= (right.last().unwrap().1 - (n - j - 1)) as u64;
                right.push((tree[n - i - 1][n - j - 1], n - j - 1));
            } else {
                while !right.is_empty() && tree[n - i - 1][n - j - 1] > right.last().unwrap().0 {
                    right.pop();
                }
    
                scenic_score[n - i - 1][n - j - 1] *= (right.last().unwrap_or_else(|| &temp).1 - (n - j - 1)) as u64;
                right.push((tree[n - i - 1][n - j - 1], n - j - 1));
            }


            // // ------------------------- top ------------------------- \\
            if tree[j][i] < top.last().unwrap().0 {
                scenic_score[j][i] *= (j - top.last().unwrap().1) as u64;
                top.push((tree[j][i], j));
            } else {
                while !top.is_empty() && tree[j][i] > top.last().unwrap().0 {
                    top.pop();
                }
    
                scenic_score[j][i] *= (j - top.last().unwrap_or_else(|| &(0, 0)).1) as u64;
                top.push((tree[j][i], j));
            }


            // // ------------------------- bottom ------------------------- \\
            if tree[n - j - 1][n - i - 1] < bottom.last().unwrap().0 {
                scenic_score[n - j - 1][n - i - 1] *= (bottom.last().unwrap().1 - (n - j - 1)) as u64;
                bottom.push((tree[n - j - 1][n - i - 1], n - j - 1));
            } else {
                while !bottom.is_empty() && tree[n - j - 1][n - i - 1] > bottom.last().unwrap().0 {
                    bottom.pop();
                }
    
                scenic_score[n - j - 1][n - i - 1] *= (bottom.last().unwrap_or_else(|| &temp).1 - (n - j - 1)) as u64;
                bottom.push((tree[n - j - 1][n - i - 1], n - j - 1));
            }
        }
    }

    *scenic_score.iter().flat_map(|v| v.iter()).max().unwrap()
}