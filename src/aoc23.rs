use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

pub trait SetItem: PartialOrd + Ord + Clone + Hash {}
impl<T> SetItem for T where T: PartialOrd + Ord + Clone + Hash {}

#[derive(Debug, Clone)]
pub struct Set<T: SetItem> {
    set: HashSet<Vec<T>>
}

impl<T: SetItem> Set<T> {
    pub fn new() -> Set<T> {
        Set { set: HashSet::new() }
    }

    pub fn insert(&mut self, item: Vec<T>) -> bool {
        let mut cloned = item.clone();
        cloned.sort();
        self.set.insert(cloned)
    }

    pub fn remove(&mut self, item: Vec<T>) -> bool {
        let mut cloned = item.clone();
        cloned.sort();
        self.set.remove(&cloned)
    }

    pub fn contains(&self, item: Vec<&T>) -> bool {
        let mut cloned: Vec<T> = item.into_iter().cloned().collect();
        cloned.sort();
        self.set.contains(&cloned)
    }

    pub fn vec_snapshot(&self) -> Vec<Vec<T>> {
        self.set.iter().cloned().collect()
    }
}

impl<T: SetItem> IntoIterator for Set<T> {
    type Item = Vec<T>;
    type IntoIter = std::collections::hash_set::IntoIter<Vec<T>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

fn parse<'a>() -> Set<String> {
    let mut result = Set::new();
    for line in fs::read_to_string("input/23.txt").expect("Input file not found").lines() {
        result.insert(line.split('-').map(|v| String::from(v)).collect());
    }
    result
}

pub fn part1() {
    let parsed = parse();
    let mut edges = Set::new();
    let mut nodes = Set::new();
    let parse_snapshot = parsed.vec_snapshot();
    for edge in &parse_snapshot {
        edges.insert(edge.iter().map(|v| v.as_str()).collect());
        nodes.insert(vec![edge[0].as_str()]);
        nodes.insert(vec![edge[1].as_str()]);
    }
    let snapshot = nodes.vec_snapshot();
    let mut sum = 0;
    for i in 0..snapshot.len() {
        for j in (i + 1)..snapshot.len() {
            let first = snapshot[i][0];
            let second = snapshot[j][0];
            if edges.contains(vec![&first, &second]) {
                for k in (j + 1)..snapshot.len() {
                    let third = snapshot[k][0];
                    if edges.contains(vec![&first, &third]) && edges.contains(vec![&second, &third]) {
                        if first.chars().nth(0).unwrap() == 't' || second.chars().nth(0).unwrap() == 't' || third.chars().nth(0).unwrap() == 't' {
                            sum += 1;
                        }
                    }
                }
            }
        }
        println!("{} of {} finished", i + 1, snapshot.len());
    }
    println!("{sum}")
}

fn build_connections<'a>(edges: &Set<&'a str>, nodes: &Set<&'a str>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut result = HashMap::new();
    for node in nodes.vec_snapshot() {
        let target = node[0];
        let mut connected = Vec::new();
        for edge in edges.vec_snapshot() {
            let targetpos: Vec<usize> = edge.iter().enumerate().filter(|(_, v)| **v == target).map(|(i, _)| i).collect();
            if targetpos.len() == 1 {
                connected.push(edge[1 - targetpos[0]])
            }
        }
        result.insert(target, connected);
    }
    result
}

fn find_next_subgraphs<'a>(previous: &Set<&'a str>, edges: &Set<&
'a str>, connections: &HashMap<&'a str, Vec<&'a str>>) -> Set<&'a str> {
    let mut result = Set::new();
    for graph in previous.vec_snapshot() {
        'findnext: for node in connections.get(graph[0]).unwrap() {
            if graph.contains(node) { continue 'findnext; }
            for existing in &graph {
                if !edges.contains(vec![node, existing]) { continue 'findnext; }
            }
            let mut nextgraph: Vec<&str> = graph.iter().cloned().collect();
            nextgraph.push(node);
            result.insert(nextgraph);
        }
    }
    result
}

pub fn part2() {
    let parsed = parse();
    let mut edges = Set::new();
    let mut nodes = Set::new();
    let parse_snapshot = parsed.vec_snapshot();
    for edge in &parse_snapshot {
        edges.insert(edge.iter().map(|v| v.as_str()).collect());
        nodes.insert(vec![edge[0].as_str()]);
        nodes.insert(vec![edge[1].as_str()]);
    }

    let connections = build_connections(&edges, &nodes);
    let mut graphsize = 2;
    let mut graphs = edges.clone();
    loop {
        let nextgraphs = find_next_subgraphs(&graphs, &edges, &connections);
        if nextgraphs.vec_snapshot().len() == 0 {
            break;
        }
        graphs = nextgraphs;
        graphsize += 1;
        println!("{graphsize}");
    }
    println!("{graphs:?}");
}