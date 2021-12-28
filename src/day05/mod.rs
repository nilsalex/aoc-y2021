use rustc_hash::FxHashSet;
use std::num::ParseIntError;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Error {
    ParseError(ParseIntError),
    IOError,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IOError
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseError(error)
    }
}

struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Line {
    fn is_non_diagonal(&self) -> bool {
        self.start_x == self.end_x || self.start_y == self.end_y
    }

    fn slope_and_count(&self) -> (i32, i32, i32) {
        let dx_ = self.end_x - self.start_x;
        let dy_ = self.end_y - self.start_y;

        if dx_ == 0 {
            (0, dy_.signum(), dy_.abs())
        } else if dy_ == 0 {
            (dx_.signum(), 0, dx_.abs())
        } else {
            let gcd = num::integer::gcd(dx_.abs(), dy_.abs());
            (dx_ / gcd, dy_ / gcd, gcd)
        }
    }
}

pub fn part1(s: &str) -> usize {
    let lines: Vec<Line> = parse_input(s).unwrap();
    let non_diagonal: Vec<&Line> = lines.iter().filter(|l| l.is_non_diagonal()).collect();

    let mut points_once: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut points_twice: FxHashSet<(i32, i32)> = FxHashSet::default();

    for line in non_diagonal {
        let (dx, dy, c) = line.slope_and_count();
        for i in 0..=c {
            let point = (line.start_x + i * dx, line.start_y + i * dy);
            if points_once.contains(&point) {
                points_twice.insert(point);
            } else {
                points_once.insert(point);
            }
        }
    }

    points_twice.len()
}

pub fn part2(input_path: &str) -> usize {
    let lines: Vec<Line> = parse_input(input_path).unwrap();
    let mut points_once: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut points_twice: FxHashSet<(i32, i32)> = FxHashSet::default();

    for line in lines {
        let (dx, dy, c) = line.slope_and_count();
        for i in 0..=c {
            let point = (line.start_x + i * dx, line.start_y + i * dy);
            if points_once.contains(&point) {
                points_twice.insert(point);
            } else {
                points_once.insert(point);
            }
        }
    }

    points_twice.len()
}

fn parse_input(s: &str) -> Result<Vec<Line>, Error> {
    s.lines().map(parse_line).collect()
}

fn parse_line(input_line: &str) -> Result<Line, Error> {
    let points: Vec<&str> = input_line.split(" -> ").collect();
    let start: Vec<i32> = points[0]
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    let end: Vec<i32> = points[1]
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    Ok(Line {
        start_x: start[0],
        start_y: start[1],
        end_x: end[0],
        end_y: end[1],
    })
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day05_part1() {
    assert_eq!(part1(INPUT), 5632);
}

#[test]
fn test_day05_part2() {
    assert_eq!(part2(INPUT), 22213);
}

#[bench]
fn bench_day05_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day05_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
