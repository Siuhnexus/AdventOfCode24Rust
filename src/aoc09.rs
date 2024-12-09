use std::{fs};

fn parse() -> Vec<Option<u16>> {
    let memory = fs::read_to_string("input/09.txt").expect("Input file not found");
    let mut result: Vec<Option<u16>> = Vec::new();
    let mut describesfile = true;
    let mut id: u16 = 0;
    for c in memory.chars() {
        if c == '\n' || c == '\r' { continue; }
        if !c.is_numeric() { panic!("Non-number found in input"); }
        if describesfile {
            result.append(&mut vec![Some(id); c.to_digit(10).expect("Char could not be converted to digit") as usize]);
            id += 1;
        }
        else {
            result.append(&mut vec![None; c.to_digit(10).expect("Char could not be converted to digit") as usize]);
        }
        describesfile = !describesfile;
    }
    result
}

fn printresult(result: Vec<Option<u16>>) {
    println!("{:?}", result.iter().fold(String::from(""), |prev: String, v| { let result = match v { None => String::from("."), Some(v) => v.to_string() }; format!("{prev}{result}")}));
    println!("{:?}", result.iter().enumerate().fold(0, |prev, (i, v)| {
        prev + (i as u64) * match v { None => 0, Some(val) => *val as u64 }
    }))
}

pub fn part1() {
    let mut result = parse();
    
    let mut j: usize = 0;
    for i in (0..result.len()).rev() {
        let f = result[i];
        match f {
            None => { continue },
            Some(v) => {
                let mut p = result[j];
                while p != None && j <= i { j += 1; p = result[j]; }
                if j > i { break; }
                result[j] = Some(v);
                result[i] = None;
            }
        }
    }

    printresult(result);
}

fn trymove(result: &mut Vec<Option<u16>>, i: usize, fid: u16, clen: u16) {
    if clen == 0 { return; }
    let mut cfree = 0;
    for j in 0..=i {
        match result[j] {
            None => {
                cfree += 1;
                if cfree == clen {
                    for k in (j - clen as usize + 1)..=j {
                        result[k] = Some(fid);
                    }
                    for k in (i + 1)..(i + 1 + clen as usize) {
                        result[k] = None;
                    }
                    break;
                }
            },
            Some(_v) => { cfree = 0; }
        }
    }
}
pub fn part2() {
    let mut result = parse();

    let mut fid = result.iter().max_by_key(|k| match k { None => 0, Some(v) => *v }).expect("Unexpected error").expect("Memory has not more than one file");
    let mut clen: u16 = 0;

    for i in (0..result.len()).rev() {
        let f = result[i];
        loop {
            match f {
                None => {
                    if clen > 0 { trymove(&mut result, i, fid, clen); fid -= 1; clen = 0; }
                    break;
                },
                Some(v) => {
                    if v == fid {
                        clen += 1;
                        break;
                    }
                    else {
                        if clen > 0 { trymove(&mut result, i, fid, clen); fid -= 1; clen = 0; } else { break; }
                    }
                }
            }
        }
    }

    printresult(result);
}