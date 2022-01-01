use std::collections::{HashSet, VecDeque};

pub const INPUT: &str = include_str!("input.txt");

pub fn part1(s: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = s
        .lines()
        .map(|line| {
            line.chars()
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

pub fn part2(s: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = s
        .lines()
        .map(|line| {
            line.chars()
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

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day11_part1() {
    assert_eq!(part1(INPUT), 1655);
}

#[test]
fn test_day11_part2() {
    assert_eq!(part2(INPUT), 337);
}

#[bench]
fn bench_day11_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day11_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
