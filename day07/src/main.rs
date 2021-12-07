use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;

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

fn part1(positions: &[i32]) -> i32 {
    let mut sorted = positions.to_vec();
    sorted.sort_unstable();

    let guesses = if sorted.len() % 2 > 0 {
        vec![sorted[sorted.len() / 2]]
    } else {
        vec![sorted[sorted.len() / 2 - 1], sorted[sorted.len() / 2]]
    };

    guesses
        .iter()
        .map(|x| result1(positions, *x))
        .min()
        .unwrap()
}

fn result1(positions: &[i32], shift: i32) -> i32 {
    positions.iter().map(|x| (*x - shift).abs()).sum()
}

fn part2(positions: &[i32]) -> i32 {
    let len: i32 = positions.len() as i32;
    let sum: i32 = positions.iter().sum();
    let guess = (2 * sum + len) / (2 * len);

    [guess - 1, guess, guess + 1]
        .iter()
        .map(|x| result2(positions, *x))
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

fn main() -> Result<(), Error> {
    const INPUT_FILE: &str = "day07/input.txt";
    let crabs: Vec<i32> = parse_crabs(INPUT_FILE)?;
    println!("{}", part1(&crabs));
    println!("{}", part2(&crabs));
    Ok(())
}
