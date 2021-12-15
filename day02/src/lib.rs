#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<i32, i32> for Solution {
    fn part1(&self) -> i32 {
        part1(&self.input_path)
    }
    fn part2(&self) -> i32 {
        part2(&self.input_path)
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

fn part1(input_path: &str) -> i32 {
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    if let Ok(file) = File::open(input_path) {
        for line in io::BufReader::new(file).lines().flatten() {
            let split: Vec<&str> = line.split(' ').collect();
            if let Ok(parsed) = split[1].parse::<i32>() {
                match split[0] {
                    "forward" => horizontal += parsed,
                    "up" => vertical -= parsed,
                    "down" => vertical += parsed,
                    _ => {}
                }
            }
        }
    }

    horizontal * vertical
}

fn part2(input_path: &str) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(file) = File::open(input_path) {
        for line in io::BufReader::new(file).lines().flatten() {
            let split: Vec<&str> = line.split(' ').collect();
            if let Ok(parsed) = split[1].parse::<i32>() {
                match split[0] {
                    "forward" => {
                        horizontal += parsed;
                        depth += aim * parsed
                    }
                    "up" => aim -= parsed,
                    "down" => aim += parsed,
                    _ => {}
                }
            }
        }
    }

    horizontal * depth
}
