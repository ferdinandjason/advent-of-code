pub fn parse(input: &str) -> Vec<i32> {
    let mut x = 1;
    let mut xv = vec![1];
    for token in input.split_ascii_whitespace() {
        match token {
            "noop" | "addx" => {},
            num => {
                x += num.parse::<i32>().unwrap();
            }
        }
        xv.push(x);
    }

    xv
}

pub fn solve(xs: &Vec<i32>) -> (i32, String) {
    let part1 = xs.iter().enumerate().skip(19).step_by(40).map(|(i, x)| (i + 1) as i32 * x).sum::<i32>();
    let part2 = xs.chunks_exact(40).map(|row| {
        row.iter().enumerate().map(|(i, x)| {
            if (i as i32 - x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        }).collect::<String>()
    }).collect::<Vec<_>>().join("\n");

    (part1, part2)
}
