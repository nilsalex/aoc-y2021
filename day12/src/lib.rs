#![feature(test)]
mod bench;

use std::collections::{HashMap, HashSet, VecDeque};
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

#[derive(Debug, Clone)]
struct State {
    pos: String,
    small_visited: HashSet<String>,
    some_small_twice: bool,
}

fn add_edge(edges: &mut HashMap<String, Vec<String>>, v1: &str, v2: &str) {
    if let Some(entry) = edges.get_mut(v1) {
        entry.push(v2.to_owned());
    } else {
        edges.insert(v1.to_owned(), vec![v2.to_owned()]);
    }
}

fn part1(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    lines.for_each(|line| {
        let split = line.split('-').collect::<Vec<&str>>();
        add_edge(&mut edges, split[0], split[1]);
        add_edge(&mut edges, split[1], split[0]);
    });

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State {
        pos: "start".to_owned(),
        small_visited: HashSet::from(["start".to_owned()]),
        some_small_twice: false,
    });

    let mut counter: usize = 0;
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        if state.pos == "end" {
            counter += 1;
        } else {
            let next_vertices = edges.get(&state.pos).unwrap();

            for next_vertex in next_vertices.iter() {
                let is_upper: bool = next_vertex.to_ascii_uppercase() == *next_vertex;
                if is_upper || !state.small_visited.contains(next_vertex) {
                    let mut new_state = state.clone();
                    new_state.pos = next_vertex.to_owned();
                    if !is_upper {
                        new_state.small_visited.insert(next_vertex.to_owned());
                    }
                    queue.push_back(new_state);
                }
            }
        }
    }

    counter
}

fn part2(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    lines.for_each(|line| {
        let split = line.split('-').collect::<Vec<&str>>();
        add_edge(&mut edges, split[0], split[1]);
        add_edge(&mut edges, split[1], split[0]);
    });

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State {
        pos: "start".to_owned(),
        small_visited: HashSet::from(["start".to_owned()]),
        some_small_twice: false,
    });

    let mut counter: usize = 0;
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        if state.pos == "end" {
            counter += 1;
        } else {
            let next_vertices = edges.get(&state.pos).unwrap();

            for next_vertex in next_vertices.iter() {
                let is_upper: bool = next_vertex.to_ascii_uppercase() == *next_vertex;
                let already_seen: bool = state.small_visited.contains(next_vertex);
                let is_start: bool = next_vertex == "start";

                if is_upper || !already_seen || !is_start && !state.some_small_twice {
                    let mut new_state = state.clone();
                    if !is_upper && already_seen {
                        new_state.some_small_twice = true;
                    }
                    new_state.pos = next_vertex.to_owned();
                    if !is_upper && !already_seen {
                        new_state.small_visited.insert(next_vertex.to_owned());
                    }
                    queue.push_back(new_state);
                }
            }
        }
    }

    counter
}
