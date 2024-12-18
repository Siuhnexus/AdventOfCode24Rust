use std::{fs, collections::{HashMap, HashSet}};

use crate::helpers::Direction;

#[derive(PartialEq, Clone, Debug)]
pub enum CellContent {
    Wall,
    Start,
    End,
    Path
}

impl CellContent {
    pub fn from_char(c: char) -> CellContent {
        match c {
            'S' => CellContent::Start,
            'E' => CellContent::End,
            '#' => CellContent::Wall,
            '.' => CellContent::Path,
            _ => panic!("Unknown cell content char")
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            CellContent::Wall => '#',
            CellContent::Start => 'S',
            CellContent::End => 'E',
            CellContent::Path => '.',
        }
    }
}

fn parse() -> Vec<Vec<CellContent>> {
    let mut result = Vec::new();

    for line in fs::read_to_string("input/16.txt").expect("Input file not found").lines() {
        if line.trim() == "" { continue; }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(CellContent::from_char(c));
        }
        result.push(row);
    }

    result
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Node {
    pos: (usize, usize),
    dir: Direction
}
fn addedge(edges: &mut HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>, first: Node, second: Node) {
    edges.insert((first.clone(), second.clone()));
    edges.insert((second.clone(), first));
    distances.entry(second).or_insert(u32::MAX);
}
fn ispath(grid: &Vec<Vec<CellContent>>, pos: Option<(usize, usize)>) -> bool {
    match pos {
        None => panic!("Map had no enclosing wall"),
        Some(v) => grid[v.1][v.0] != CellContent::Wall
    }
}
fn processnode(grid: &Vec<Vec<CellContent>>, mut last: Node, edges: &mut HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>) {
    loop {
        let next = Node { pos: last.dir.step(last.pos).expect("Current node is not in grid bounds"), dir: last.dir.clone() };
        let edge = (last, next);
        if edges.contains(&edge) { return; }
        addedge(edges, distances, edge.0, edge.1);
        let mut possible: Vec<Direction> = vec![next.dir.turn_left(), next.dir, next.dir.turn_right()].into_iter().filter(|dir| ispath(grid, dir.step(next.pos))).collect();

        if possible.len() == 1 {
            let only = possible.pop().unwrap();
            if only != next.dir {
                addedge(edges, distances, next, Node { pos: next.pos, dir: only })
            }
            last = Node { pos: next.pos, dir: only };
        }
        else {
            for dir in possible {
                if dir != next.dir {
                    addedge(edges, distances, next, Node { pos: next.pos, dir: dir });
                }
                processnode(grid, Node { pos: next.pos, dir }, edges, distances);
            }
            break;
        }
    }
}
fn makegraph(grid: &Vec<Vec<CellContent>>) -> (HashSet<(Node, Node)>, HashMap<Node, u32>) {
    let mut edges = HashSet::new();
    let mut distances = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == CellContent::Start {
                let right = Node { pos: (x, y), dir: Direction::Right };
                distances.insert(right, 0);
                let top = Node { pos: right.pos, dir: right.dir.turn_left() };
                let bottom = Node { pos: right.pos, dir: right.dir.turn_right() };
                let left = Node { pos: right.pos, dir: right.dir.flip() };
            
                addedge(&mut edges, &mut distances, right, top);
                addedge(&mut edges, &mut distances, right, bottom);
                addedge(&mut edges, &mut distances, top, left);
                addedge(&mut edges, &mut distances, bottom, left);

                for possible in Direction::Top.into_iter().filter(|dir| ispath(grid, dir.step((x, y)))) {
                    processnode(&grid, Node { pos: (x, y), dir: possible }, &mut edges, &mut distances)
                }
            }
        }
    }
    (edges, distances)
}

