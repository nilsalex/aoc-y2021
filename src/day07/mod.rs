use std::num::ParseIntError;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Error {
    ParseInt(ParseIntError),
    ParseFile,
    IO,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IO
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

pub fn part1(s: &str) -> usize {
    let crabs: Vec<usize> = parse_crabs(s).unwrap();
    let mut sorted = crabs.to_vec();
    sorted.sort_unstable();

    let guesses = if sorted.len() % 2 > 0 {
        vec![sorted[sorted.len() / 2]]
    } else {
        vec![sorted[sorted.len() / 2 - 1], sorted[sorted.len() / 2]]
    };

    guesses.iter().map(|x| result1(&crabs, *x)).min().unwrap()
}

fn result1(positions: &[usize], shift: usize) -> usize {
    positions.iter().map(|x| x.abs_diff(shift)).sum()
}

pub fn part2(s: &str) -> usize {
    let crabs: Vec<usize> = parse_crabs(s).unwrap();
    let len = crabs.len();
    let sum: usize = crabs.iter().sum();
    let guess = (2 * sum + len) / (2 * len);

    [guess - 1, guess, guess + 1]
        .iter()
        .map(|x| result2(&crabs, *x))
        .min()
        .unwrap()
}

fn result2(positions: &[usize], shift: usize) -> usize {
    positions
        .iter()
        .map(|x| (x.abs_diff(shift) * (x.abs_diff(shift) + 1)) / 2)
        .sum()
}

fn parse_crabs(s: &str) -> Result<Vec<usize>, Error> {
    let line = s.lines().next().ok_or(Error::ParseFile)?;
    line.split(',')
        .map(|s| s.parse().map_err(Error::from))
        .collect()
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day07_part1() {
    assert_eq!(part1(INPUT), 348996);
}

#[test]
fn test_day07_part2() {
    assert_eq!(part2(INPUT), 98231647);
}

#[bench]
fn bench_day07_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day07_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
