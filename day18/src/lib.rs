#![feature(box_patterns)]
#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<i32, i32> for Solution {
    fn part1(&self) -> i32 {
        part1(&self.input_path)
    }
    fn part2(&self) -> i32 {
        part2(&self.input_path)
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

#[derive(Debug)]
enum Result<T> {
    Exploded(T),
    Split(T),
    NoOp,
}

fn reduce_repeat(numbers: &[i32]) -> Vec<i32> {
    let mut result = numbers.to_vec();

    loop {
        match reduce(&result) {
            Result::NoOp => return result,
            Result::Exploded(v) => result = v,
            Result::Split(v) => result = v,
        }
    }
}

fn reduce(str: &[i32]) -> Result<Vec<i32>> {
    let mut left_number_index: Option<usize> = None;
    let mut explode_number_index: Option<usize> = None;
    let mut split_number_index: Option<usize> = None;

    let mut depth: u8 = 0;

    for (i, v) in str.iter().enumerate() {
        match v {
            -1 => {
                depth += 1;
                if depth == 5 {
                    explode_number_index = Some(i);
                    break;
                }
            }
            -2 => depth -= 1,
            -3 => {}
            0..=9 => left_number_index = Some(i),
            10.. => {
                if split_number_index.is_none() {
                    split_number_index = Some(i);
                }
                left_number_index = Some(i);
            }
            _ => unreachable!(),
        }
    }

    if let Some(i) = explode_number_index {
        let mut result: Vec<i32> = Vec::from(&str[0..i]);
        result.push(0);
        result.append(&mut Vec::from(&str[i + 5..]));

        if let Some(l) = left_number_index {
            result[l] = str[l] + str[i + 1];
        }

        for c in result.iter_mut().skip(i + 1) {
            if let 0.. = c {
                *c += str[i + 3];
                break;
            }
        }

        return Result::Exploded(result);
    }

    if let Some(i) = split_number_index {
        let v = str[i];
        let new_c_1 = v / 2;
        let new_c_2 = (v + 1) / 2;
        let mut result: Vec<i32> = Vec::from(&str[0..i]);
        result.push(-1);
        result.push(new_c_1);
        result.push(-3);
        result.push(new_c_2);
        result.push(-2);
        result.append(&mut Vec::from(&str[i + 1..]));

        return Result::Split(result);
    }

    Result::NoOp
}

fn add(number_1: &[i32], number_2: &[i32]) -> Vec<i32> {
    let mut result = vec![-1];
    result.append(&mut Vec::from(number_1));
    result.push(-3);
    result.append(&mut Vec::from(number_2));
    result.push(-2);

    result
}

fn map_input(str: &str) -> Vec<i32> {
    str.chars()
        .map(|c| match c {
            '[' => -1,
            ']' => -2,
            ',' => -3,
            _ => c.to_digit(10).unwrap() as i32,
        })
        .collect()
}

fn magnitude(numbers: &[i32]) -> i32 {
    let mut comma_pos = 0;
    let mut depth: u8 = 0;

    for (i, v) in numbers.iter().enumerate() {
        match v {
            -1 => depth += 1,
            -2 => depth -= 1,
            -3 => {
                if depth == 1 {
                    comma_pos = i;
                    break;
                }
            }
            _ => {}
        }
    }

    let left = &numbers[1..comma_pos];
    let right = &numbers[comma_pos + 1..numbers.len() - 1];

    let left_val = if left.len() == 1 {
        left[0]
    } else {
        magnitude(left)
    };

    let right_val = if right.len() == 1 {
        right[0]
    } else {
        magnitude(right)
    };

    3 * left_val + 2 * right_val
}

fn part1(input_path: &str) -> i32 {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();
    let numbers_vec: Vec<Vec<i32>> = lines.map(|str| map_input(&str)).collect();

    let final_numbers = numbers_vec
        .iter()
        .skip(1)
        .fold(numbers_vec[0].clone(), |acc, numbers| {
            reduce_repeat(&add(&acc, numbers))
        });

    magnitude(&final_numbers)
}

fn part2(input_path: &str) -> i32 {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();
    let numbers_vec: Vec<Vec<i32>> = lines.map(|str| map_input(&str)).collect();

    let mut result: i32 = 0;

    for numbers1 in numbers_vec.iter() {
        for numbers2 in numbers_vec.iter() {
            if numbers1 != numbers2 {
                result = std::cmp::max(result, magnitude(&reduce_repeat(&add(numbers1, numbers2))));
            }
        }
    }

    result
}
