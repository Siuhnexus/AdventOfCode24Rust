use std::{fs, num::TryFromIntError, collections::HashSet};

fn addtosize(a: usize, b: i32) -> Result<usize, TryFromIntError> {
    let newval = match i32::try_from(a) { Err(e) => return Err(e), Ok(v) => v } + b;
    return usize::try_from(newval);
}
fn parse() -> (Vec<Vec<bool>>, (usize, usize), (i32, i32)) {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let mut dir: (i32, i32) = (0, 0);

    let input = fs::read_to_string("input/06.txt").expect("Input file not found");
    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        if line == "" { continue; }
        grid.push(line.chars().enumerate().map(|(x, c)| -> bool {
            match c {
                '#' => return true,
                '^' => { pos = (x, y); dir = (0, -1); return false },
                '>' => { pos = (x, y); dir = (1, 0); return false },
                'v' => { pos = (x, y); dir = (0, 1); return false },
                '<' => { pos = (x, y); dir = (-1, 0); return false },
                _ => return false
            }
        }).collect());
    }

    return (grid, pos, dir)
}

pub fn part1() {
    let (grid, mut pos, mut dir) = parse();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for (y, row) in (&grid).iter().enumerate() {
        let mut vrow: Vec<bool> = (0..row.len()).map(|n| -> bool { return false; }).collect();
        if y == pos.1 {
            vrow[pos.0] = true;
        }
        visited.push(vrow);
    }

    loop {
        let newx = match addtosize(pos.0, dir.0) { Err(_e) => break, Ok(v) => v };
        let newy = match addtosize(pos.1, dir.1) { Err(_e) => break, Ok(v) => v };
        let nexttile = match grid.get(newy) { None => break, Some(v) => match v.get(newx) {
            None => break,
            Some(v) => v
        }};
        if nexttile == &true {
            dir = (-dir.1, dir.0);
            continue;
        }
        pos = (newx, newy);
        visited[newy][newx] = true;
    }
    let sum = visited.iter().fold(0, |prev, row| -> i32 { return prev + row.iter().fold(0, |prev, v| -> i32 { return prev + if v == &true { 1 } else { 0 } }) });
    println!("{:?}", sum);
}

pub fn part2() {
    let (originalgrid, startingpos, startingdir) = parse();

    let mut possibilities = 0;

    for (y, row) in originalgrid.iter().enumerate() {
        for(x, v) in row.iter().enumerate() {
            if v == &true { continue; }
            let mut grid = originalgrid.clone();
            grid[y][x] = true;
            let mut pos = startingpos.clone();
            let mut dir = startingdir.clone();
            let mut encountered: HashSet<((usize, usize), (i32, i32))> = HashSet::new();

            let mut loops = false;
            loop {
                let newx = match addtosize(pos.0, dir.0) { Err(_e) => break, Ok(v) => v };
                let newy = match addtosize(pos.1, dir.1) { Err(_e) => break, Ok(v) => v };
                let nexttile = match grid.get(newy) { None => break, Some(v) => match v.get(newx) {
                    None => break,
                    Some(v) => v
                }};
                if nexttile == &true {
                    if encountered.contains(&(pos, dir)) {
                        loops = true;
                        break;
                    }
                    encountered.insert((pos, dir));
                    dir = (-dir.1, dir.0);
                    continue;
                }
                pos = (newx, newy);
            }
            if loops { possibilities += 1; }
        }
    }

    println!("{:?}", possibilities);
}