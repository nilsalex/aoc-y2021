use std::fs::File;
use std::io::{self, BufRead};

fn move_right(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::from(grid);

    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '>' && grid[r][(c + 1) % cols] == '.' {
                result[r][c] = '.';
                result[r][(c + 1) % cols] = '>';
            }
        }
    }

    result
}

fn move_down(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::from(grid);

    let rows = grid.len();
    let cols = grid[0].len();

    for c in 0..cols {
        for r in 0..rows {
            if grid[r][c] == 'v' && grid[(r + 1) % rows][c] == '.' {
                result[r][c] = '.';
                result[(r + 1) % rows][c] = 'v';
            }
        }
    }

    result
}

fn part1() -> usize {
    let file = File::open("day25/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect::<Vec<char>>()).collect();

    let mut steps = 0;

    loop {
        let next_grid = move_down(&move_right(&grid));

        steps += 1;

        if next_grid == grid {
            break;
        }

        grid = next_grid;
    }

    steps
}

fn main() {
    println!("{}", part1());
}
