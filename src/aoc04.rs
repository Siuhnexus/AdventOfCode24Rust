use std::{fs, num::TryFromIntError};

const BEGINNING: char = 'X';
const END: &str = "MAS";

const MIDDLE: char = 'A';
const ARMS: (char, char) = ('M', 'S');

fn parse() -> Vec<Vec<char>> {
    let words: String = match fs::read_to_string("input/04.txt") {
        Err(_e) => panic!("No input file found"),
        Ok(v) => v
    };

    let mut result: Vec<Vec<char>> = Vec::new();
    for line in words.lines() {
        if line == "" { continue };
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }

    result
}

fn addtosize(a: usize, b: i32) -> Result<usize, TryFromIntError> {
    let newval = match i32::try_from(a) { Err(e) => return Err(e), Ok(v) => v } + b;
    return usize::try_from(newval);
}

fn check1(field: &Vec<Vec<char>>, position: (usize, usize), direction: (i32, i32)) -> bool {
    let mut x = position.0;
    let mut y = position.1;
    let dx = direction.0;
    let dy = direction.1;

    for c in END.chars() {
        x = match addtosize(x, dx) { Err(_e) => return false, Ok(v) => v};
        y = match addtosize(y, dy) { Err(_e) => return false, Ok(v) => v};

        let fieldchar = match field.get(y) {
            None => return false,
            Some(v) => match v.get(x) {
                None => return false,
                Some(fc) => fc
            }
        };
        if *fieldchar != c { return false }
    }
    return true;
}

pub fn part1() {
    let field = parse();

    let mut sum = 0;
    for (row, y) in field.iter().zip(0..field.len()) {
        for (c, x) in row.iter().zip(0..row.len()) {
            if *c == BEGINNING {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if check1(&field, (x, y), (dx, dy)) {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", sum);
}

fn check2(field: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    let x = position.0;
    let y = position.1;

    let armcharsresult: Result<Vec<char>, &str> = (0..=3).map(|dir| -> Result<char, &str> {
        let dx = (dir / 2) * 2 - 1;
        let dy = (dir % 2) * 2 - 1;
        let armx = match addtosize(x, dx) { Err(_e) => return Err("Calculation not possible"), Ok(v) => v };
        let army = match addtosize(y, dy) { Err(_e) => return Err("Calculation not possible"), Ok(v) => v };
        match field.get(army) { None => Err("Index doesn't exist"), Some(r) => match r.get(armx) { None => Err("Index doesn't exist"), Some(c) => Ok(*c) }}
    }).collect();
    let armchars = match armcharsresult {
        Err(_e) => return false,
        Ok(v) => v
    };
    if armchars.len() != 4 { return false };
    let mut matches = false;
    for permute in [false, true] {
        let tomatch = if permute { [ARMS.1, ARMS.0] } else { [ARMS.0, ARMS.1] };
        let mut permutationmatch = false;
        for order in [[0, 2, 1, 3], [0, 1, 2, 3]] { // Horizontal or vertical orientation
            let mut current = true;
            for (i, j) in order.iter().zip([0, 0, 1, 1]) {
                current = current && armchars[*i] == tomatch[j];
            }
            permutationmatch = permutationmatch || current;
        }
        matches = matches || permutationmatch
    }
    return matches;
}

pub fn part2() {
    let field = parse();

    let mut sum = 0;
    for (row, y) in field.iter().zip(0..field.len()) {
        for (c, x) in row.iter().zip(0..row.len()) {
            if *c == MIDDLE && y > 0 && y < field.len() - 1 && x > 0 && x < row.len() - 1 {
                if check2(&field, (x, y)) {
                    sum += 1;
                }
            }
        }
    }
    println!("{:?}", sum);
}