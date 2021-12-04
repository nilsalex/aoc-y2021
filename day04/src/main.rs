use std::fs::File;
use std::io::{self, BufRead};

const GRID_SIZE: usize = 5;

fn part1() -> i32 {
    let file = File::open("input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let numbers = parse_numbers(&lines.next().unwrap().unwrap());
    lines.next();

    let mut grids = parse_grids(&mut lines);

    for number in numbers {
        for grid in grids.iter_mut() {
            mark_grid(grid, number);

            if won(grid, GRID_SIZE) {
                return number * score(grid);
            }
        }
    }

    panic!()
}

fn score(grid: &[(i32, bool)]) -> i32 {
    let mut sum = 0;

    for (i, flag) in grid {
        if !flag {
            sum += i;
        }
    }

    sum
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

fn mark_grid(grid: &mut Vec<(i32, bool)>, number: i32) {
    for (i, flag) in grid {
        if *i == number {
            *flag = true;
        }
    }
}

fn parse_numbers(line: &str) -> Vec<i32> {
    line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn parse_grids(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<Vec<(i32, bool)>> {
    let mut grids: Vec<Vec<(i32, bool)>> = vec![];

    let mut grid: Vec<(i32, bool)> = vec![];
    for line in lines.flatten() {
        if line.is_empty() {
            grids.push(grid);
            grid = vec![]
        } else {
            grid.append(&mut parse_grid_line(&line))
        }
    }
    grids.push(grid);
    grids
}

fn parse_grid_line(line: &str) -> Vec<(i32, bool)> {
    line.split_whitespace()
        .map(|s| (s.parse().unwrap(), false))
        .collect()
}

fn main() {
    println!("{}", part1());
}
