use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

struct Image {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    points: HashSet<(i32, i32)>,
    default: u8,
}

impl Image {
    fn light_points(&self) -> usize {
        self.points.len()
    }

    fn enhance(&mut self, algo: &[u8]) {
        let mut new_points: HashSet<(i32, i32)> = HashSet::new();

        for x in self.min_x - 1..=self.max_x + 1 {
            for y in self.min_y - 1..=self.max_y + 1 {
                let number_bin = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ]
                .map(|(x_, y_)| {
                    if self.points.contains(&(x_, y_)) {
                        1
                    } else if x_ >= self.min_x
                        && x_ <= self.max_x
                        && y_ >= self.min_y
                        && y_ <= self.max_y
                    {
                        0
                    } else {
                        self.default
                    }
                });
                let number = number_bin
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (i, v)| acc + (*v as usize) * (1 << i));
                if algo[number] == 1 {
                    new_points.insert((x, y));
                }
            }
        }

        self.default = if self.default == 0 {
            algo[0]
        } else {
            algo[511]
        };
        self.points = new_points;
        self.min_x -= 1;
        self.max_x += 1;
        self.min_y -= 1;
        self.max_y += 1;
    }
}

fn parse_input() -> (Image, Vec<u8>) {
    let file = File::open("day20/input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().flatten();

    let algo: Vec<u8> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => unreachable!(),
        })
        .collect();

    lines.next();

    let mut points: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert((x as i32, y as i32));
            }
        }
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (x, y) in points.iter() {
        min_x = std::cmp::min(min_x, *x);
        max_x = std::cmp::max(max_x, *x);
        min_y = std::cmp::min(min_x, *y);
        max_y = std::cmp::max(max_x, *y);
    }

    (
        Image {
            min_x,
            max_x,
            min_y,
            max_y,
            points,
            default: 0,
        },
        algo,
    )
}

fn part1() -> usize {
    let (mut image, algo) = parse_input();

    image.enhance(&algo);
    image.enhance(&algo);

    image.light_points()
}

fn part2() -> usize {
    let (mut image, algo) = parse_input();

    for _ in 0..50 {
        image.enhance(&algo);
    }

    image.light_points()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
