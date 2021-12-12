#![feature(test)]

use std::fs::File;
use std::io::{self, BufRead};

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(part1)
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(part2)
    }
}

fn part1() -> i32 {
    if let Ok(file) = File::open("input.txt") {
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

fn part2() -> i32 {
    let mut prev1: i32 = 0;
    let mut prev2: i32 = 0;
    let mut prev3: i32 = 0;
    let mut counter: i32 = 0;

    if let Ok(file) = File::open("day01/input.txt") {
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

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
