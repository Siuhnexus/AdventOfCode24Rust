use std::fs;
use std::collections::HashMap;

pub fn parsing() -> (Vec<i32>, Vec<i32>) {
    let message: String = match fs::read_to_string("input/01.txt") {
        Err(_e) => String::from(""),
        Ok(v) => v
    };

    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in message.lines() {
        let mut i: i32 = 0;
        for part in line.split("   ") {
            let rnum = part.parse::<i32>();
            let num = match rnum {
                Err(_e) => panic!("Input text contains non-numbers"),
                Ok(v) => v
            };
            
            match i {
                0 => first.push(num),
                1 => second.push(num),
                _ => panic!("This point should be unreachable")
            }
            if i > 0 { break };
            i += 1;
        }
    }

    (first, second)
}

pub fn part1() {
    let (first, second) = parsing();

    let mut sum = 0;
    for (f, s) in first.iter().zip(second.iter())  {
        sum += i32::abs(f - s);
    }
    println!("{:?}", sum)
}

pub fn part2() {
    let (mut first, mut second) = parsing();
    first.sort();
    second.sort();

    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in second.iter() {
        if map.contains_key(num) {
            map.insert(*num, map.get(num).unwrap() + 1);
        }
        else {
            map.insert(*num, 1);
        }
    }

    let mut sum = 0;
    for num in first.iter() {
        sum += num * match map.get(num) {
            Some(v) => v,
            None => &0
        };
    }
    println!("{:?}", sum)
}