use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> usize {
    const INPUT_FILE: &str = "day13/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    0
}

fn parse_point(line: &str) -> (u32, u32) {
    let mut iter = line.split(',').map(|number| number.parse::<u32>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn parse_folding(line: &str) -> (char, u32) {
    let mut iter = line.split('=');
    (
        iter.next().unwrap().chars().last().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn part2() -> String {
    const INPUT_FILE: &str = "day13/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let mut lines = io::BufReader::new(file).lines().flatten();

    let mut points: HashSet<(u32, u32)> = HashSet::new();
    let mut line = lines.next().unwrap();
    while !line.is_empty() {
        let point = parse_point(&line);
        points.insert(point);
        line = lines.next().unwrap();
    }

    let mut foldings: Vec<(char, u32)> = vec![];
    for line_ in lines {
        let folding = parse_folding(&line_);
        foldings.push(folding);
    }

    foldings.iter().for_each(|(dir, pos)| {
        points.clone().iter().for_each(|(x, y)| match dir {
            'x' => {
                if *x == *pos {
                    points.remove(&(*x, *y));
                } else if *x > *pos {
                    points.remove(&(*x, *y));
                    points.insert((2 * (*pos) - *x, *y));
                }
            }
            'y' => {
                if *y == *pos {
                    points.remove(&(*x, *y));
                } else if *y > *pos {
                    points.remove(&(*x, *y));
                    points.insert((*x, 2 * (*pos) - *y));
                }
            }
            _ => {}
        })
    });

    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    let mut result = String::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                result.push('\u{2593}');
            } else {
                result.push('\u{2591}');
            }
        }
        result.push('\n');
    }

    result
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
