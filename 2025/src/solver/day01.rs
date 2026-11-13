pub fn parse(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|s| {
            let mut chars = s.chars();
            (chars.next().unwrap(), chars.as_str().parse::<i32>().unwrap())
        })
        .collect()
}

pub fn solve(input: &Vec<(char, i32)>) -> (i32, i32) {
    let (_, p1, p2) = input.iter().fold(
        (50, 0, 0), 
        |(dial, p1, p2), (dir, delta)| {
            let (dial, zeros) = match dir {
                'L' => {
                    let total = dial - delta;
                    (total.rem_euclid(100), ((100 - dial) % 100 + delta) / 100)
                },
                'R' => {
                    let total = dial + delta;
                    (total % 100, total / 100)
                }
                _ => unimplemented!()
            };

            (dial, p1 + i32::from(dial == 0), p2 + zeros)
        }   
    );


    (p1, p2)
}
