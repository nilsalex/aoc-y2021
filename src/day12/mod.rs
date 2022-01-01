use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");

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

fn parse_input(s: &str) -> HashMap<Node, Vec<Node>> {
    let mut nodes_dict: HashMap<String, Node> = HashMap::from([
        ("start".to_owned(), Node::Start),
        ("end".to_owned(), Node::End),
    ]);

    let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();

    s.lines().for_each(|line| {
        let split = line.split('-').collect::<Vec<&str>>();
        let node1 = get_or_add_node(&mut nodes_dict, split[0]);
        let node2 = get_or_add_node(&mut nodes_dict, split[1]);
        add_edge(&mut edges, &node1, &node2);
        add_edge(&mut edges, &node2, &node1);
    });

    edges
}

pub fn part1(s: &str) -> usize {
    let graph = parse_input(s);

    let mut state = State {
        small_visited: HashSet::new(),
        path: vec![],
        number_paths: 0,
        small_visited_twice: None,
    };

    dfs(&mut state, &graph, &Node::Start, &Node::End, false);

    state.number_paths
}

pub fn part2(s: &str) -> usize {
    let graph = parse_input(s);

    let mut state = State {
        small_visited: HashSet::new(),
        path: vec![],
        number_paths: 0,
        small_visited_twice: None,
    };

    dfs(&mut state, &graph, &Node::Start, &Node::End, true);

    state.number_paths
}

extern crate test;

#[cfg(test)]
use test::Bencher;

#[test]
fn test_day12_part1() {
    assert_eq!(part1(INPUT), 3450);
}

#[test]
fn test_day12_part2() {
    assert_eq!(part2(INPUT), 96528);
}

#[bench]
fn bench_day12_part1(b: &mut Bencher) {
    b.iter(|| part1(INPUT))
}

#[bench]
fn bench_day12_part2(b: &mut Bencher) {
    b.iter(|| part2(INPUT))
}
