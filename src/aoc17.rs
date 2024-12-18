use std::fs;

fn parse() -> (u64, u64, u64, Vec<u8>) {
    let (mut a, mut b, mut c) = (0, 0, 0);
    let mut instructions = Vec::new();

    for line in fs::read_to_string("input/17.txt").expect("Input file not found").lines() {
        if line.trim() == "" { continue; }
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 { panic!("Input was formatted incorrectly"); }
        if parts[0].contains('A') {
            a = parts[1].parse().expect("Non-number found in input");
        }
        else if parts[0].contains('B') {
            b = parts[1].parse().expect("Non-number found in input");
        }
        else if parts[0].contains('C') {
            c = parts[1].parse().expect("Non-number found in input");
        }
        else {
            instructions = parts[1].split(",").map(|s| s.parse().expect("Non-number found in input")).collect();
        }
    }

    (a, b, c, instructions)
}

fn combo(input: u8, a: &u64, b: &u64, c: &u64) -> u64 {
    match input {
        0 | 1 | 2 | 3 => input as u64,
        4 => *a,
        5 => *b,
        6 => *c,
        7 => panic!("This combo parameter can not appear in valid programs"),
        _ => panic!("This computer can only handle 3-bit integers")
    }
}
fn instruction(instruction: u8, input: u8, a: &mut u64, b: &mut u64, c: &mut u64, ip: &mut usize) -> (Vec<u8>, bool) {
    let mut output = Vec::new();
    let cinput = combo(input, a, b, c);
    let mut jumped = false;
    match instruction {
        0 => {
            *a = *a / 2_u64.pow(cinput as u32);
        },
        1 => {
            *b = *b ^ input as u64;
        },
        2 => {
            *b = cinput % 8;
        },
        3 => {
            if *a != 0 {
                *ip = input as usize;
                jumped = true;
            }
        },
        4 => {
            *b = *b ^ *c;
        },
        5 => {
            output.push((cinput % 8) as u8);
        },
        6 => {
            *b = *a / 2_u64.pow(cinput as u32);
        },
        7 => {
            *c = *a / 2_u64.pow(cinput as u32);
        },
        _ => panic!("This computer can only handle 3-bit integers")
    }
    (output, jumped)
}

fn execute(mut a: u64, mut b: u64, mut c: u64, instructions: Vec<u8>) {
    let mut ip = 0;
    let mut wholeoutput = Vec::new();
    while let Some(instr) = instructions.get(ip) {
        let (mut output, jumped) = instruction(*instr, *instructions.get(ip + 1).expect("Program had no input left to give"), &mut a, &mut b, &mut c, &mut ip);
        wholeoutput.append(&mut output);
        if !jumped { ip += 2; }
    }
    println!("{}", wholeoutput.into_iter().fold(String::new(), |prev, v| prev + "," + &v.to_string()).chars().skip(1).collect::<String>());
}

pub fn part1() {
    let (a, b, c, instructions) = parse();
    execute(a, b, c, instructions);
}

fn step(instructions: &Vec<u8>, index: usize, currenta: u64, bound: u64) -> Option<u64> {
    let next = instructions[index];

    let start = currenta * bound;
    for nexta in start..(start + bound) {
        if nexta == 0 { continue; }
        let mut ip = 0;
        let mut wholeoutput = Vec::new();
        let mut tempa = nexta;
        let mut b = 0;
        let mut c = 0;
        while let Some(instr) = instructions.get(ip) {
            let (mut toprint, jumped) = instruction(*instr, *instructions.get(ip + 1).expect("Program had no input left to give"), &mut tempa, &mut b, &mut c, &mut ip);
            wholeoutput.append(&mut toprint);
            if !jumped { ip += 2; } else { break; }
        }
        if wholeoutput.len() == 1 && wholeoutput[0] as u8 == next {
            if index == 0 {
                return Some(nexta);
            }
            let ending = step(instructions, index - 1, nexta, bound);
            if ending.is_some() { return ending }
        }
    }
    return None;
}
fn analyze(instructions: Vec<u8>) -> u64 {
    let mut bound = 0;
    for i in 0..(instructions.len() / 2) {
        let first = instructions[i * 2];
        if first == 0 {
            bound = 2_u64.pow(instructions[i * 2 + 1] as u32);
        }
    }
    let result = step(&instructions, instructions.len() - 1, 0, bound);
    if result.is_none() { panic!("Not possible") }
    result.unwrap()
}

pub fn part2() {
    let (_, _, _, instructions) = parse();
    println!("{}", analyze(instructions));
}