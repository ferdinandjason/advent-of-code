use std::{str, ops::{Add, Sub, Mul}};

const ORE: Mineral = Mineral::new(1, 0, 0, 0);
const CLAY: Mineral = Mineral::new(0, 1, 0, 0);
const OBSIDIAN: Mineral = Mineral::new(0, 0, 1, 0);
const GEODE: Mineral = Mineral::new(0, 0, 0, 1);
const ROBOT_TYPE: [Mineral; 4] = [
    ORE, CLAY, OBSIDIAN, GEODE
];
const INITIAL_ROBOT: Mineral = Mineral::new(1, 0, 0, 0);
const INITIAL_RESOURCES: Mineral = Mineral::new(0, 0, 0, 0);

#[derive(Debug, Copy, Clone)]
struct Mineral {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Mineral {
    const fn new(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Self{ ore, clay, obsidian, geode }
    }

    fn gt(&self, other: &Self) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }

    fn can(&self, other: &Self) -> bool {
        let enough_ore = if other.ore != 0 { self.ore > 0 } else { true };
        let enough_clay = if other.clay != 0 { self.clay > 0 } else { true };
        let enough_obsidian = if other.obsidian != 0 { self.obsidian > 0 } else { true };
        let enough_geode = if other.geode != 0 { self.geode > 0 } else { true };

        enough_ore && enough_clay && enough_obsidian && enough_geode
    }

    fn index(&self, i: u32) -> u32 {
        match i {
            0 => self.ore,
            1 => self.clay,
            2 => self.obsidian,
            3 => self.geode,
            _ => 0,
        }
    }
}

impl Add for Mineral {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Mineral {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Mineral {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Mineral {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Mul<u32> for Mineral {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Mineral {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}


pub struct Blueprint {
    id: u32,
    ore_robot_cost: Mineral,
    clay_robot_cost: Mineral,
    obsidian_robot_cost: Mineral,
    geode_robot_cost: Mineral,

    max: Mineral,
}

impl Blueprint {
    fn cost_for(&self, robot_id: u32) -> Mineral {
        match robot_id {
            0 => self.ore_robot_cost,
            1 => self.clay_robot_cost,
            2 => self.obsidian_robot_cost,
            3 => self.geode_robot_cost,
            _ => unreachable!(),
        }
    }
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let nums = line
                .as_bytes()
                .split(|c| c.is_ascii_whitespace() || c.is_ascii_alphabetic() || c == &b'.' || c == &b':')
                .filter(|c| !c.is_empty())
                .map(|c| str::from_utf8(c).unwrap().parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            Blueprint{
                id: nums[0],
                ore_robot_cost: Mineral::new(nums[1], 0, 0, 0),
                clay_robot_cost: Mineral::new(nums[2], 0, 0, 0),
                obsidian_robot_cost: Mineral::new(nums[3], nums[4], 0, 0),
                geode_robot_cost: Mineral::new(nums[5], 0, nums[6], 0),
                max: Mineral::new(nums[1].max(nums[2]).max(nums[3]).max(nums[5]), nums[4], nums[6], 0)
            }
        })
        .collect::<Vec<_>>()
}

pub fn solve(blueprints: &Vec<Blueprint>) -> (u32, u32) {
    (
        part1(blueprints),
        part2(blueprints)
    )
}

fn part1(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.iter().map(|blueprint| blueprint.id * find_max_geodes(blueprint, 24)).sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.iter().take(3).map(|blueprint| find_max_geodes(blueprint, 32)).product()
}

fn find_max_geodes(blueprint: &Blueprint, time: i32) -> u32{
    let mut result = 0_u32;
    maximize(blueprint, time, &mut result, &INITIAL_ROBOT, &INITIAL_RESOURCES);

    result
}

fn maximize(blueprint: &Blueprint, time_remaining: i32, result: &mut u32, robot: &Mineral, resources: &Mineral) {
    if time_remaining < 0 { return; }
    if theoretical_max_geode(resources.geode, robot.geode, time_remaining as u32) <= (*result) { return; }

    (*result) = (*result).max(resources.geode + robot.geode * time_remaining as u32);

    for robot_id in 0..4 {
        let robot_cost = blueprint.cost_for(robot_id);
        let resources = resources.clone();
        let robot = robot.clone();
        if robot_lt_max_mineral(&robot, &blueprint.max, robot_id) && (resources + robot).can(&robot_cost) && time_remaining > 1 {
            create_new_robot(blueprint, time_remaining, result, robot_cost, ROBOT_TYPE[robot_id as usize], robot, resources);
        }
    }
}

fn create_new_robot(blueprint: &Blueprint, time: i32, result: &mut u32, cost: Mineral, new_robot: Mineral, robot: Mineral, mut resources: Mineral) {
    for i in 1..time {
        if resources.gt(&cost) {
            if time - i >= 0 {
                maximize(blueprint, time - i, result, &(robot + new_robot), &(resources + robot - cost));
            }
            break;
        }
        resources = resources + robot;
    }
}

fn theoretical_max_geode(resource_geode: u32, robot_geode: u32, time: u32) -> u32 {
    resource_geode + (2 * robot_geode * time + time * time + time) / 2
}

fn robot_lt_max_mineral(robot: &Mineral, max: &Mineral, robot_id: u32) -> bool {
    if robot_id == 3 { return true; }

    robot.index(robot_id) < max.index(robot_id)
}

