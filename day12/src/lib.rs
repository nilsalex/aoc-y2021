#![feature(test)]
mod bench;

use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Node {
    Start,
    End,
    Small(u32),
    Large(u32),
}

impl Node {
    fn is_small(&self) -> bool {
        matches!(self, Self::Small(_) | Self::Start | Self::End)
    }
}

#[derive(Debug, Clone)]
struct State {
    small_visited: HashSet<Node>,
    path: Vec<Node>,
    number_paths: usize,
    small_visited_twice: Option<Node>,
}

impl State {
    fn small_node_visited(&self, node: &Node) -> bool {
        self.small_visited.contains(node)
    }
}

fn dfs(
    state: &mut State,
    graph: &HashMap<Node, Vec<Node>>,
    node: &Node,
    final_node: &Node,
    allow_extra_small: bool,
) {
    let is_small: bool = node.is_small();

    if is_small && state.small_node_visited(node) {
        if allow_extra_small && state.small_visited_twice.is_none() && node != &Node::Start {
            state.small_visited_twice = Some(*node);
        } else {
            return;
        }
    }

    if is_small {
        state.small_visited.insert(*node);
    }

    state.path.push(*node);

    if node == final_node {
        state.number_paths += 1;
        if is_small {
            if state.small_visited_twice == Some(*node) {
                state.small_visited_twice = None;
            } else {
                state.small_visited.remove(node);
            }
        }
        state.path.pop();
        return;
    }

    for next in graph.get(node).unwrap() {
        dfs(state, graph, next, final_node, allow_extra_small);
    }

    state.path.pop();
    if is_small {
        if state.small_visited_twice == Some(*node) {
            state.small_visited_twice = None;
        } else {
            state.small_visited.remove(node);
        }
    }
}

fn add_edge(edges: &mut HashMap<Node, Vec<Node>>, v1: &Node, v2: &Node) {
    if let Some(entry) = edges.get_mut(v1) {
        entry.push(v2.to_owned());
    } else {
        edges.insert(v1.to_owned(), vec![v2.to_owned()]);
    }
}

fn get_or_add_node(nodes_dict: &mut HashMap<String, Node>, name: &str) -> Node {
    if let Some(node) = nodes_dict.get(name) {
        *node
    } else {
        let node_number = nodes_dict.len() as u32;
        let node = if name.to_uppercase() == name {
            Node::Large(node_number)
        } else {
            Node::Small(node_number)
        };
        nodes_dict.insert(name.to_owned(), node);
        node
    }
}

fn parse_input(input_path: &str) -> HashMap<Node, Vec<Node>> {
    let file = File::open(input_path).unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut nodes_dict: HashMap<String, Node> = HashMap::from([
        ("start".to_owned(), Node::Start),
        ("end".to_owned(), Node::End),
    ]);

    let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();

    lines.for_each(|line| {
        let split = line.split('-').collect::<Vec<&str>>();
        let node1 = get_or_add_node(&mut nodes_dict, split[0]);
        let node2 = get_or_add_node(&mut nodes_dict, split[1]);
        add_edge(&mut edges, &node1, &node2);
        add_edge(&mut edges, &node2, &node1);
    });

    edges
}

fn part1(input_path: &str) -> usize {
    let graph = parse_input(input_path);

    let mut state = State {
        small_visited: HashSet::new(),
        path: vec![],
        number_paths: 0,
        small_visited_twice: None,
    };

    dfs(&mut state, &graph, &Node::Start, &Node::End, false);

    state.number_paths
}

fn part2(input_path: &str) -> usize {
    let graph = parse_input(input_path);

    let mut state = State {
        small_visited: HashSet::new(),
        path: vec![],
        number_paths: 0,
        small_visited_twice: None,
    };

    dfs(&mut state, &graph, &Node::Start, &Node::End, true);

    state.number_paths
}
