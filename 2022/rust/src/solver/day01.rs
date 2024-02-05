use std::collections::BinaryHeap;

pub fn parse(input: &str) -> Vec<u32> {
    input.split("\n\n")
        .map(|s| s.lines().map(|x| x.parse::<u32>().unwrap() ).sum() )
        .collect()
}

pub fn solve(calories: &Vec<u32>) -> (u32, u32) {
    let mut heap = BinaryHeap::new();
    for calories in calories { heap.push(calories) }

    let part1 = *heap.pop().unwrap();
    let part2 = part1 + heap.pop().unwrap() + heap.pop().unwrap();

    (part1, part2)
}
