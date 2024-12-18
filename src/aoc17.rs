use std::fs;

fn parse() -> (u32, u32, u32, Vec<u8>) {
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

fn combo(input: u8, a: &u32, b: &u32, c: &u32) -> u32 {
    match input {
        0 | 1 | 2 | 3 => input as u32,
        4 => *a,
        5 => *b,
        6 => *c,
        7 => panic!("This combo parameter can not appear in valid programs"),
        _ => panic!("This computer can only handle 3-bit integers")
    }
}
fn instruction(instruction: u8, input: u8, a: &mut u32, b: &mut u32, c: &mut u32, ip: &mut usize) -> (Vec<u32>, bool) {
    let mut output = Vec::new();
    let cinput = combo(input, a, b, c);
    let mut jumped = false;
    match instruction {
        0 => {
            *a = *a / 2_u32.pow(cinput);
        },
        1 => {
            *b = *b ^ input as u32;
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
            output.push(cinput % 8);
        },
        6 => {
            *b = *a / 2_u32.pow(cinput);
        },
        7 => {
            *c = *a / 2_u32.pow(cinput);
        },
        _ => panic!("This computer can only handle 3-bit integers")
    }
    (output, jumped)
}

fn execute(mut a: u32, mut b: u32, mut c: u32, instructions: Vec<u8>) {
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