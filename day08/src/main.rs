use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn count_unique_in_line(line: &str) -> usize {
    let right = line.split(" | ").collect::<Vec<&str>>()[1];
    right
        .split_whitespace()
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

fn solve_line(line: &str) -> usize {
    let split = line.split(" | ").collect::<Vec<&str>>();
    let mut input = split[0]
        .split_whitespace()
        .map(|s| {
            let mut chars = s.chars().map(|c| c as u8 - 97).collect::<Vec<u8>>();
            chars.sort_unstable();
            chars
        })
        .collect::<Vec<Vec<u8>>>();
    input.sort_unstable();

    let items: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
    let numbers: Vec<Vec<u8>> = vec![
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];

    let mut solution_perm: Vec<u8> = vec![];

    for perm in items.into_iter().permutations(items.len()) {
        let mut numbers_permuted = numbers.clone();

        for number in numbers_permuted.iter_mut() {
            for bar in number.iter_mut() {
                *bar = perm[*bar as usize];
            }
            number.sort_unstable();
        }

        numbers_permuted.sort();

        if numbers_permuted == input {
            solution_perm = perm.clone();
        }
    }

    let mut solution_numbers: Vec<Vec<u8>> = numbers.into_iter().collect();
    for number in solution_numbers.iter_mut() {
        for bar in number.iter_mut() {
            *bar = solution_perm[*bar as usize];
        }
        number.sort_unstable();
    }

    let result_digits = split[1]
        .split_whitespace()
        .map(|s| {
            let mut chars = s.chars().map(|c| c as u8 - 97).collect::<Vec<u8>>();
            chars.sort_unstable();
            for (number, pattern) in solution_numbers.iter().enumerate() {
                if chars == *pattern {
                    return number as i32;
                }
            }
            panic!();
        })
        .collect::<Vec<i32>>();

    let mut result: i32 = 0;

    for (i, digit) in result_digits.iter().rev().enumerate() {
        result += digit * i32::pow(10, i as u32);
    }

    result as usize
}

fn part1() -> usize {
    const INPUT_FILE: &str = "day08/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    lines.map(|s| count_unique_in_line(&s)).sum()
}

fn part2() -> usize {
    const INPUT_FILE: &str = "day08/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    lines.map(|s| solve_line(&s)).sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
