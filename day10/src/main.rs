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

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut open_stack: Vec<char> = vec![];

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => open_stack.push(c),
            ')' | ']' | '}' | '>' => {
                if open_stack.pop().unwrap() != match_opener(c) {
                    return Err(c);
                }
            }
            _ => panic!(),
        }
    }

    Ok(open_stack)
}

fn part1() -> i32 {
    const INPUT_FILE: &str = "day10/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    lines
        .filter_map(|line| match parse_line(&line) {
            Ok(_) => None,
            Err(c) => Some(score(c)),
        })
        .sum()
}

fn stack_score(stack: &[char]) -> usize {
    stack.iter().rev().fold(0_usize, |acc, c| match c {
        '(' => acc * 5 + 1,
        '[' => acc * 5 + 2,
        '{' => acc * 5 + 3,
        '<' => acc * 5 + 4,
        _ => panic!(),
    })
}

fn part2() -> usize {
    const INPUT_FILE: &str = "day10/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut scores: Vec<usize> = lines
        .filter_map(|line| parse_line(&line).ok().map(|stack| stack_score(&stack)))
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
