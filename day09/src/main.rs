use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> i32 {
    const INPUT_FILE: &str = "day09/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
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

fn part2() -> usize {
    const INPUT_FILE: &str = "day09/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
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

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
