#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<usize, usize> for Solution {
    fn part1(&self) -> usize {
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

fn part1(input_path: &str) -> usize {
    let fish: Vec<u8> = parse_fish(input_path).unwrap();
    let mut fish_hist: [usize; 9] = new_hist(&fish);

    for _ in 0..80 {
        next(&mut fish_hist);
    }

    fish_hist.iter().sum()
}

fn part2(input_path: &str) -> usize {
    let fish: Vec<u8> = parse_fish(input_path).unwrap();
    let mut fish_hist: [usize; 9] = new_hist(&fish);

    for _ in 0..256 {
        next(&mut fish_hist);
    }

    fish_hist.iter().sum()
}

fn new_hist(fish: &[u8]) -> [usize; 9] {
    let mut fish_hist: [usize; 9] = [0; 9];

    for f in fish {
        fish_hist[*f as usize] += 1;
    }

    fish_hist
}

fn next(fish_hist: &mut [usize]) {
    let new_fish = fish_hist[0];

    for n in 0..8 {
        fish_hist[n] = fish_hist[n + 1];
    }

    fish_hist[6] = fish_hist[7] + new_fish;
    fish_hist[8] = new_fish;
}

fn parse_fish(filename: &str) -> Result<Vec<u8>, Error> {
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();
    let line = lines.next().ok_or(Error::ParseFile)??;
    line.split(',')
        .map(|s| s.parse().map_err(Error::from))
        .collect()
}
