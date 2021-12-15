use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    risk: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1() -> usize {
    const INPUT_FILE: &str = "day15/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let grid: Vec<Vec<usize>> = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut dist: Vec<Vec<usize>> = grid
        .iter()
        .map(|v| v.iter().map(|_| usize::MAX).collect())
        .collect();

    let mut heap = BinaryHeap::new();

    let xdim = grid[0].len();
    let ydim = grid.len();

    let start = (0, 0);
    let goal = (xdim - 1, ydim - 1);

    dist[start.1][start.0] = 0;
    heap.push(State {
        risk: 0,
        position: start,
    });

    let mut result = usize::MAX;

    while let Some(State { risk, position }) = heap.pop() {
        if position == goal {
            result = risk;
            break;
        }

        if risk > dist[position.1][position.0] {
            continue;
        }

        for position_ in next_positions(position, xdim, ydim) {
            let next = State {
                risk: risk + grid[position_.1][position_.0],
                position: position_,
            };

            if next.risk < dist[next.position.1][next.position.0] {
                heap.push(next);
                dist[next.position.1][next.position.0] = next.risk;
            }
        }
    }

    result
}

fn next_positions(position: (usize, usize), xdim: usize, ydim: usize) -> Vec<(usize, usize)> {
    let x = position.0 as isize;
    let y = position.1 as isize;

    let positions = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

    positions
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < xdim as isize && *y >= 0 && *y < ydim as isize)
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
}

fn get_risk(grid: &[Vec<usize>], xdim: usize, ydim: usize, x: usize, y: usize) -> usize {
    (((grid[y % ydim][x % xdim] + (y / ydim) + (x / xdim)) - 1) % 9) + 1
}

fn part2() -> usize {
    const INPUT_FILE: &str = "day15/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let grid: Vec<Vec<usize>> = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut heap = BinaryHeap::new();

    let xdim = grid[0].len();
    let ydim = grid.len();

    let mut dist: Vec<Vec<usize>> = (0..5 * ydim)
        .map(|_| (0..5 * xdim).map(|_| usize::MAX).collect())
        .collect();

    let start = (0, 0);
    let goal = (5 * xdim - 1, 5 * ydim - 1);

    dist[start.1][start.0] = 0;
    heap.push(State {
        risk: 0,
        position: start,
    });

    let mut result = usize::MAX;

    while let Some(State { risk, position }) = heap.pop() {
        if position == goal {
            result = risk;
            break;
        }

        if risk > dist[position.1][position.0] {
            continue;
        }

        for position_ in next_positions(position, 5 * xdim, 5 * ydim) {
            let next = State {
                risk: risk + get_risk(&grid, xdim, ydim, position_.0, position_.1),
                position: position_,
            };

            if next.risk < dist[next.position.1][next.position.0] {
                heap.push(next);
                dist[next.position.1][next.position.0] = next.risk;
            }
        }
    }

    result
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
