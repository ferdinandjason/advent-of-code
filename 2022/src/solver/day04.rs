use std::ops::RangeInclusive;
use itertools::Itertools;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

pub fn parse(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| {
            let (a, b, c, d) = line
                .split(',')
                .flat_map(|chunk| chunk.split('-') ).map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();
            
            (a..=b, c..=d)
        })
        .collect()
}

pub fn solve(assignments: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> (usize, usize) {
    let part1 = assignments
        .iter()
        .filter(|(a, b)| a.contains_range(b) || b.contains_range(a))
        .count();

    let part2 = assignments
        .iter()
        .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
        .count();

    (part1, part2)
}
