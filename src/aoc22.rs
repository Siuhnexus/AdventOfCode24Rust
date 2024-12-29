use std::{collections::{HashMap, HashSet}, fs};

fn parse() -> Vec<u64> {
    let mut result = Vec::new();
    for line in fs::read_to_string("input/22.txt").expect("Input file not found").lines() {
        if line.trim() == "" { continue; }
        result.push(line.parse::<u64>().expect("Non-number found in input"));
    }
    result
}

fn mix(secret: &mut u64, other: u64) {
    *secret = *secret ^ other;
}
fn prune(secret: &mut u64) {
    *secret = *secret % 16777216;
}
fn mix_and_prune(secret: &mut u64, other: u64) {
    mix(secret, other);
    prune(secret);
}

fn gennew(secret: &mut u64) {
    mix_and_prune(secret, *secret * 64);
    mix_and_prune(secret, *secret / 32);
    mix_and_prune(secret, *secret * 2048);
}

pub fn part1() {
    let nums = parse();
    let mut sum = 0;
    for num in nums {
        let mut secret = num;
        for _ in 0..2000 {
            gennew(&mut secret);
        }
        sum += secret;
    }
    println!("{sum}");
}

fn price(secret: u64) -> u8 {
    (secret % 10) as u8
}


struct ConstrainedValues {
    values: Vec<i8>,
    possible: Vec<u8>
}
fn pattern_iterator(seq_len: u8) -> Vec<ConstrainedValues> {
    if seq_len == 1 {
        return (-9..=9).into_iter().map(|v| ConstrainedValues { values: vec![v], possible: (0..=9).filter(|pos| *pos as i8 - v >= 0 && *pos as i8 - v <= 9).collect() }).collect()
    }
    else {
        let prev = pattern_iterator(seq_len - 1);
        let mut current = Vec::new();
        for pattern in prev {
            for pivot in -9..=9 {
                let mut still_possible = Vec::new();
                for possible in pattern.possible.iter() {
                    let transformed = *possible as i8 + pivot;
                    if transformed >= 0 && transformed <= 9 {
                        still_possible.push(transformed as u8);
                    }
                }
                if still_possible.len() > 0 {
                    let mut newpattern = pattern.values.clone();
                    newpattern.push(pivot);
                    current.push(ConstrainedValues{ values: newpattern, possible: still_possible });
                }
            }
        }
        current
    }
}

pub fn part2_bruteforce() {
    let mut prices = Vec::new();
    let mut price_changes = Vec::new();
    let nums = parse();
    for num in nums {
        let mut sequence = vec![price(num)];
        let mut change_sequence = Vec::new();
        let mut secret = num;
        for _ in 0..2000 {
            gennew(&mut secret);
            change_sequence.push(price(secret) as i8 - *sequence.last().unwrap() as i8);
            sequence.push(price(secret));
        }
        prices.push(sequence);
        price_changes.push(change_sequence);
    }

    let mut change_sequences = Vec::new();
    for change_sequence in price_changes.iter() {
        let mut sequences = Vec::new();
        for k in 3..2000 {
            sequences.push(vec![change_sequence[k - 3], change_sequence[k - 2], change_sequence[k - 1], change_sequence[k]])
        }
        change_sequences.push(sequences);
    }

    let mut bestsum = 0;
    let patterns = pattern_iterator(4);
    for p in 0..patterns.len() {
        let pattern = &patterns[p];
        if pattern.possible.iter().all(|v| *v < 3) { continue; }
        let mut sum = 0;
        for i in 0..prices.len() {
            let price_sequence = &prices[i];
            let change_sequence = &change_sequences[i];

            'outer: for j in 3..2000 {
                if pattern.values != change_sequence[j - 3] { continue 'outer; }
                sum += price_sequence[j + 1] as u32;
                break;
            }
        }
        if sum > bestsum { bestsum = sum; }
        println!("{} out of {} finished", p + 1, patterns.len());
    }
    println!("{bestsum}");
}

pub fn part2() {
    let mut prices = Vec::new();
    let mut price_changes = Vec::new();
    let nums = parse();
    for num in nums {
        let mut sequence = vec![price(num)];
        let mut change_sequence = Vec::new();
        let mut secret = num;
        for _ in 0..2000 {
            gennew(&mut secret);
            change_sequence.push(price(secret) as i8 - *sequence.last().unwrap() as i8);
            sequence.push(price(secret));
        }
        prices.push(sequence);
        price_changes.push(change_sequence);
    }

    let mut change_sequences = Vec::new();
    for change_sequence in price_changes.iter() {
        let mut sequences = Vec::new();
        for k in 3..2000 {
            sequences.push(vec![change_sequence[k - 3], change_sequence[k - 2], change_sequence[k - 1], change_sequence[k]])
        }
        change_sequences.push(sequences);
    }

    let mut values = HashMap::new();
    for i in 0..prices.len() {
        let price_sequence = &prices[i];
        let change_sequence = &change_sequences[i];

        let mut recorded = HashSet::new();
        for j in 3..2000 {
            let seq = &change_sequence[j - 3];
            if recorded.contains(seq) { continue; }
            *values.entry(seq.clone()).or_insert(0) += price_sequence[j + 1] as u32;
            recorded.insert(seq.clone());
        }
    }
    println!("{}", values.iter().map(|(_, v)| *v).max().unwrap());
}