#![feature(test)]
mod bench;

use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution for Solution {
    fn part1(&self) -> String {
        part1(&self.input_path).to_string()
    }
    fn part2(&self) -> String {
        part2(&self.input_path).to_string()
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

enum FilterMode {
    MostCommon,
    LeastCommon,
}

pub fn part1(input_path: &str) -> i32 {
    const NUM_BITS: usize = 12;
    let mut bits: [i32; NUM_BITS] = [0; NUM_BITS];
    let mut count: i32 = 0;

    if let Ok(file) = File::open(input_path) {
        for line in io::BufReader::new(file).lines().flatten() {
            count += 1;
            for (i, c) in line.chars().enumerate() {
                if c == '1' {
                    bits[i] += 1
                }
            }
        }
    }

    let mut a: i32 = 0;
    let mut b: i32 = 0;

    for (i, bit) in bits.iter().enumerate() {
        let x = 1 << (NUM_BITS - i - 1);
        if bit > &(&count / 2) {
            a += x
        } else {
            b += x
        }
    }

    a * b
}

pub fn part2(input_path: &str) -> i32 {
    const NUM_BITS: usize = 12;

    let mut numbers: Vec<Vec<i32>> = vec![];

    if let Ok(file) = File::open(input_path) {
        for line in io::BufReader::new(file).lines().flatten() {
            let mut number: Vec<i32> = vec![];
            number.reserve(NUM_BITS);
            for c in line.chars() {
                match c {
                    '0' => number.push(0),
                    '1' => number.push(1),
                    _ => {
                        panic!()
                    }
                }
            }
            numbers.push(number)
        }
    }

    let a = filter(numbers.to_vec(), 0, FilterMode::MostCommon);
    let b = filter(numbers.to_vec(), 0, FilterMode::LeastCommon);

    to_int(a, NUM_BITS) * to_int(b, NUM_BITS)
}

fn filter(numbers: Vec<Vec<i32>>, bit: usize, filter_mode: FilterMode) -> Vec<i32> {
    let mut count_zeros = 0;
    let mut count_ones = 0;

    for number in &numbers {
        match number[bit] {
            0 => count_zeros += 1,
            1 => count_ones += 1,
            _ => {
                panic!()
            }
        }
    }

    let mut filtered: Vec<Vec<i32>> = vec![];

    let number_to_take: i32 = match filter_mode {
        FilterMode::LeastCommon => {
            if count_zeros <= count_ones {
                0
            } else {
                1
            }
        }
        FilterMode::MostCommon => {
            if count_zeros <= count_ones {
                1
            } else {
                0
            }
        }
    };

    for number in &numbers {
        if number[bit] == number_to_take {
            filtered.push(number.to_vec())
        }
    }

    match filtered.len() {
        0 => panic!(),
        1 => filtered[0].to_vec(),
        _ => filter(filtered, bit + 1, filter_mode),
    }
}

fn to_int(number: Vec<i32>, num_bits: usize) -> i32 {
    let mut result = 0;
    for (i, bit) in number.iter().enumerate() {
        let x = 1 << (num_bits - i - 1);
        if *bit != 0 {
            result += x
        }
    }
    result
}
