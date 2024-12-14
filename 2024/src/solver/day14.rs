// use std::io::{stdout, Write};
//
// use image::ImageBuffer;

use core::str;

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn step(&mut self, boundary: (i32, i32)) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;

        if self.p.0 < 0 {
            self.p.0 += boundary.0;
        }

        if self.p.1 < 0 {
            self.p.1 += boundary.1;
        }

        if self.p.0 >= boundary.0 {
            self.p.0 -= boundary.0;
        }

        if self.p.1 >= boundary.1 {
            self.p.1 -= boundary.1;
        }
    }
}

const SKIP_OUTPUT_PART2: bool = true;

pub fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (pstr, vstr) = line[2..].split_once(" v=").unwrap();
            let (px, py) = pstr.split_once(",").unwrap();
            let (vx, vy) = vstr.split_once(",").unwrap();

            return Robot {
                p: (py.parse().unwrap(), px.parse().unwrap()),
                v: (vy.parse().unwrap(), vx.parse().unwrap()),
            };
        })
        .collect()
}

pub fn solve(input: &[Robot]) -> (i32, String) {
    let (n, m) = (103i32, 101i32);
    let mut robots = input.into_iter().map(|r| *r).collect::<Vec<_>>();
    let mut maps = vec![vec![0; m as usize]; n as usize];
    for i in 0..7623 {
        // uncomment to generate image frames for finding to part2, and inspect visually
        // let mut imgbuf = ImageBuffer::new(m as u32, n as u32);

        robots.iter_mut().for_each(|r| r.step((n, m)));
        // robots.iter().for_each(|r| {
        //     imgbuf.put_pixel(r.p.1 as u32, r.p.0 as u32, image::Rgb([0u8, 128u8, 0u8]));
        // });

        // imgbuf.save(format!("output/day14_{:04}.png", i)).unwrap();
        // println!("generated frame {:04}", i);
        // stdout().flush().unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(100));

        if i == 99 {
            robots.iter().for_each(|r| {
                maps[r.p.0 as usize][r.p.1 as usize] += 1;
            });
        }
    }

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for i in 0..n {
        for j in 0..m {
            if maps[i as usize][j as usize] != 0 {
                match to_quadrant((i, j), (n, m)) {
                    1 => q1 += maps[i as usize][j as usize],
                    2 => q2 += maps[i as usize][j as usize],
                    3 => q3 += maps[i as usize][j as usize],
                    4 => q4 += maps[i as usize][j as usize],
                    _ => (),
                }
            }
        }
    }

    if SKIP_OUTPUT_PART2 {
        return (q1 * q2 * q3 * q4, "7623".to_string());
    }

    let mut tree = vec![vec![b'.'; m as usize]; n as usize];
    robots.iter().for_each(|r| {
        tree[r.p.0 as usize][r.p.1 as usize] = b'*';
    });
    let xmas_tree = tree
        .iter()
        .map(|v| str::from_utf8(v).unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    (q1 * q2 * q3 * q4, xmas_tree)
}

fn to_quadrant((x, y): (i32, i32), boundary: (i32, i32)) -> u8 {
    let bound_x = (boundary.0 - 1) / 2;
    let bound_y = (boundary.1 - 1) / 2;

    if x < bound_x && y < bound_y {
        1
    } else if x < bound_x && y > bound_y {
        2
    } else if x > bound_x && y < bound_y {
        3
    } else if x > bound_x && y > bound_y {
        4
    } else {
        0
    }
}
