pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve(input: &Vec<&str>) -> (u64, u64) {
    (
        calculate(input, 2),
        calculate(input, 12)
    )
}

fn calculate(input: &Vec<&str>, n: usize) -> u64 {
    input
        .iter()
        .map(|s| {
            let mut k = s.len() - n;
            let mut st = Vec::with_capacity(n);
            for digit in (*s).as_bytes() {
                while let Some(&top) = st.last() {
                    if k > 0 && top < digit {
                        st.pop();
                        k -= 1;
                    } else {
                        break;
                    }
                }

                st.push(digit);
            }

            st.into_iter()
                .take(n)
                .fold(0_u64, |acc, b| acc * 10 + (b - b'0') as u64)
        })
        .sum::<u64>()
}