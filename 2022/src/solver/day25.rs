use std::str;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve(snafus: &[&str]) -> (String, i32) {
    (
        to_snafu(snafus.iter().map(from_snafu).sum()),
        0
    )
}

fn from_snafu(snafu: &&str) -> i64 {
    snafu.bytes().fold(0, |acc, b| {
        let digit = match b {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!(),
        };

        5 * acc + digit
    })
}

fn to_snafu(mut num: i64) -> String {
    let mut snafu_vec = Vec::new();

    while num > 0 {
        let next = match num % 5 {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'=',
            4 => b'-',
            _ => unreachable!(),
        };
        snafu_vec.push(next);
        num = (num + 2) / 5;
    }

    snafu_vec.reverse();

    str::from_utf8(&snafu_vec).unwrap().to_string()
}
