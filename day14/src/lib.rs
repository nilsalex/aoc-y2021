#![feature(test)]
mod bench;

use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<usize, usize> for Solution {
    fn part1(&self) -> usize {
        part1(&self.input_path)
    }
    fn part2(&self) -> usize {
        part2(&self.input_path)
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

struct Data {
    pairs: HashMap<(char, char), usize>,
    counts: HashMap<char, usize>,
    rules: HashMap<(char, char), char>,
}

fn part1(input_path: &str) -> usize {
    let Data {
        mut pairs,
        mut counts,
        rules,
    } = parse_input(input_path);

    for _ in 0..10 {
        step(&mut pairs, &mut counts, &rules);
    }

    let min_count = counts.iter().map(|p| p.1).min().unwrap();
    let max_count = counts.iter().map(|p| p.1).max().unwrap();

    max_count - min_count
}

fn part2(input_path: &str) -> usize {
    let Data {
        mut pairs,
        mut counts,
        rules,
    } = parse_input(input_path);

    for _ in 0..40 {
        step(&mut pairs, &mut counts, &rules);
    }

    let min_count = counts.iter().map(|p| p.1).min().unwrap();
    let max_count = counts.iter().map(|p| p.1).max().unwrap();

    max_count - min_count
}

fn parse_input(input_path: &str) -> Data {
    let file = File::open(input_path).unwrap();
    let mut lines = io::BufReader::new(file).lines().flatten();
    let polymer: Vec<char> = lines.next().unwrap().chars().collect();

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for i in 0..polymer.len() {
        if i < polymer.len() - 1 {
            let p = (polymer[i], polymer[i + 1]);
            insert_or_add(&mut pairs, p, 1);
        }

        insert_or_add(&mut counts, polymer[i], 1);
    }

    lines.next();

    let rules: HashMap<(char, char), char> = lines
        .map(|line| {
            let (fst, snd) = line.split_once(" -> ").unwrap();
            (
                (
                    fst.chars().collect::<Vec<char>>()[0],
                    fst.chars().collect::<Vec<char>>()[1],
                ),
                snd.chars().collect::<Vec<char>>()[0],
            )
        })
        .collect();

    Data {
        pairs,
        counts,
        rules,
    }
}

fn step(
    pairs: &mut HashMap<(char, char), usize>,
    counts: &mut HashMap<char, usize>,
    rules: &HashMap<(char, char), char>,
) {
    for (pair, count) in pairs.clone() {
        if let Some(new_char) = rules.get(&pair) {
            remove_or_subtract(pairs, &pair, &count);

            insert_or_add(pairs, (pair.0, *new_char), count);
            insert_or_add(pairs, (*new_char, pair.1), count);

            insert_or_add(counts, *new_char, count);
        }
    }
}

fn remove_or_subtract(
    pairs: &mut HashMap<(char, char), usize>,
    pair: &(char, char),
    count: &usize,
) {
    let old_count = pairs.get_mut(pair).unwrap();

    if *old_count == *count {
        pairs.remove(pair);
    } else {
        *old_count -= *count;
    }
}

fn insert_or_add<T, N>(pairs: &mut HashMap<T, N>, new_pair: T, new_count: N)
where
    T: Eq,
    T: Hash,
    N: std::ops::AddAssign,
{
    if let Some(old_count) = pairs.get_mut(&new_pair) {
        *old_count += new_count;
    } else {
        pairs.insert(new_pair, new_count);
    }
}
