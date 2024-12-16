use std::{fs, collections::{HashMap, HashSet}};

use crate::helpers::Direction;

#[derive(PartialEq, Clone)]
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node {
    pos: (usize, usize),
    dir: Direction
}
fn processalldirs(grid: &mut Vec<Vec<CellContent>>, current: Node, edges: &mut HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>) {

}
fn processnode(grid: &mut Vec<Vec<CellContent>>, last: Node, edges: &mut HashSet<(Node, Node)>, distances: &mut HashMap<Node, u32>) {
    
}
fn makegraph(grid: &Vec<Vec<CellContent>>) -> (HashSet<(Node, Node)>, HashMap<Node, u32>) {
    let mut edges = HashSet::new();
    let mut distances = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == CellContent::Start {
                let starting = Node { pos: (x, y), dir: Direction::Right };
                distances.insert(starting.clone(), 0);
                processalldirs(&mut grid.clone(), starting, &mut edges, &mut distances)
            }
        }
    }
    (edges, distances)
}

pub fn part1() {
    let grid = parse();
}