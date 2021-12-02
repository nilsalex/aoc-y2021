use std::fs::File;
use std::io::{self,BufRead};

fn part1() -> i32 {
    let mut first_pass: bool = true;
    let mut prev: i32 = 0;
    let mut counter: i32 = 0;

    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines().flatten() {
            if let Ok(parsed) = line.parse::<i32>() {
                if !first_pass && parsed > prev {
                    counter += 1;
                }
                first_pass = false;
                prev = parsed;
            }
        }
    }

    counter
}

fn part2() -> i32 {
    let mut prev1: i32 = 0;
    let mut prev2: i32 = 0;
    let mut prev3: i32 = 0;
    let mut counter: i32 = 0;

    if let Ok(file) = File::open("input.txt") {
        for (i, line) in io::BufReader::new(file).lines().enumerate() {
            if let Ok(num) = line {
                if let Ok(parsed) = num.parse::<i32>() {
                    match i {
                        0 => {
                            prev3 = parsed
                        }
                        1 => {
                            prev2 = parsed
                        }
                        2 => {
                            prev1 = parsed
                        }
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

