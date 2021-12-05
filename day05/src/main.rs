use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "day05/input.txt";

struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Line {
    fn is_non_diagonal(&self) -> bool {
        self.start_x == self.end_x || self.start_y == self.end_y
    }

    fn slope_and_count(&self) -> (i32, i32, i32) {
        let dx_ = self.end_x - self.start_x;
        let dy_ = self.end_y - self.start_y;

        if dx_ == 0 {
            (0, num::signum(dy_), num::abs(dy_))
        } else if dy_ == 0 {
            (num::signum(dx_), 0, num::abs(dx_))
        } else {
            let gcd = num::integer::gcd(num::abs(dx_), num::abs(dy_));
            (dx_ / gcd, dy_ / gcd, gcd)
        }
    }
}

fn part1() -> usize {
    let lines = parse_input(INPUT_FILE);
    let non_diagonal: Vec<Line> = lines.filter(|l| l.is_non_diagonal()).collect();

    let mut points_once: HashSet<(i32, i32)> = HashSet::new();
    let mut points_twice: HashSet<(i32, i32)> = HashSet::new();

    for line in non_diagonal {
        let (dx, dy, c) = line.slope_and_count();
        for i in 0..=c {
            let point = (line.start_x + i * dx, line.start_y + i * dy);
            if points_once.contains(&point) {
                points_twice.insert(point);
            } else {
                points_once.insert(point);
            }
        }
    }

    points_twice.len()
}

fn part2() -> usize {
    let lines: Vec<Line> = parse_input(INPUT_FILE).collect();

    let mut points_once: HashSet<(i32, i32)> = HashSet::new();
    let mut points_twice: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let (dx, dy, c) = line.slope_and_count();
        for i in 0..=c {
            let point = (line.start_x + i * dx, line.start_y + i * dy);
            if points_once.contains(&point) {
                points_twice.insert(point);
            } else {
                points_once.insert(point);
            }
        }
    }

    points_twice.len()
}

fn parse_input(filename: &str) -> impl Iterator<Item = Line> {
    let file = File::open(filename).unwrap();
    let input_lines = io::BufReader::new(file).lines().flatten();
    input_lines.map(|s| parse_line(&s))
}

fn parse_line(input_line: &str) -> Line {
    let points: Vec<&str> = input_line.split(" -> ").collect();
    let start: Vec<i32> = points[0].split(',').map(|s| s.parse()).flatten().collect();
    let end: Vec<i32> = points[1].split(',').map(|s| s.parse()).flatten().collect();

    Line {
        start_x: start[0],
        start_y: start[1],
        end_x: end[0],
        end_y: end[1],
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
