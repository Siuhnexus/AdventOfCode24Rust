use std::{fs, collections::HashSet};

fn parse() -> Vec<Vec<u8>> {
    let map = fs::read_to_string("input/10.txt").expect("Input file not found");
    let mut result = Vec::new();
    for line in map.lines() {
        if line.trim() == "" { continue; }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).expect("Non-number found in map") as u8);
        }
        result.push(row);
    }
    result
}

fn nextheight1(map: &Vec<Vec<u8>>, height: u8, position: (usize, usize), found: &mut HashSet<(usize, usize)>) -> u8 {
    let mut dx: i32 = 0;
    let mut dy: i32 = -1;
    let mut sum = 0;
    loop {
        let newx = position.0 as i32 + dx;
        let newy = position.1 as i32 + dy;
        if newx >= 0 && newy >= 0 && (newx as usize) < map.get(0).expect("Map has no rows").len() && (newy as usize) < map.len() {
            if map[newy as usize][newx as usize] == height + 1 {
                let nextpos = (newx as usize, newy as usize);
                if height == 8 {
                    if !found.contains(&nextpos) {
                        found.insert(nextpos);
                        sum += 1;
                    }
                }
                else {
                    sum += nextheight1(map, height + 1, nextpos, found)
                }
            }
        }
        if dx == -1 { break; }
        let temp = dx;
        dx = -dy;
        dy = temp;
    }
    sum
}
fn nextheight2(map: &Vec<Vec<u8>>, height: u8, position: (usize, usize)) -> u8 {
    let mut dx: i32 = 0;
    let mut dy: i32 = -1;
    let mut sum = 0;
    loop {
        let newx = position.0 as i32 + dx;
        let newy = position.1 as i32 + dy;
        if newx >= 0 && newy >= 0 && (newx as usize) < map.get(0).expect("Map has no rows").len() && (newy as usize) < map.len() {
            if map[newy as usize][newx as usize] == height + 1 {
                let nextpos = (newx as usize, newy as usize);
                if height == 8 {
                    sum += 1;
                }
                else {
                    sum += nextheight2(map, height + 1, nextpos)
                }
            }
        }
        if dx == -1 { break; }
        let temp = dx;
        dx = -dy;
        dy = temp;
    }
    sum
}
pub fn part1() {
    let map = parse();
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if *h == 0 {
                sum += nextheight1(&map, *h, (x, y), &mut HashSet::new()) as u32;
            }
        }
    }
    println!("{:?}", sum);
}
pub fn part2() {
    let map = parse();
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if *h == 0 {
                sum += nextheight2(&map, *h, (x, y)) as u32;
            }
        }
    }
    println!("{:?}", sum);
}