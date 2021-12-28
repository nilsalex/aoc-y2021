use std::collections::HashSet;
use std::num::ParseIntError;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    ParseFile,
    IO,
    NoWinner,
    NotAllGridsWin,
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

struct Game {
    numbers: Vec<usize>,
    grids: Vec<Vec<(usize, bool)>>,
}

pub fn part1(s: &str) -> Result<usize, Error> {
    const GRID_SIZE: usize = 5;
    let mut game = parse_input(s)?;

    for number in &game.numbers {
        for grid in game.grids.iter_mut() {
            mark_grid(grid, number);

            if won(grid, GRID_SIZE) {
                return Ok(number * score(grid));
            }
        }
    }

    Err(Error::NoWinner)
}

pub fn part2(input_path: &str) -> Result<usize, Error> {
    const GRID_SIZE: usize = 5;
    let mut game = parse_input(input_path)?;
    let grid_count = game.grids.len();

    let mut finished_grids: HashSet<usize> = HashSet::new();

    for number in &game.numbers {
        for (i, grid) in game.grids.iter_mut().enumerate() {
            mark_grid(grid, number);

            if won(grid, GRID_SIZE) {
                finished_grids.insert(i);
                if finished_grids.len() == grid_count {
                    return Ok(number * score(grid));
                }
            }
        }
    }

    Err(Error::NotAllGridsWin)
}

fn score(grid: &[(usize, bool)]) -> usize {
    grid.iter().filter(|x| !x.1).map(|x| x.0).sum()
}

fn won(grid: &[(usize, bool)], grid_size: usize) -> bool {
    for i in 0..grid_size {
        let mut row_complete: bool = true;
        let mut col_complete: bool = true;
        for j in 0..grid_size {
            if !grid[i + j * grid_size].1 {
                row_complete = false
            }
            if !grid[j + i * grid_size].1 {
                col_complete = false
            }
        }
        if row_complete || col_complete {
            return true;
        }
    }

    false
}

fn mark_grid(grid: &mut Vec<(usize, bool)>, number: &usize) {
    grid.iter_mut().for_each(|(i, flag)| {
        if *i == *number {
            *flag = true;
        }
    })
}

fn parse_numbers(line: &str) -> Result<Vec<usize>, ParseIntError> {
    line.split(',').map(|s| s.parse()).collect()
}

fn parse_grids(lines: &mut core::str::Lines) -> Result<Vec<Vec<(usize, bool)>>, ParseIntError> {
    let mut grids: Vec<Vec<(usize, bool)>> = vec![];

    let mut grid: Vec<(usize, bool)> = vec![];
    for line in lines {
        if line.is_empty() {
            grids.push(grid);
            grid = vec![]
        } else {
            grid.append(&mut parse_grid_line(line)?)
        }
    }
    grids.push(grid);
    Ok(grids)
}

fn parse_grid_line(line: &str) -> Result<Vec<(usize, bool)>, ParseIntError> {
    line.split_whitespace()
        .map(|s| s.parse().map(|parsed| (parsed, false)))
        .collect()
}

fn parse_input(s: &str) -> Result<Game, Error> {
    let mut lines = s.lines();

    let numbers = parse_numbers(lines.next().ok_or(Error::ParseFile)?)?;
    lines.next();

    let grids = parse_grids(&mut lines)?;

    Ok(Game { numbers, grids })
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day04_part1() {
    assert_eq!(part1(INPUT).unwrap(), 34506);
}

#[test]
fn test_day04_part2() {
    assert_eq!(part2(INPUT).unwrap(), 7686);
}

#[bench]
fn bench_day04_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT).unwrap())
}

#[bench]
fn bench_day04_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT).unwrap())
}
