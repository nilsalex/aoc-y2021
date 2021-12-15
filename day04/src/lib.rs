#![feature(test)]
mod bench;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<i32, i32> for Solution {
    fn part1(&self) -> i32 {
        part1(&self.input_path).unwrap()
    }
    fn part2(&self) -> i32 {
        part2(&self.input_path).unwrap()
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    ParseFile,
    IO,
    NoWinner,
    NotAllGridsWin,
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

struct Game {
    numbers: Vec<i32>,
    grids: Vec<Vec<(i32, bool)>>,
}

fn part1(input_path: &str) -> Result<i32, Error> {
    const GRID_SIZE: usize = 5;
    let mut game = parse_input(input_path)?;

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

fn part2(input_path: &str) -> Result<i32, Error> {
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

fn score(grid: &[(i32, bool)]) -> i32 {
    grid.iter().filter(|x| !x.1).map(|x| x.0).sum()
}

fn won(grid: &[(i32, bool)], grid_size: usize) -> bool {
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

fn mark_grid(grid: &mut Vec<(i32, bool)>, number: &i32) {
    grid.iter_mut().for_each(|(i, flag)| {
        if *i == *number {
            *flag = true;
        }
    })
}

fn parse_numbers(line: &str) -> Result<Vec<i32>, ParseIntError> {
    line.split(',').map(|s| s.parse()).collect()
}

fn parse_grids(
    lines: &mut io::Lines<io::BufReader<File>>,
) -> Result<Vec<Vec<(i32, bool)>>, ParseIntError> {
    let mut grids: Vec<Vec<(i32, bool)>> = vec![];

    let mut grid: Vec<(i32, bool)> = vec![];
    for line in lines.flatten() {
        if line.is_empty() {
            grids.push(grid);
            grid = vec![]
        } else {
            grid.append(&mut parse_grid_line(&line)?)
        }
    }
    grids.push(grid);
    Ok(grids)
}

fn parse_grid_line(line: &str) -> Result<Vec<(i32, bool)>, ParseIntError> {
    line.split_whitespace()
        .map(|s| s.parse().map(|parsed| (parsed, false)))
        .collect()
}

fn parse_input(filename: &str) -> Result<Game, Error> {
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let numbers = parse_numbers(&lines.next().ok_or(Error::ParseFile)??)?;
    lines.next();

    let grids = parse_grids(&mut lines)?;

    Ok(Game { numbers, grids })
}
