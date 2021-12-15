#![feature(test)]
mod bench;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<i32, usize> for Solution {
    fn part1(&self) -> i32 {
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

fn part1(input_path: &str) -> i32 {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();
    let mut map_: Vec<Vec<i32>> = lines
        .map(|l| {
            let mut inner: Vec<i32> = vec![9];
            let inner2 = l.chars().map(|c| c.to_digit(10).unwrap() as i32);
            inner.extend(inner2);
            inner.push(9);
            inner
        })
        .collect();

    let x_dim = map_[0].len();
    let mut map = vec![vec![9; x_dim]];
    map.append(&mut map_);
    map.push(vec![9; x_dim]);
    let y_dim = map.len();
    let mut minima: Vec<i32> = vec![];

    for y in 1..y_dim - 1 {
        for x in 1..x_dim - 1 {
            let m = map[y][x];
            if m < map[y - 1][x] && m < map[y + 1][x] && m < map[y][x - 1] && m < map[y][x + 1] {
                minima.push(m + 1);
            }
        }
    }

    minima.iter().sum()
}

fn part2(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();
    let mut map_: Vec<Vec<i32>> = lines
        .map(|l| {
            let mut inner: Vec<i32> = vec![9];
            let inner2 = l.chars().map(|c| c.to_digit(10).unwrap() as i32);
            inner.extend(inner2);
            inner.push(9);
            inner
        })
        .collect();

    let x_dim = map_[0].len();
    let mut map = vec![vec![9; x_dim]];
    map.append(&mut map_);
    map.push(vec![9; x_dim]);
    let y_dim = map.len();
    let mut basins: Vec<usize> = vec![];

    for y in 1..y_dim - 1 {
        for x in 1..x_dim - 1 {
            let m = map[y][x];
            if m < map[y - 1][x] && m < map[y + 1][x] && m < map[y][x - 1] && m < map[y][x + 1] {
                let mut seen = HashSet::new();
                explore_basin(&map, x, y, &mut seen);
                basins.push(seen.len());
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));

    basins[0] * basins[1] * basins[2]
}

fn explore_basin(map: &[Vec<i32>], x: usize, y: usize, seen: &mut HashSet<(usize, usize)>) {
    seen.insert((x, y));
    let directions: Vec<(usize, usize)> = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|(x_, y_)| !seen.contains(&(*x_, *y_)) && map[*y_][*x_] < 9)
        .collect();

    for (x_, y_) in directions {
        explore_basin(map, x_, y_, seen);
    }
}
