pub const INPUT: &str = include_str!("input.txt");

fn score(closer: char) -> usize {
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

pub fn part1(s: &str) -> usize {
    s.lines()
        .filter_map(|line| match parse_line(line) {
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

pub fn part2(s: &str) -> usize {
    let mut scores: Vec<usize> = s
        .lines()
        .filter_map(|line| parse_line(line).ok().map(|stack| stack_score(&stack)))
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day10_part1() {
    assert_eq!(part1(INPUT), 168417);
}

#[test]
fn test_day10_part2() {
    assert_eq!(part2(INPUT), 2802519786);
}

#[bench]
fn bench_day10_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day10_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
