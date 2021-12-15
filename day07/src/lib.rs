#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
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

#[derive(Debug)]
enum Error {
    ParseInt(ParseIntError),
    ParseFile,
    IO,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::IO
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

fn part1(input_path: &str) -> i32 {
    let crabs: Vec<i32> = parse_crabs(input_path).unwrap();
    let mut sorted = crabs.to_vec();
    sorted.sort_unstable();

    let guesses = if sorted.len() % 2 > 0 {
        vec![sorted[sorted.len() / 2]]
    } else {
        vec![sorted[sorted.len() / 2 - 1], sorted[sorted.len() / 2]]
    };

    guesses.iter().map(|x| result1(&crabs, *x)).min().unwrap()
}

fn result1(positions: &[i32], shift: i32) -> i32 {
    positions.iter().map(|x| (*x - shift).abs()).sum()
}

fn part2(input_path: &str) -> i32 {
    let crabs: Vec<i32> = parse_crabs(input_path).unwrap();
    let len: i32 = crabs.len() as i32;
    let sum: i32 = crabs.iter().sum();
    let guess = (2 * sum + len) / (2 * len);

    [guess - 1, guess, guess + 1]
        .iter()
        .map(|x| result2(&crabs, *x))
        .min()
        .unwrap()
}

fn result2(positions: &[i32], shift: i32) -> i32 {
    positions
        .iter()
        .map(|x| ((*x - shift).abs() * ((*x - shift).abs() + 1)) / 2)
        .sum()
}

fn parse_crabs(filename: &str) -> Result<Vec<i32>, Error> {
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();
    let line = lines.next().ok_or(Error::ParseFile)??;
    line.split(',')
        .map(|s| s.parse().map_err(Error::from))
        .collect()
}
