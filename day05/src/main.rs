use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

const INPUT_FILE: &str = "day05/input.txt";

fn part1() -> i32 {
    let file = File::open(INPUT_FILE).unwrap();

    let input_lines = io::BufReader::new(file).lines().flatten();

    let lines = input_lines.map(|s| parse_line(&s));

    let non_diagonal: Vec<Line> = lines.filter(is_non_diagonal).collect();

    let (min_x, min_y, max_x, max_y) = get_bounds(&non_diagonal);

    let mut number_of_intersections = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let intersection_count = non_diagonal.iter().filter(|l| is_point_of(x, y, l)).count();
            if intersection_count > 1 {
                number_of_intersections += 1;
            }
        }
    }

    number_of_intersections
}

fn part2() -> i32 {
    let file = File::open(INPUT_FILE).unwrap();

    let input_lines = io::BufReader::new(file).lines().flatten();

    let lines: Vec<Line> = input_lines.map(|s| parse_line(&s)).collect();

    let (min_x, min_y, max_x, max_y) = get_bounds(&lines);

    let mut number_of_intersections = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let intersection_count = lines.iter().filter(|l| is_point_of(x, y, l)).count();
            if intersection_count > 1 {
                number_of_intersections += 1;
            }
        }
    }

    number_of_intersections
}

fn is_point_of(x: i32, y: i32, line: &Line) -> bool {
    let dx_ = line.end_x - line.start_x;
    let dy_ = line.end_y - line.start_y;

    let x_rel = x - line.start_x;
    let y_rel = y - line.start_y;

    if x_rel == 0 && y_rel == 0 {
        return true;
    }

    let (dx1, dy1, c1) = if dx_ == 0 {
        (0, num::signum(dy_), num::abs(dy_))
    } else if dy_ == 0 {
        (num::signum(dx_), 0, num::abs(dx_))
    } else {
        let gcd = num::integer::gcd(num::abs(dx_), num::abs(dy_));
        (dx_ / gcd, dy_ / gcd, gcd)
    };

    let (dx2, dy2, c2) = if x_rel == 0 {
        (0, num::signum(y_rel), num::abs(y_rel))
    } else if y_rel == 0 {
        (num::signum(x_rel), 0, num::abs(x_rel))
    } else {
        let gcd = num::integer::gcd(num::abs(x_rel), num::abs(y_rel));
        (x_rel / gcd, y_rel / gcd, gcd)
    };

    dx1 == dx2 && dy1 == dy2 && 0 <= c2 && c2 <= c1
}

fn get_bounds(lines: &[Line]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for line in lines {
        min_x = std::cmp::min(std::cmp::min(min_x, line.start_x), line.end_x);
        max_x = std::cmp::max(std::cmp::max(max_x, line.end_x), line.start_x);
        min_y = std::cmp::min(std::cmp::min(min_y, line.start_y), line.end_y);
        max_y = std::cmp::max(std::cmp::max(max_y, line.end_y), line.start_y);
    }

    (min_x, min_y, max_x, max_y)
}

fn is_non_diagonal(line: &Line) -> bool {
    line.start_x == line.end_x || line.start_y == line.end_y
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
