use std::fs;

fn parse() -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let (mut locks, mut keys) = (Vec::new(), Vec::new());
    let input = fs::read_to_string("input/25.txt").expect("Input file not found");
    for item in input.split("\r\n\r\n") {
        let mut current = [0; 5];
        for line in item.lines() {
            for (i, char) in line.chars().enumerate() {
                if char == '#' {
                    current[i] += 1;
                }
            }
        }
        for i in 0..5 { current[i] -= 1 }
        if item.starts_with("#") {
            locks.push(current);
        }
        else {
            keys.push(current);
        }
    }
    (locks, keys)
}

pub fn part1() {
    let max_height = 5;
    let (locks, keys) = parse();
    let mut possible = 0;
    for lock in &locks {
        'keyloop: for key in &keys {
            for i in 0..5 {
                if lock[i] + key[i] > max_height { continue 'keyloop; }
            }
            possible += 1;
        }
    }
    println!("{possible}");
}