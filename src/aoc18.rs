use std::{fs, collections::{HashMap, HashSet}};

use crate::helpers::Direction;

fn parse() -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for line in fs::read_to_string("input/18.txt").expect("Input file not found").lines() {
        if line.trim() == "" { continue }
        let nums: Vec<usize> = line.split(",").map(|numstr| numstr.parse().expect("Non-number found in input")).collect();
        if nums.len() != 2 { panic!("Input was formatted incorrectly") }
        result.push((nums[0], nums[1]))
    }
    result
}

fn makegrid(bytes: &[(usize, usize)], bounds: (usize, usize)) -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    for _ in 0..bounds.0 {
        let mut col = Vec::new();
        for _ in 0..bounds.1 {
            col.push(true)
        }
        grid.push(col);
    }
    for byte in bytes {
        grid[byte.0][byte.1] = false;
    }
    grid
}

fn dijkstra(grid: &Vec<Vec<bool>>, bounds: (usize, usize), distances: &mut HashMap<(usize, usize), u32>) -> u32 {
    let mut nodes: HashSet<(usize, usize)> = grid.iter().enumerate().flat_map(|(x, col)| col.iter().enumerate().filter(|(y, v)| **v).map(|(y, _)| (x, y)).collect::<Vec<_>>()).collect();
    while nodes.len() > 0 {
        let temp = match distances.iter().filter(|(node, _)| nodes.contains(*node)).min_by_key(|(_, d)| **d) { None => return u32::MAX, Some(v) => v };
        let (nnode, dist) = (*temp.0, *temp.1);
        nodes.remove(&nnode);
        for dir in Direction::Top.into_iter() {
            let pivot =  match dir.step(nnode) { None => continue, Some(v) => v };
            if pivot.0 >= bounds.0 || pivot.1 >= bounds.1 { continue; }
            if !grid[pivot.0][pivot.1] { continue; }
            let existing = match distances.get(&pivot) { None => u32::MAX, Some(v) => *v };
            distances.insert(pivot, existing.min(dist + 1));
            if pivot == (bounds.0 - 1, bounds.1 - 1) {
                return dist + 1;
            }
        }
    }
    return u32::MAX
}

pub fn part1() {
    let bytes = parse();
    let bounds = (71, 71);
    let grid = makegrid(&bytes[..1024], bounds);
    let mut distances = HashMap::new();
    distances.insert((0, 0), 0);
    println!("{}", dijkstra(&grid, bounds, &mut distances));
}

pub fn part2() {
    let bytes = parse();
    let bounds = (71, 71);
    let mut grid = makegrid(&bytes[..2800], bounds);
    let mut i = 2800;
    while i < bytes.len() {
        println!("{i}");
        let current = bytes[i];
        grid[current.0][current.1] = false;
        let mut distances = HashMap::new();
        distances.insert((0, 0), 0);
        if dijkstra(&grid, bounds, &mut distances) == u32::MAX {
            println!("{},{}", current.0, current.1);
            break;
        }
        i += 1;
    }
}