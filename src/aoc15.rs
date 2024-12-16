use std::{fs};

use crate::helpers::Direction;

#[derive(Debug, PartialEq)]
enum CellContent {
    Box,
    BoxLeft,
    BoxRight,
    Space,
    Wall,
}

fn parse() -> (Vec<Vec<CellContent>>, Vec<Direction>, (usize, usize)) {
    let mut grid = Vec::new();
    let mut dirs = Vec::new();
    let mut pos = (0, 0);

    let input = fs::read_to_string("input/15.txt").expect("Input file not found");
    let instructions: Vec<&str> = input.split("\r\n\r\n").collect();
    if instructions.len() < 2 { panic!("Input file was formatted incorrectly") }
    for (y, line) in instructions[0].lines().enumerate() {
        if line.trim() == "" { continue; }
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => CellContent::Wall,
                'O' => CellContent::Box,
                _ => CellContent::Space
            });
            if c == '@' {
                pos = (x, y);
            }
        }
        grid.push(row)
    }
    for line in instructions[1..].join("").lines() {
        for c in line.chars() {
            dirs.push(Direction::from_char(c));
        }
    }

    (grid, dirs, pos)
}

pub fn part1() {
    let (mut grid, dirs, mut pos) = parse();
    for dir in dirs {
        let mut lookahead = dir.step((pos.0, pos.1)).expect("Input map had no enclosing wall");
        while grid[lookahead.1 as usize][lookahead.0 as usize] != CellContent::Wall {
            match grid[lookahead.1 as usize][lookahead.0 as usize] {
                CellContent::Box => { lookahead = dir.step((lookahead.0, lookahead.1)).expect("Input map had no enclosing wall"); },
                CellContent::Space => {
                    let reverse = dir.flip();
                    let mut prev = reverse.step(lookahead).expect("Impossible state");
                    while (prev.0, prev.1) != pos && grid[prev.1][prev.0] == CellContent::Box {
                        grid[lookahead.1][lookahead.0] = CellContent::Box;
                        grid[prev.1][prev.0] = CellContent::Space;
                        lookahead = prev;
                        prev = reverse.step(lookahead).expect("Impossible state");
                    }
                    pos = dir.step(pos).expect("Input map had no enclosing wall");
                    break;
                },
                _ => { panic!("Impossible state"); }
            }
        }
    }
    let mut gpssum: u32 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == CellContent::Box {
                gpssum += y as u32 * 100 + x as u32;
            }
        }
    }
    println!("{gpssum}");
}

fn expand(grid: Vec<Vec<CellContent>>) -> Vec<Vec<CellContent>> {
    let mut result = Vec::new();
    for row in grid {
        let mut crow = Vec::new();
        for c in row {
            match c {
                CellContent::Space => { crow.push(CellContent::Space); crow.push(CellContent::Space); }
                CellContent::Wall => { crow.push(CellContent::Wall); crow.push(CellContent::Wall); }
                CellContent::Box => { crow.push(CellContent::BoxLeft); crow.push(CellContent::BoxRight); },
                _ => { panic!("Grid cannot contain box parts. Was it already expanded?") }
            }
        }
        result.push(crow);
    }
    result
}

pub fn part2() {
    let (mut grid, dirs, mut pos) = parse();
    grid = expand(grid);
    pos = (pos.0 * 2, pos.1);
}