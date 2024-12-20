use std::{fs, collections::HashMap};

fn parse() -> (Vec<String>, Vec<String>) {
    let available;
    let needed;
    let input = fs::read_to_string("input/19.txt").expect("Input file not found");
    let sections: Vec<&str> = input.split("\r\n\r\n").collect();
    if sections.len() < 2 { panic!("Input was formatted incorrectly"); }
    available = sections[0].trim().split(", ").map(|pat| String::from(pat)).collect();
    needed = sections[1].trim().split("\r\n").map(|pat| String::from(pat)).collect();
    (available, needed)
}

fn ispossible(pattern: &str, available: &Vec<String>) -> bool {
    if pattern == "" { return true; }
    let mut i = 0;
    let mut possible = available.clone();
    while i < pattern.chars().count() && possible.len() > 0 {
        let current = pattern.chars().nth(i).unwrap();
        for j in (0..possible.len()).rev() {
            let pos = &possible[j];
            match pos.chars().nth(i) {
                None => { possible.remove(j); },
                Some(v) => {
                    if v != current { possible.remove(j); continue; }
                    if i == pos.chars().count() - 1 {
                        if ispossible(&pattern.chars().skip(i + 1).collect::<String>(), available) {
                            return true;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    false
}

pub fn part1() {
    let (available, needed) = parse();
    let mut sum = 0;
    for pattern in needed {
        if ispossible(&pattern, &available) {
            sum += 1;
        }
    }
    println!("{sum}");
}

fn possibilities(pattern: &str, available: &Vec<String>, memo: &mut HashMap<String, u64>) -> u64 {
    if pattern == "" { return 1; }
    if memo.contains_key(pattern) { return *memo.get(pattern).unwrap(); }
    let mut sum = 0;
    let mut i = 0;
    let mut possible = available.clone();
    while i < pattern.chars().count() && possible.len() > 0 {
        let current = pattern.chars().nth(i).unwrap();
        for j in (0..possible.len()).rev() {
            let pos = &possible[j];
            let c = pos.chars().nth(i).unwrap();
            if c != current { possible.remove(j); continue; }
            if i == pos.chars().count() - 1 {
                sum += possibilities(&pattern.chars().skip(i + 1).collect::<String>(), available, memo);
                possible.remove(j);
            }
        }
        i += 1;
    }
    memo.insert(String::from(pattern), sum);
    sum
}

pub fn part2() {
    let (available, needed) = parse();
    let mut sum = 0;
    let mut memo = HashMap::new();
    for pattern in needed {
        sum += possibilities(&pattern, &available, &mut memo);
    }
    println!("{sum}");
}