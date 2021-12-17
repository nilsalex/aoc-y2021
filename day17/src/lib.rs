#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<i32, usize> for Solution {
    fn part1(&self) -> i32 {
        part1(&self.input_path)
    }
    fn part2(&self) -> usize {
        part2(&self.input_path)
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

struct State {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl State {
    fn initial(vx: i32, vy: i32) -> Self {
        State { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vx = match self.vx.cmp(&0) {
            std::cmp::Ordering::Less => self.vx + 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.vx - 1,
        };
        self.vy -= 1;
    }

    fn out_of_bounds(&self, x_max: i32, y_min: i32) -> bool {
        self.x > x_max || self.y < y_min
    }

    fn in_target(&self, x_target: (i32, i32), y_target: (i32, i32)) -> bool {
        self.x >= x_target.0 && self.x <= x_target.1 && self.y >= y_target.0 && self.y <= y_target.1
    }
}

fn vx_range(x_target: &(i32, i32)) -> std::ops::RangeInclusive<i32> {
    let min = (0.5 + (0.25 + (2 * x_target.0) as f32).sqrt()).floor() as i32;
    let max = x_target.1;

    min..=max
}

fn parse_input(input_path: &str) -> ((i32, i32), (i32, i32)) {
    let file = File::open(input_path).unwrap();
    let line = io::BufReader::new(file).lines().flatten().next().unwrap();
    let trimmed = line.trim_start_matches("target area: ");
    let (mut str1, mut str2) = trimmed.split_once(", ").unwrap();

    str1 = str1.trim_start_matches("x=");
    str2 = str2.trim_start_matches("y=");

    let (x_min_str, x_max_str) = str1.split_once("..").unwrap();
    let (y_min_str, y_max_str) = str2.split_once("..").unwrap();

    (
        (x_min_str.parse().unwrap(), x_max_str.parse().unwrap()),
        (y_min_str.parse().unwrap(), y_max_str.parse().unwrap()),
    )
}

fn part1(input_path: &str) -> i32 {
    let (_, y_target) = parse_input(input_path);

    let vy: i32 = -(1 + y_target.0);

    (vy * (vy + 1)) / 2
}

fn part2(input_path: &str) -> usize {
    let (x_target, y_target) = parse_input(input_path);

    let mut result: usize = 0;

    for vx in vx_range(&x_target) {
        for vy in y_target.0..=500 {
            let mut state = State::initial(vx, vy);

            loop {
                if state.out_of_bounds(x_target.1, y_target.0) {
                    break;
                }

                if state.in_target(x_target, y_target) {
                    result += 1;
                    break;
                }

                state.step()
            }
        }
    }

    result
}
