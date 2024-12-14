use std::fs;

use crate::helpers::{lcm, Wrapped};

#[derive(Debug)]
struct SlotMachine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64)
}

fn parse() -> Vec<SlotMachine> {
    let mut result = Vec::new();
    let mut ca: Option<(u64, u64)> = None;
    let mut cb: Option<(u64, u64)> = None;
    for line in fs::read_to_string("input/13.txt").expect("Input file not found").lines() {
        if line.trim() == "" { ca = None; cb = None; continue; }
        match line.starts_with("Button") {
            true => {
                let isfirst = line.chars().nth(7).expect("Input file was formatted incorrectly") == 'A';
                let shortened = line.chars().skip(12).collect::<String>();
                let numvals: Vec<&str> = shortened.split(", Y+").collect();
                if numvals.len() != 2 { panic!("Input file was formatted incorrectly") }
                if isfirst {
                    ca = Some((numvals[0].parse::<u64>().expect("Non-number in input"), numvals[1].parse::<u64>().expect("Non-number in input")))
                }
                else {
                    cb = Some((numvals[0].parse::<u64>().expect("Non-number in input"), numvals[1].parse::<u64>().expect("Non-number in input")))
                }
            },
            false => {
                let shortened = line.chars().skip(9).collect::<String>();
                let numvals: Vec<&str> = shortened.split(", Y=").collect();
                if ca.is_none() || cb.is_none() || numvals.len() != 2 { panic!("Input file was formatted incorrectly") }
                result.push(SlotMachine {
                    a: ca.unwrap(),
                    b: cb.unwrap(),
                    prize: (numvals[0].parse::<u64>().expect("Non-number in input"), numvals[1].parse::<u64>().expect("Non-number in input"))
                })
            }
        }
    }
    result
}

pub fn part1() {
    let machines = parse();
    let mut tokens: u64 = 0;

    for machine in machines {
        let mut possibilities: Vec<(u8, u8)> = Vec::new();
        for i in 1..101 {
            let firstx = machine.a.0 * i;
            let firsty = machine.a.1 * i;
            for j in 1..101 {
                if firstx + machine.b.0 * j == machine.prize.0 && firsty + machine.b.1 * j == machine.prize.1 {
                    possibilities.push((i as u8, j as u8));
                }
            }
        }

        if possibilities.len() == 0 { continue; }
        possibilities.sort_by_key(|(a, b)| (*a as i32) * 3 + (*b as i32));
        tokens += possibilities[0].0 as u64 * 3 + possibilities[0].1 as u64;
    }

    println!("{tokens}");
}

pub fn part2() {
    let machines = parse();
    let mut tokens: u64 = 0;

    for machine in machines {
        let (a, b, prize) = (machine.a, machine.b, (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000));

        let mut possibilities: Vec<(u64, u64)> = Vec::new();
        
        let xtarget = Wrapped::from(prize.0, a.0);
        let ytarget = Wrapped::from(prize.1, a.1);
        
        let xfirstlineup = match (Wrapped { value: 0, max: xtarget.max }).additions_until_target(b.0, xtarget.value) {
            None => continue,
            Some(v) => v
        };
        let yfirstlineup = match (Wrapped { value: 0, max: ytarget.max }).additions_until_target(b.1,ytarget.value) {
            None => continue,
            Some(v) => v
        };
        let xroundtrip = xtarget.additions_until_loop(b.0);
        let yroundtrip = ytarget.additions_until_loop(b.1);
        // a + kb = c + jd
        // a - c + kb = jd
        // (a - c + kb) / d = j
        let mut firstlineup = if xfirstlineup > yfirstlineup {
            match Wrapped::from(xfirstlineup - yfirstlineup, yroundtrip).additions_until_target(xroundtrip, 0) {
                None => continue,
                Some(v) => xfirstlineup + v * xroundtrip
            }
        }
        else {
            match Wrapped::from(yfirstlineup - xfirstlineup, xroundtrip).additions_until_target(yroundtrip, 0) {
                None => continue,
                Some(v) => yfirstlineup + v * yroundtrip
            }
        };
        let roundtrip = lcm(xroundtrip, yroundtrip);
        let mut current = (b.0 * firstlineup, b.1 * firstlineup);
        if current.0 > prize.0 || current.1 > prize.1 { continue; }
        let mut diffdiff = (b.0 * roundtrip / a.0) as i64 - (b.1 * roundtrip / a.1) as i64;
        let mut diff = ((prize.0 - current.0) / a.0) as i64 - ((prize.1 - current.1) / a.1) as i64;
        if diff != 0 {
            if diff.signum() != diffdiff.signum() { continue; }
            diff = diff.abs();
            diffdiff = diffdiff.abs();
            if diffdiff > diff || diff % diffdiff != 0 { continue; }
            firstlineup += (diff / diffdiff) as u64 * roundtrip;
            current = (b.0 * firstlineup, b.1 * firstlineup);
            if current.0 > prize.0 || current.1 > prize.1 { continue; }
        }

        let left = (prize.0 - current.0) / a.0;
        possibilities.push((left, firstlineup));
        
        if diffdiff == 0 {
            let lastlineup = firstlineup + ((prize.0 - current.0) / (b.0 * roundtrip)) * roundtrip;
            possibilities.push(((prize.0 - b.0 * lastlineup) / a.0, lastlineup));
        }

        if possibilities.len() == 0 { continue; }
        possibilities.sort_by_key(|(a, b)| (*a as i64) * 3 + (*b as i64));
        tokens += possibilities[0].0 as u64 * 3 + possibilities[0].1 as u64;
    }

    println!("{tokens}");
}