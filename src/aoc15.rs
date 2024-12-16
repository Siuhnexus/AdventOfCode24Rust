use std::fs;

use crate::helpers::{Direction, Orientation};

#[derive(Debug, PartialEq, Clone)]
enum CellContent {
    Box,
    BoxLeft,
    BoxRight,
    Space,
    Wall,
}

impl CellContent {
    fn _to_char(&self) -> char {
        match self {
            CellContent::Box => 'O',
            CellContent::BoxLeft => '[',
            CellContent::BoxRight => ']',
            CellContent::Space => '.',
            CellContent::Wall => '#',
        }
    }
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

fn simplemove(grid: &mut Vec<Vec<CellContent>>, dir: &Direction, pos: &mut (usize, usize)) {
    let mut lookahead = dir.step((pos.0, pos.1)).expect("Input map had no enclosing wall");
    while grid[lookahead.1][lookahead.0] != CellContent::Wall {
        match grid[lookahead.1][lookahead.0] {
            CellContent::Box | CellContent::BoxLeft | CellContent::BoxRight => { lookahead = dir.step((lookahead.0, lookahead.1)).expect("Input map had no enclosing wall"); },
            CellContent::Space => {
                let reverse = dir.flip();
                let mut prev = reverse.step(lookahead).expect("Impossible state");
                while (prev.0, prev.1) != *pos && grid[prev.1][prev.0] != CellContent::Space {
                    grid[lookahead.1][lookahead.0] = grid[prev.1][prev.0].clone();
                    grid[prev.1][prev.0] = CellContent::Space;
                    lookahead = prev;
                    prev = reverse.step(lookahead).expect("Impossible state");
                }
                *pos = dir.step(*pos).expect("Input map had no enclosing wall");
                break;
            },
            _ => { panic!("Impossible state"); }
        }
    }
}

fn gpssum(grid: &Vec<Vec<CellContent>>) {
    let mut gpssum: u32 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == CellContent::Box || *c == CellContent::BoxLeft {
                gpssum += y as u32 * 100 + x as u32;
            }
        }
    }
    println!("{gpssum}");
}
fn _printmap(grid: &Vec<Vec<CellContent>>, pos: &(usize, usize)) {
    println!("{}", grid.iter().enumerate().fold(String::new(), |prev, (y, row)| prev + &row.iter().enumerate().fold(String::new(), |prev, (x, c)| {
        let current = if x == pos.0 && y == pos.1 { String::from("@") } else { c._to_char().to_string() };
        prev + &current
    }) + "\n"))
}

pub fn part1() {
    let (mut grid, dirs, mut pos) = parse();
    for dir in dirs {
        simplemove(&mut grid, &dir, &mut pos)
    }
    gpssum(&grid);
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

fn canmove(grid: &mut Vec<Vec<CellContent>>, dir: &Direction, pos: &(usize, usize)) -> bool {
    let lookahead = dir.step(*pos).expect("Input map had no enclosing wall");
    match grid[lookahead.1][lookahead.0] {
        CellContent::BoxLeft => {
            if !canmove(grid, dir, &lookahead) { return false; }
            return canmove(grid, dir, &Direction::Right.step(lookahead).expect("Input map had half box"));
        },
        CellContent::BoxRight => {
            if !canmove(grid, dir, &lookahead) { return false; }
            return canmove(grid, dir, &Direction::Left.step(lookahead).expect("Input map had half box"));
        },
        CellContent::Wall => return false,
        CellContent::Space => return true,
        CellContent::Box => panic!("Impossible value. Was the grid not expanded?")
    }
}
fn executemove(grid: &mut Vec<Vec<CellContent>>, dir: &Direction, pos: &(usize, usize)) {
    let lookahead = dir.step(*pos).expect("Input map had no enclosing wall");
    match grid[lookahead.1][lookahead.0] {
        CellContent::BoxLeft => {
            executemove(grid, dir, &lookahead);
            executemove(grid, dir, &Direction::Right.step(lookahead).expect("Input map had half box"));
        },
        CellContent::BoxRight => {
            executemove(grid, dir, &lookahead);
            executemove(grid, dir, &Direction::Left.step(lookahead).expect("Input map had half box"));
        },
        CellContent::Wall => panic!("Impossible move executed. Did you check with canmove() if this move is possible?"),
        CellContent::Space => { },
        CellContent::Box => panic!("Impossible value. Was the grid not expanded?")
    }
    if grid[pos.1][pos.0] != CellContent::Space {
        grid[lookahead.1][lookahead.0] = grid[pos.1][pos.0].clone();
        grid[pos.1][pos.0] = CellContent::Space;
    }
}

pub fn part2() {
    let (mut grid, dirs, mut pos) = parse();
    grid = expand(grid);
    pos = (pos.0 * 2, pos.1);

    for dir in dirs {
        if dir.orientation() == Orientation::Horizontal {
            simplemove(&mut grid, &dir, &mut pos);
        }
        else {
            if canmove(&mut grid, &dir, &pos) {
                executemove(&mut grid, &dir, &pos);
                pos = dir.step::<usize>(pos).expect("Input map had no enclosing wall");
            }
        }
    }
    gpssum(&grid);
}