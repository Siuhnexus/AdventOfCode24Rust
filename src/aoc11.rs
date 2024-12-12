use std::{fs, collections::HashMap};
 
fn parse() -> Vec<u64> {
    let stones = fs::read_to_string("input/11.txt").expect("Input file not found");
    let mut result = Vec::new();
    for line in stones.lines() {
        if line.trim() == "" { continue; }
        result.append(&mut line.split_whitespace().map(|s| s.parse::<u64>().expect("Non-number found in input")).collect());
    }
    result
}

pub fn part1() {
    let mut stones = parse();
    for _n in 0..75 {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 { stones[i] = 1; i += 1; continue; }
            let numlen = (stone as f64).log10().floor() as u8;
            if numlen % 2 == 1 {
                let cuttable = stone.to_string();
                let mut first = String::new();
                let mut second = String::new();
                for (j, c) in cuttable.chars().enumerate() {
                    if j < cuttable.len() / 2 {
                        first += &c.to_string();
                    }
                    else {
                        second += &c.to_string();
                    }
                }
                stones.splice(i..=i, [first.parse::<u64>().expect("Splitting produced an unexpected error"), second.parse::<u64>().expect("Splitting produced an unexpected error")]);
                i += 2;
                continue;
            }
            stones[i] *= 2024;
            i += 1;
        }
    }
    println!("{}", stones.len());
}

fn splitnum(value: u64) -> (u64, u64) {
    let cuttable = value.to_string();
    let mut first = String::new();
    let mut second = String::new();
    for (j, c) in cuttable.chars().enumerate() {
        if j < cuttable.len() / 2 {
            first += &c.to_string();
        }
        else {
            second += &c.to_string();
        }
    }
    let nums = (first.parse::<u64>().expect("Splitting produced an unexpected error"), second.parse::<u64>().expect("Splitting produced an unexpected error"));
    nums
}
pub fn stonesafter(stone: u64, n: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
    match memo.get(&(stone, n)) {
        Some(v) => *v,
        None => {
            let result = if n == 0 { 1 }
            else {
                if stone == 0 { stonesafter(1, n - 1, memo) }
                else {
                    let numlen = (stone as f64).log10().floor() as u8;
                    if numlen % 2 == 1 {
                        let nums = splitnum(stone);
                        stonesafter(nums.0, n - 1, memo) + stonesafter(nums.1, n - 1, memo)
                    }
                    else { stonesafter(stone * 2024, n - 1, memo) }
                }
            };
            memo.insert((stone, n), result);
            result
        }
    }
}
pub fn part2() {
    let stones = parse();
    let mut sum: u64 = 0;
    let mut memo = HashMap::new();
    for stone in stones {
        sum += stonesafter(stone, 75, &mut memo);
    }
    println!("{}", sum);
}