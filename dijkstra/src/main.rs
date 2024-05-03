use std::cmp::Ordering;
use std::collections::HashMap;

// first we need to represent the graph and edges

#[derive(Debug, Eq, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Graph {
    nodes: usize,
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Graph {
            nodes,
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize, cost: usize) {
        self.edges.entry(src).or_insert(vec![]).push(Edge {
            node: dest,
            cost,
        })
    }
}


fn main() {
    println!("Hello, world!");
}