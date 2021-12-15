#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution for Solution {
    fn part1(&self) -> String {
        part1(&self.input_path).to_string()
    }
    fn part2(&self) -> String {
        part2(&self.input_path).to_string()
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

fn part1(input_path: &str) -> i32 {
    if let Ok(file) = File::open(input_path) {
        let lines = io::BufReader::new(file).lines();
        lines
            .filter_map(|l| l.ok().and_then(|l_| l_.parse::<i32>().ok()))
            .fold((0, i32::MAX), |(acc, prev), x| {
                (if x > prev { acc + 1 } else { acc }, x)
            })
            .0
    } else {
        panic!();
    }
}

fn part2(input_path: &str) -> i32 {
    let mut prev1: i32 = 0;
    let mut prev2: i32 = 0;
    let mut prev3: i32 = 0;
    let mut counter: i32 = 0;

    if let Ok(file) = File::open(input_path) {
        for (i, line) in io::BufReader::new(file).lines().enumerate() {
            if let Ok(num) = line {
                if let Ok(parsed) = num.parse::<i32>() {
                    match i {
                        0 => prev3 = parsed,
                        1 => prev2 = parsed,
                        2 => prev1 = parsed,
                        _ => {
                            let old_sum = prev3 + prev2 + prev1;
                            let new_sum = prev2 + prev1 + parsed;
                            if new_sum > old_sum {
                                counter += 1;
                            }
                            prev3 = prev2;
                            prev2 = prev1;
                            prev1 = parsed;
                        }
                    }
                }
            }
        }
    }

    counter
}
