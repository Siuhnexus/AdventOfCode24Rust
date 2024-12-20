use std::{fs, collections::{HashMap, HashSet}};

use crate::helpers::Direction;

fn parse() -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut result = Vec::new();
    let mut start = (0, 0);
    for (y, line) in fs::read_to_string("input/20.txt").expect("Input file not found").lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' | 'E' => { result.push((x, y)); },
                'S' => { start = (x, y); result.push(start); },
                _ => {}
            }
        }
    }
    (result, start)
}

fn sorttrack(track: &Vec<(usize, usize)>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut consumable: Vec<(usize, usize)> = track.iter().filter(|pos| **pos != start).copied().collect();
    let mut result = vec![start];
    let mut current = start;
    'outer: loop {
        for i in 0..consumable.len() {
            let possible = consumable[i];
            let dist = (current.0 as i32 - possible.0 as i32).abs() + (current.1 as i32 - possible.1 as i32).abs();
            if dist == 1 {
                result.push(possible);
                current = possible;
                consumable.remove(i);
                continue 'outer;
            }
        }
        break;
    }
    if consumable.len() > 0 { panic!("Given track was not continuous"); }
    result
}

pub fn part1() {
    let (mut pathtiles, start) = parse();
    pathtiles = sorttrack(&pathtiles, start);
    let tile_to_index: HashMap<(usize, usize), usize> = pathtiles.iter().enumerate().map(|(i, pos)| (*pos, i)).collect();
    let mut analyzed: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut cheats: HashMap<((usize, usize), (usize, usize)), u32> = HashMap::new();
    for tile in pathtiles.iter() {
        for dir in Direction::Top.into_iter() {
            let next = match dir.step(*tile) {
                None => continue,
                Some(v) => {
                    match dir.step(v) {
                        None => continue,
                        Some(v) => {
                            v
                        }
                    }
                }
            };

            if pathtiles.contains(&next) && !analyzed.contains(&(next, *tile)) {
                let distance = *tile_to_index.get(&next).unwrap() as u32 - *tile_to_index.get(tile).unwrap() as u32 - 2;
                if distance == 0 { analyzed.insert((*tile, next)); continue; }
                cheats.insert((*tile, next), distance);
                analyzed.insert((*tile, next));
            }
        }
    }
    let mut count: u32 = 0;
    for (_, value) in cheats {
        if value >= 100 {
            count += 1;
        }
    }
    println!("{count}");
}

pub fn part2() {
    let (mut pathtiles, start) = parse();
    pathtiles = sorttrack(&pathtiles, start);
    let tile_to_index: HashMap<(usize, usize), usize> = pathtiles.iter().enumerate().map(|(i, pos)| (*pos, i)).collect();
    let mut cheats: HashMap<((usize, usize), (usize, usize)), u32> = HashMap::new();
    for (i, tile) in pathtiles.iter().enumerate() {
        println!("{} / {}", tile_to_index.get(tile).unwrap(), pathtiles.len());
        for next in pathtiles.iter().skip(i + 1) {
            let dist = (tile.0 as i32 - next.0 as i32).abs() + (tile.1 as i32 - next.1 as i32).abs();
            let trackdist = *tile_to_index.get(next).unwrap() as i32 - *tile_to_index.get(tile).unwrap() as i32;
            if dist < 2 || dist == trackdist || dist > 20 { continue; }
            cheats.insert((*tile, *next), (trackdist - dist) as u32);
        }
    }
    let mut count: u32 = 0;
    for (_, value) in cheats {
        if value >= 100 {
            count += 1;
        }
    }
    println!("{count}");
}