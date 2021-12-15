use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct State {
    risk: u32,
    position: (usize, usize),
}

fn parse_input() -> Vec<Vec<u32>> {
    const INPUT_FILE: &str = "day15/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn part1() -> u32 {
    let grid = parse_input();

    let xdim = grid[0].len();
    let ydim = grid.len();

    // we need a starting point and an end point
    let start = (0, 0);
    let goal = (xdim - 1, ydim - 1);

    // initialize dist and heap
    let mut dist: Vec<Vec<u32>> = grid
        .iter()
        .map(|v| v.iter().map(|_| u32::MAX).collect())
        .collect();

    let mut heap = BinaryHeap::new();

    // set up dist and heap for starting point
    dist[start.1][start.0] = 0;

    heap.push(Reverse(State {
        risk: 0,
        position: start,
    }));

    while let Some(Reverse(State { risk, position })) = heap.pop() {
        if position == goal {
            return risk;
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
                heap.push(Reverse(next));
                dist[next.position.1][next.position.0] = next.risk;
            }
        }
    }

    panic!();
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

fn get_risk(grid: &[Vec<u32>], xdim: usize, ydim: usize, x: usize, y: usize) -> u32 {
    (((grid[y % ydim][x % xdim] + (y / ydim) as u32 + (x / xdim) as u32) - 1) % 9) + 1
}

fn part2() -> u32 {
    let grid = parse_input();

    let xdim = grid[0].len();
    let ydim = grid.len();

    let start = (0, 0);
    let goal = (5 * xdim - 1, 5 * ydim - 1);

    let mut dist: Vec<Vec<u32>> = (0..5 * ydim)
        .map(|_| (0..5 * xdim).map(|_| u32::MAX).collect())
        .collect();
    let mut heap = BinaryHeap::new();

    dist[start.1][start.0] = 0;
    heap.push(Reverse(State {
        risk: 0,
        position: start,
    }));

    while let Some(Reverse(State { risk, position })) = heap.pop() {
        if position == goal {
            return risk;
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
                heap.push(Reverse(next));
                dist[next.position.1][next.position.0] = next.risk;
            }
        }
    }

    panic!();
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
