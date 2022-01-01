use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");

pub fn part1(s: &str) -> usize {
    let mut map_: Vec<Vec<usize>> = s
        .lines()
        .map(|l| {
            let mut inner: Vec<usize> = vec![9];
            let inner2 = l.chars().map(|c| c.to_digit(10).unwrap() as usize);
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
    let mut minima: Vec<usize> = vec![];

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

pub fn part2(s: &str) -> usize {
    let mut map_: Vec<Vec<i32>> = s
        .lines()
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

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day09_part1() {
    assert_eq!(part1(INPUT), 452);
}

#[test]
fn test_day09_part2() {
    assert_eq!(part2(INPUT), 1263735);
}

#[bench]
fn bench_day09_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day09_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