fn processifexists(edges: &HashSet<(Node, Node)>, nodes: &mut Vec<(Node, u32)>, previous: Node, pivot: Node, dist: u32, toadd: u32) {
    if nodes.iter().any(|(node, _)| *node == pivot) && edges.contains(&(previous, pivot)) {
        let position = nodes.iter().position(|n| n.0 == pivot).unwrap();
        nodes[position].1 = nodes[position].1.min(dist + toadd);
    }
}
fn dijkstra(edges: &HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>) {
    let mut nodes: Vec<(Node, u32)> = distances.iter().map(|(key, value)| (*key, *value)).collect();
    distances.clear();
    while nodes.len() > 0 {
        let temp = nodes.iter().min_by_key(|(_, d)| d).expect("No starting node exists");
        let (nnode, dist) = nodes.swap_remove(nodes.iter().position(|n|  n == temp).unwrap());
        distances.entry(nnode).or_insert(dist);
        for dir in Direction::Top.into_iter() {
            let pivot = Node { pos: nnode.pos, dir };
            processifexists(edges, &mut nodes, nnode, pivot, dist, 1000);
        }
        let pivot = Node { pos: nnode.dir.step(nnode.pos).expect("Input map had no enclosing wall"), dir: nnode.dir.clone() };
        processifexists(edges, &mut nodes, nnode, pivot, dist, 1);
    }
}

pub fn part1() {
    let grid = parse();
    let (edges, mut distances) = makegraph(&grid);
    dijkstra(&edges, &mut distances);
    println!("{}", distances.iter().filter(|(node, _)| grid[node.pos.1][node.pos.0] == CellContent::End).map(|(_, dist)| dist).min().unwrap());
}

fn trackedprocess(edges: &HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>, predecessor: &mut HashMap<Node, Vec<Node>>, previous: Node, pivot: Node, dist: u32, toadd: u32) {
    if distances.contains_key(&pivot) && edges.contains(&(previous, pivot)) {
        let prevval = *distances.get(&pivot).unwrap();
        let current = dist + toadd;
        if current < prevval {
            distances.insert(pivot, current);
            predecessor.insert(pivot, vec![previous]);
        }
        else if current == prevval {
            distances.insert(pivot, current);
            predecessor.entry(pivot).or_insert(Vec::new()).push(previous);
        }
    }
}
fn trackeddijkstra(edges: &HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>, predecessor: &mut HashMap<Node, Vec<Node>>) {
    let mut consumable = distances.clone();
    distances.clear();
    while consumable.len() > 0 {
        let temp = consumable.iter().min_by_key(|(_, d)| **d).expect("No starting node exists");
        let (nnode, dist) = (*temp.0, *temp.1);
        consumable.remove(&nnode);
        distances.entry(nnode).or_insert(dist);
        for dir in Direction::Top.into_iter() {
            let pivot = Node { pos: nnode.pos, dir };
            trackedprocess(edges, &mut consumable, predecessor, nnode, pivot, dist, 1000);
        }
        let pivot = Node { pos: nnode.dir.step(nnode.pos).expect("Input map had no enclosing wall"), dir: nnode.dir.clone() };
        trackedprocess(edges, &mut consumable, predecessor, nnode, pivot, dist, 1);
    }
}
fn trace(pathparts: &mut HashSet<(usize, usize)>, predecessor: &HashMap<Node, Vec<Node>>, current: Node) {
    pathparts.insert(current.pos);
    match predecessor.get(&current) {
        None => return,
        Some(v) => {
            for p in v {
                trace(pathparts, predecessor, *p);
            }
        }
    }
}
pub fn part2() {
    let grid = parse();
    let (edges, mut distances) = makegraph(&grid);
    let mut predecessor = HashMap::new();
    trackeddijkstra(&edges, &mut distances, &mut predecessor);
    let mut pathparts = HashSet::new();
    let endnode = *distances.keys().into_iter().find(|n| grid[n.pos.1][n.pos.0] == CellContent::End).unwrap();
    let minlen = Direction::Top.into_iter().map(|dir| match distances.get(&Node { pos: endnode.pos, dir }) { None => u32::MAX, Some(v) => *v }).min().unwrap();
    for dir in Direction::Top.into_iter() {
        let pivot = Node { pos: endnode.pos, dir };
        if *distances.get(&pivot).unwrap_or(&u32::MAX) > minlen { continue; }
        trace(&mut pathparts, &predecessor, Node { pos: endnode.pos, dir });
    }
    println!("{}", pathparts.len());
}