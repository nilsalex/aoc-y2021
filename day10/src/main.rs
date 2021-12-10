use std::fs::File;
use std::io::{self, BufRead};

fn score(closer: char) -> i32 {
    match closer {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn match_opener(closer: char) -> char {
    match closer {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

fn part1() -> i32 {
    const INPUT_FILE: &str = "day10/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut final_score: i32 = 0;

    for line in lines {
        let mut open_stack: Vec<char> = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => open_stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if open_stack.pop().unwrap() != match_opener(c) {
                        final_score += score(c);
                        break;
                    }
                }
                _ => panic!(),
            }
        }
    }

    final_score
}

fn part2() -> usize {
    const INPUT_FILE: &str = "day10/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut scores: Vec<usize> = vec![];

    'outer: for line in lines {
        let mut open_stack: Vec<char> = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => open_stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if open_stack.pop().unwrap() != match_opener(c) {
                        continue 'outer;
                    }
                }
                _ => panic!(),
            }
        }

        let mut score: usize = 0;

        for c in open_stack.iter().rev() {
            match c {
                '(' => score = score * 5 + 1,
                '[' => score = score * 5 + 2,
                '{' => score = score * 5 + 3,
                '<' => score = score * 5 + 4,
                _ => panic!(),
            }
        }

        scores.push(score);
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
