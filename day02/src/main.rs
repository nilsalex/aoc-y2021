use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "day02/input.txt";

fn part1() -> i32 {
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    if let Ok(file) = File::open(INPUT_FILE) {
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

fn part2() -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(file) = File::open(INPUT_FILE) {
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

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
