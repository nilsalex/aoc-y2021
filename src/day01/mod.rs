pub fn input() -> &'static str {
    include_str!("input.txt")
}

pub fn part1(s: &str) -> usize {
    s.lines()
        .filter_map(|l| l.parse::<usize>().ok())
        .fold((0, usize::MAX), |(acc, prev), x| {
            (if x > prev { acc + 1 } else { acc }, x)
        })
        .0
}

pub fn part2(s: &str) -> usize {
    let mut prev1: usize = 0;
    let mut prev2: usize = 0;
    let mut prev3: usize = 0;
    let mut counter: usize = 0;

    for (i, line) in s.lines().enumerate() {
        if let Ok(parsed) = line.parse::<usize>() {
            match i {
                0 => prev3 = parsed,
                1 => prev2 = parsed,
                2 => prev1 = parsed,
                _ => {
                    let old_sum = prev3 + prev2 + prev1;
                    let new_sum = prev2 + prev1 + parsed;
                    if new_sum > old_sum {
                        counter += 1;
                    }
                    prev3 = prev2;
                    prev2 = prev1;
                    prev1 = parsed;
                }
            }
        }
    }

    counter
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day01_part1() {
    assert_eq!(part1(input()), 1466);
}

#[test]
fn test_day01_part2() {
    assert_eq!(part2(input()), 1491);
}

#[bench]
fn bench_day01_part1(b: &mut Bencher) {
    b.iter(|| part1(input()))
}

#[bench]
fn bench_day01_part2(b: &mut Bencher) {
    b.iter(|| part2(input()))
}
