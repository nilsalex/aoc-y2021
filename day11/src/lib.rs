#![feature(test)]
mod bench;

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
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

fn part1(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let mut grid: Vec<Vec<usize>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut flash_counter: usize = 0;

    for _ in 0..100 {
        let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
        let mut glowing: HashSet<(isize, isize)> = HashSet::new();

        for (y, line) in grid.iter_mut().enumerate() {
            for (x, val) in line.iter_mut().enumerate() {
                if *val == 9 {
                    queue.push_back((x as isize, y as isize));
                    glowing.insert((x as isize, y as isize));
                }
                *val += 1;
            }
        }

        while !queue.is_empty() {
            let (x_0, y_0) = queue.pop_front().unwrap();
            let neighbours = [
                (x_0 - 1, y_0 - 1),
                (x_0 - 1, y_0),
                (x_0 - 1, y_0 + 1),
                (x_0, y_0 - 1),
                (x_0, y_0),
                (x_0, y_0 + 1),
                (x_0 + 1, y_0 - 1),
                (x_0 + 1, y_0),
                (x_0 + 1, y_0 + 1),
            ];

            let filtered: Vec<(isize, isize)> = neighbours
                .into_iter()
                .filter(|(x, y)| *x >= 0 && *x < 10 && *y >= 0 && *y < 10)
                .filter(|(x, y)| !glowing.contains(&(*x, *y)))
                .collect();

            for (x, y) in filtered {
                let val: &mut usize = &mut grid[y as usize][x as usize];
                if *val < 9 {
                    *val += 1;
                } else {
                    queue.push_back((x, y));
                    glowing.insert((x, y));
                }
            }
        }

        for (x, y) in glowing.iter() {
            grid[*y as usize][*x as usize] = 0;
        }

        flash_counter += glowing.len();
    }

    flash_counter
}

fn part2(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let mut grid: Vec<Vec<usize>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut iteration: usize = 0;
    loop {
        iteration += 1;

        let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
        let mut glowing: HashSet<(isize, isize)> = HashSet::new();

        for (y, line) in grid.iter_mut().enumerate() {
            for (x, val) in line.iter_mut().enumerate() {
                if *val == 9 {
                    queue.push_back((x as isize, y as isize));
                    glowing.insert((x as isize, y as isize));
                }
                *val += 1;
            }
        }

        while !queue.is_empty() {
            let (x_0, y_0) = queue.pop_front().unwrap();
            let neighbours = [
                (x_0 - 1, y_0 - 1),
                (x_0 - 1, y_0),
                (x_0 - 1, y_0 + 1),
                (x_0, y_0 - 1),
                (x_0, y_0),
                (x_0, y_0 + 1),
                (x_0 + 1, y_0 - 1),
                (x_0 + 1, y_0),
                (x_0 + 1, y_0 + 1),
            ];

            let filtered: Vec<(isize, isize)> = neighbours
                .into_iter()
                .filter(|(x, y)| *x >= 0 && *x < 10 && *y >= 0 && *y < 10)
                .filter(|(x, y)| !glowing.contains(&(*x, *y)))
                .collect();

            for (x, y) in filtered {
                let val: &mut usize = &mut grid[y as usize][x as usize];
                if *val < 9 {
                    *val += 1;
                } else {
                    queue.push_back((x, y));
                    glowing.insert((x, y));
                }
            }
        }

        for (x, y) in glowing.iter() {
            grid[*y as usize][*x as usize] = 0;
        }

        if glowing.len() == 100 {
            return iteration;
        }
    }
}
