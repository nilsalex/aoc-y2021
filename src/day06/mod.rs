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
    let fish: Vec<u8> = parse_fish(s).unwrap();
    let mut fish_hist: [usize; 9] = new_hist(&fish);

    for _ in 0..80 {
        next(&mut fish_hist);
    }

    fish_hist.iter().sum()
}

pub fn part2(s: &str) -> usize {
    let fish: Vec<u8> = parse_fish(s).unwrap();
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

    fish_hist[6] += new_fish;
    fish_hist[8] = new_fish;
}

fn parse_fish(s: &str) -> Result<Vec<u8>, Error> {
    let line = s.lines().next().ok_or(Error::ParseFile)?;
    line.split(',')
        .map(|s| s.parse().map_err(Error::from))
        .collect()
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day06_part1() {
    assert_eq!(part1(INPUT), 366057);
}

#[test]
fn test_day06_part2() {
    assert_eq!(part2(INPUT), 1653559299811);
}

#[bench]
fn bench_day06_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day06_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
