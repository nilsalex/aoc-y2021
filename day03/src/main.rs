use std::fs::File;
use std::io::{self, BufRead};

enum FilterMode {
    MostCommon,
    LeastCommon,
}

const NUM_BITS: usize = 12;

fn part1() -> i32 {
    let mut bits: [i32; NUM_BITS] = [0; NUM_BITS];
    let mut count: i32 = 0;

    if let Ok(file) = File::open("input.txt") {
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

fn part2() -> i32 {
    let mut numbers: Vec<Vec<i32>> = vec![];

    if let Ok(file) = File::open("input.txt") {
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

    to_int(a) * to_int(b)
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

fn to_int(number: Vec<i32>) -> i32 {
    let mut result = 0;
    for (i, bit) in number.iter().enumerate() {
        let x = 1 << (NUM_BITS - i - 1);
        if *bit != 0 {
            result += x
        }
    }
    result
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
