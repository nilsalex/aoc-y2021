use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");

fn count_unique_in_line(line: &str) -> usize {
    let right = line.split(" | ").collect::<Vec<&str>>()[1];
    right
        .split_whitespace()
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

fn solve_line(line: &str) -> usize {
    let (input, output) = line.split_once(" | ").unwrap();

    let ref_numbers: HashMap<u8, HashSet<u8>> =
        HashMap::from_iter(input.split_whitespace().filter_map(|s| {
            let bars = HashSet::from_iter(s.chars().map(|c| c as u8 - 97));
            match bars.len() {
                2 => Some((1, bars)),
                3 => Some((7, bars)),
                4 => Some((4, bars)),
                7 => Some((8, bars)),
                _ => None,
            }
        }));

    let sol_numbers = output
        .split_whitespace()
        .map(|s| HashSet::from_iter(s.chars().map(|c| c as u8 - 97)));

    let digits = sol_numbers.map(|number| match number.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => match (
            number.intersection(ref_numbers.get(&4).unwrap()).count(),
            number.intersection(ref_numbers.get(&7).unwrap()).count(),
            number.intersection(ref_numbers.get(&8).unwrap()).count(),
        ) {
            (3, 3, 6) => 0,
            (2, 2, 5) => 2,
            (3, 3, 5) => 3,
            (3, 2, 5) => 5,
            (3, 2, 6) => 6,
            (4, 3, 6) => 9,
            _ => unreachable!(),
        },
    });

    let mut result: usize = 0;

    for (i, digit) in digits.rev().enumerate() {
        result += digit * usize::pow(10, i as u32);
    }

    result as usize
}

pub fn part1(s: &str) -> usize {
    s.lines().map(count_unique_in_line).sum()
}

pub fn part2(s: &str) -> usize {
    s.lines().map(solve_line).sum()
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day08_part1() {
    assert_eq!(part1(INPUT), 278);
}

#[test]
fn test_day08_part2() {
    assert_eq!(part2(INPUT), 986179);
}

#[bench]
fn bench_day08_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day08_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
