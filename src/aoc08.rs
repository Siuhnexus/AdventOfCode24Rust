use std::{collections::HashMap, fs, u32};

fn parse() -> (HashMap<char, Vec<(u32, u32)>>, (usize, usize)) {
    let map = fs::read_to_string("input/08.txt").expect("Input file not found");
    let mut result: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    let mut xlen = 0;
    let mut ylen = 0;

    for (x, line) in map.lines().enumerate() {
        if line.trim() == "" { continue; }
        ylen = line.len();
        xlen = x + 1;
        for (y, c) in line.chars().enumerate() {
            if !c.is_alphanumeric() { continue; }
            match result.get_mut(&c) {
                None => {
                    let mut group = Vec::new();
                    group.push((x as u32, y as u32));
                    result.insert(c, group);
                },
                Some(v) => {
                    v.push((x as u32, y as u32));
                }
            }
        }
    }
    return (result, (xlen, ylen));
}

fn printmap(nodes: &Vec<Vec<bool>>) {
    println!("{}", nodes.iter().fold(String::from(""), |prev, row| { prev + &row.iter().fold(String::from(""), |prev, v| { prev + if *v { "#" } else { "." } }) + "\n" }) + "\n");
}

pub fn part1() {
    let (result, dimensions) = parse();
    let mut nodes: Vec<Vec<bool>> = Vec::new();
    for _x in 0..dimensions.0 {
        nodes.push((0..dimensions.1).map(|_y| -> bool { false }).collect());
    }
    for (_key, positions) in result {
        for i in 0..positions.len() {
            let first = positions[i];
            for j in i+1..positions.len() {
                let second = positions[j];
                let diff = (second.0 as i32 - first.0 as i32, second.1 as i32 - first.1 as i32);
                for pivot in [(second.0 as i32 + diff.0, second.1 as i32 + diff.1), (first.0 as i32 - diff.0, first.1 as i32 - diff.1)] {
                    if pivot.0 < 0 || pivot.0 as usize >= dimensions.0 || pivot.1 < 0 || pivot.1 as usize >= dimensions.1 { continue; }
                    nodes[pivot.0 as usize][pivot.1 as usize] = true;
                }
            }
        }
    }
    println!("{:?}", nodes.iter().fold(0, |prev, row| -> u32 { prev + row.iter().fold(0, |prev, v| -> u32 { prev + if *v { 1 } else { 0 } }) }))
}

pub fn part2() {
    let (result, dimensions) = parse();
    let mut nodes: Vec<Vec<bool>> = Vec::new();
    for _x in 0..dimensions.0 {
        nodes.push((0..dimensions.1).map(|_y| -> bool { false }).collect());
    }

    for (_key, positions) in result {
        for i in 0..positions.len() {
            let first = positions[i];
            for j in (i + 1)..positions.len() {
                let second = positions[j];
                let diff = (second.0 as i32 - first.0 as i32, second.1 as i32 - first.1 as i32);
                let mut k = u32::MAX;
                if diff.0 != 0 {
                    let bound = if diff.0 > 0 { 0 } else { dimensions.0 - 1 } as u32;
                    k = k.min(((first.0 as i32 - bound as i32) / diff.0) as u32);
                }
                if diff.1 != 0 {
                    let bound = if diff.1 > 0 { 0 } else { dimensions.1 - 1 } as u32;
                    k = k.min(((first.1 as i32 - bound as i32) / diff.1) as u32);
                }
                let mut start = (first.0 as i32 - k as i32 * diff.0, first.1 as i32 - k as i32 * diff.1);
                while start.0 >= 0 && (start.0 as usize) < dimensions.0 && start.1 >= 0 && (start.1 as usize) < dimensions.1 {
                    nodes[start.0 as usize][start.1 as usize] = true;
                    start = (start.0 + diff.0, start.1 + diff.1);
                }
            }
        }
    }

    println!("{:?}", nodes.iter().fold(0, |prev, row| -> u32 { prev + row.iter().fold(0, |prev, v| -> u32 { prev + if *v { 1 } else { 0 } }) }))
}