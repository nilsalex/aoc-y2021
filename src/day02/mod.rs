pub fn input() -> &'static str {
    include_str!("input.txt")
}

pub fn part1(s: &str) -> usize {
    let mut horizontal: usize = 0;
    let mut vertical: usize = 0;

    for line in s.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        if let Ok(parsed) = split[1].parse::<usize>() {
            match split[0] {
                "forward" => horizontal += parsed,
                "up" => vertical -= parsed,
                "down" => vertical += parsed,
                _ => {}
            }
        }
    }

    horizontal * vertical
}

pub fn part2(s: &str) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for line in s.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        if let Ok(parsed) = split[1].parse::<usize>() {
            match split[0] {
                "forward" => {
                    horizontal += parsed;
                    depth += aim * parsed
                }
                "up" => aim -= parsed,
                "down" => aim += parsed,
                _ => {}
            }
        }
    }

    horizontal * depth
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day02_part1() {
    assert_eq!(part1(input()), 1660158);
}

#[test]
fn test_day02_part2() {
    assert_eq!(part2(input()), 1604592846);
}

#[bench]
fn bench_day02_part1(b: &mut Bencher) {
    b.iter(|| part1(input()))
}

#[bench]
fn bench_day02_part2(b: &mut Bencher) {
    b.iter(|| part2(input()))
}
