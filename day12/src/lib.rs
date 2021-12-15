#![feature(test)]
mod bench;

use std::collections::VecDeque;
use std::fs::File;
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

fn part1(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut edges: Vec<(String, String)> = vec![];
    let mut vertices: Vec<String> = vec![];

    lines.for_each(|line| {
        let split = line.split('-').collect::<Vec<&str>>();
        edges.push((split[0].to_owned(), split[1].to_owned()));
        edges.push((split[1].to_owned(), split[0].to_owned()));
        vertices.push(split[0].to_owned());
        vertices.push(split[1].to_owned());
    });

    vertices.sort();
    vertices.dedup();

    edges.sort();
    edges.dedup();

    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec!["start".to_owned()]);

    let mut counter: usize = 0;
    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let last: &str = path.last().unwrap();
        if last == "end" {
            counter += 1;
        } else {
            let next_vertices: Vec<String> = edges
                .iter()
                .filter(|v| v.0 == last)
                .map(|v| v.1.clone())
                .collect();

            for next_vertex in next_vertices {
                let is_upper: bool = next_vertex.to_ascii_uppercase() == next_vertex;
                let already_seen: bool = path.contains(&next_vertex);
                if is_upper || !already_seen {
                    let mut new_path = path.clone();
                    new_path.push(next_vertex);
                    queue.push_back(new_path);
                }
            }
        }
    }

    counter
}

fn part2(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut edges: Vec<(String, String)> = vec![];
    let mut vertices: Vec<String> = vec![];

    lines.for_each(|line| {
        let split = line.split('-').collect::<Vec<&str>>();
        edges.push((split[0].to_owned(), split[1].to_owned()));
        edges.push((split[1].to_owned(), split[0].to_owned()));
        vertices.push(split[0].to_owned());
        vertices.push(split[1].to_owned());
    });

    vertices.sort();
    vertices.dedup();

    edges.sort();
    edges.dedup();

    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec!["start".to_owned()]);

    let mut counter: usize = 0;
    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let last: &str = path.last().unwrap();
        if last == "end" {
            counter += 1;
        } else {
            let next_vertices: Vec<String> = edges
                .iter()
                .filter(|v| v.0 == last)
                .map(|v| v.1.clone())
                .collect();

            for next_vertex in next_vertices {
                let is_upper: bool = next_vertex.to_ascii_uppercase() == next_vertex;
                let already_seen: bool = path.contains(&next_vertex);
                let is_start: bool = next_vertex == "start";

                let mut smalls: Vec<String> = path
                    .iter()
                    .cloned()
                    .filter(|v| v.to_lowercase() == *v)
                    .collect();
                smalls.sort();

                let smalls_count = smalls.len();
                smalls.dedup();
                let no_small_seen_twice = smalls_count == smalls.len();

                if is_upper || !already_seen || (no_small_seen_twice && !is_start) {
                    let mut new_path = path.clone();
                    new_path.push(next_vertex);
                    queue.push_back(new_path);
                }
            }
        }
    }

    counter
}
