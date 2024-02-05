use std::{ops::RangeInclusive, collections::HashSet};

use itertools::Itertools;

pub struct Pos{
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl Pos {
    fn distance(&self) -> i32 {
        (self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs()
    }

    fn beacon_range_on_y(&self, y: i32) -> (i32, i32) {
        let d = self.distance();
        let (bx, by) = self.sensor;
        let dy = (y - by).abs();

        if d - dy < 0 {
            return (0, 0)
        }

        let x1 = d - dy + bx;
        let x2 = dy - d + bx;

        (x1.min(x2), x1.max(x2))
    }
}

pub fn parse(input: &str) -> Vec<Pos> {
    input.lines().map(|line| {
        let nums = line
            .split(|c: char| c.is_ascii_whitespace() || c.is_ascii_alphabetic() || c == ',' || c == ':' || c == '=')
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Pos {
            sensor: (nums[0], nums[1]),
            beacon: (nums[2], nums[3])
        }
    }).collect()
}

pub fn solve(pos: &Vec<Pos>) -> (i64, i64) {
    (
        part1(pos),
        part2(pos),
    )
}

const Y_AXIS: u64 = 2_000_000;
const MAX_AXIS: i64 = 4_000_000;
const BEACON_MAX_RANGE: RangeInclusive<i64> = 0..=MAX_AXIS;
const ROTATE_CONSTANT_SQUARE_DENOMINATOR: i32 = 2;

fn part1(pos: &Vec<Pos>) -> i64 {
    let mut coverage_ranges = pos.iter()
        .map(|pos| pos.beacon_range_on_y(Y_AXIS as i32))
        .filter(|beacon_range| *beacon_range != (0, 0))
        .collect::<Vec<_>>();
    coverage_ranges.sort();

    let mut merged_coverage_ranges = vec![coverage_ranges[0]];
    coverage_ranges.iter().skip(1).for_each(|beacon_range| {
        let last = merged_coverage_ranges.last_mut().unwrap();

        if beacon_range.0 <= last.1 {
            last.1 = last.1.max(beacon_range.1)
        } else {
            merged_coverage_ranges.push(*beacon_range);
        }
    });

    merged_coverage_ranges.iter().fold(0_i64, |acc, (from, to)| {
        acc + (to - from + 1) as i64
    }) - pos.iter().filter(|pos| pos.beacon.1 == Y_AXIS as i32).map(|pos| pos.beacon.0).unique().count() as i64
}

fn part2(pos: &Vec<Pos>) -> i64 {
    let mut top = HashSet::new();
    let mut bottom = HashSet::new();
    let mut right = HashSet::new();
    let mut left = HashSet::new();

    pos.iter().for_each(|pos| {
        let d = pos.distance();
        
        // rotate (without the constant) and extend the distance by 1
        top.insert(pos.sensor.0 - pos.sensor.1 - d - 1);
        bottom.insert(pos.sensor.0 - pos.sensor.1 + d + 1);
        left.insert(pos.sensor.0 + pos.sensor.1 - d - 1);
        right.insert(pos.sensor.0 + pos.sensor.1 + d + 1);
    });

    let horizontal = top.intersection(&bottom).collect::<Vec<_>>(); // should only 1
    let vertical = left.intersection(&right).collect::<Vec<_>>();   // should only 1

    let mut tunning_frequency = 0_i64;
    for &&x in &horizontal {
        for &&y in &vertical {

            // rotate back (include the constant) to the original position
            let xx = ((x + y) / ROTATE_CONSTANT_SQUARE_DENOMINATOR) as i64;
            let yy = ((y - x) / ROTATE_CONSTANT_SQUARE_DENOMINATOR) as i64;

            if BEACON_MAX_RANGE.contains(&xx) && BEACON_MAX_RANGE.contains(&yy) {
                tunning_frequency = xx * MAX_AXIS + yy;
            }
        }
    }

    tunning_frequency
}
