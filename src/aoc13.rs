use std::fs;

#[derive(Debug)]
struct SlotMachine {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32)
}

fn parse() -> Vec<SlotMachine> {
    let mut result = Vec::new();
    let mut ca: Option<(u32, u32)> = None;
    let mut cb: Option<(u32, u32)> = None;
    for line in fs::read_to_string("input/13.txt").expect("Input file not found").lines() {
        if line.trim() == "" { ca = None; cb = None; continue; }
        match line.starts_with("Button") {
            true => {
                let isfirst = line.chars().nth(7).expect("Input file was formatted incorrectly") == 'A';
                let shortened = line.chars().skip(12).collect::<String>();
                let numvals: Vec<&str> = shortened.split(", Y+").collect();
                if numvals.len() != 2 { panic!("Input file was formatted incorrectly") }
                if isfirst {
                    ca = Some((numvals[0].parse::<u32>().expect("Non-number in input"), numvals[1].parse::<u32>().expect("Non-number in input")))
                }
                else {
                    cb = Some((numvals[0].parse::<u32>().expect("Non-number in input"), numvals[1].parse::<u32>().expect("Non-number in input")))
                }
            },
            false => {
                let shortened = line.chars().skip(9).collect::<String>();
                let numvals: Vec<&str> = shortened.split(", Y=").collect();
                if ca.is_none() || cb.is_none() || numvals.len() != 2 { panic!("Input file was formatted incorrectly") }
                result.push(SlotMachine {
                    a: ca.unwrap(),
                    b: cb.unwrap(),
                    prize: (numvals[0].parse::<u32>().expect("Non-number in input"), numvals[1].parse::<u32>().expect("Non-number in input"))
                })
            }
        }
    }
    result
}

pub fn part1() {
    let machines = parse();
    let mut tokens: u32 = 0;
    
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
        possibilities.sort_by_key(|(a, b)| (*a as i32) * (-3) - (*b as i32));
        tokens += possibilities[0].0 as u32 * 3 + possibilities[0].1 as u32;
    }

    println!("{tokens}");
}