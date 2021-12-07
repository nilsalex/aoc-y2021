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
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for position in positions {
        min = std::cmp::min(min, *position);
        max = std::cmp::max(max, *position);
    }

    let mut scores: Vec<i32> = vec![0; (max - min + 1) as usize];

    for position in positions {
        for (i, score) in scores.iter_mut().enumerate() {
            let shift = i as i32 + min;

            *score += (*position - shift).abs();
        }
    }

    let mut min_score = i32::MAX;

    for score in scores.iter() {
        min_score = std::cmp::min(min_score, *score);
    }

    min_score
}

fn part2(positions: &[i32]) -> i32 {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for position in positions {
        min = std::cmp::min(min, *position);
        max = std::cmp::max(max, *position);
    }

    let mut scores: Vec<i32> = vec![0; (max - min + 1) as usize];

    for position in positions {
        for (i, score) in scores.iter_mut().enumerate() {
            let shift = i as i32 + min;

            let dist = (*position - shift).abs();
            *score += (dist * (dist + 1)) / 2;
        }
    }

    let mut min_score = i32::MAX;

    for score in scores.iter() {
        min_score = std::cmp::min(min_score, *score);
    }

    min_score
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
