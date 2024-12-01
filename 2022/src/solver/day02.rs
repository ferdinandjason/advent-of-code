pub fn parse(input: &str) -> Vec<u8> {
    input.as_bytes().chunks_exact(4).map(|c| 3 * (c[2] - b'X') + (c[0] - b'A')).collect()
}

pub fn solve(input: &Vec<u8>) -> (u16, u16) {
    let p1_scores = vec![4, 1, 7, 8, 5, 2, 3, 9, 6];
    let p2_scores = vec![3, 1, 2, 4, 5, 6, 8, 9 ,7];
    
    let part1 = input.iter().map(|&i| p1_scores[i as usize]).sum::<u16>();
    let part2 = input.iter().map(|&i| p2_scores[i as usize]).sum::<u16>();

    (part1, part2)
}