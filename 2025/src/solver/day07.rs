pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|s| s.as_bytes()).collect()
}

pub fn solve(input: &Vec<&[u8]>) -> (u64, u64) {
    let (n, m) = (input.len(), input[0].len());
    let mut dp: Vec<Vec<u64>> = vec![vec![0u64; m + 1]; n + 1];
    let mut cnt = 0;

    for i in 0..n {
        if i > 0 {
            dp[i] = dp[i - 1].clone();
        }

        for j in 0..m {
            if input[i][j] == b'S' {
                dp[i][j] = 1u64;
            }

            if input[i][j] == b'^' && dp[i - 1][j] != 0 {
                cnt += 1;
                dp[i][j - 1] += dp[i - 1][j];
                dp[i][j + 1] += dp[i - 1][j];
                dp[i][j] = 0;
            }
        }
    }

    (cnt, (0..m).into_iter().map(|i| dp[n - 1][i]).sum())
}
