use std::fs;

// The following state machines are only half complete, since they do not include proper backtracking. It is fortunately not needed here, since every character in any command string is unique.

pub fn part1() {
    let memory: String = match fs::read_to_string("input/03.txt") {
        Err(_e) => panic!("No input file found"),
        Ok(v) => v
    };
    let starting = "mul(";
    let delimiter = ",";
    let ending = ")";
    let mut result: Vec<(u32, u32)> = Vec::new();
    let mut state: u8 = 0;
    let mut position = 0;
    let mut firstnum = String::from("");
    let mut secondnum = String::from("");
    for c in memory.chars() {
        loop {
            match state {
                0 => {
                    if c != starting.chars().nth(position).unwrap() {
                        let wascontinued = position > 0;
                        position = 0;
                        if wascontinued { continue; }
                        break;
                    }
                    if position == starting.len() - 1 {
                        state = 1;
                        position = 0;
                        break;
                    }
                    position += 1;
                    break;
                },
                1 => {
                    if !c.is_digit(10) {
                        if c == delimiter.chars().nth(0).unwrap() {
                            state = 2;
                            break;
                        }
                        firstnum = String::from(""); secondnum = String::from(""); state = 0;
                        continue;
                    }
                    firstnum.push(c);
                    break;
                },
                2 => {
                    if !c.is_digit(10) {
                        if c == ending.chars().nth(0).unwrap() {
                            let firstval = match firstnum.parse::<u32>() {
                                Err(_e) => panic!("Number could not be parsed for some reason"),
                                Ok(v) => v
                            };
                            let secondval = match secondnum.parse::<u32>() {
                                Err(_e) => panic!("Number could not be parsed for some reason"),
                                Ok(v) => v
                            };
                            result.push((firstval, secondval))
                        }
                        firstnum = String::from(""); secondnum = String::from(""); state = 0;
                        continue;
                    }
                    secondnum.push(c);
                    break;
                },
                _ => {}
            }
        }
    }
    println!("{:?}", result.iter().fold(0, |acc: u32, &x| -> u32 { acc + x.0 * x.1 }));
}

pub fn part2() {
    
    let memory: String = match fs::read_to_string("input/03.txt") {
        Err(_e) => panic!("No input file found"),
        Ok(v) => v
    };
    let starting = "mul(";
    let delimiter = ",";
    let ending = ")";
    let enabler = "do()";
    let disabler = "don't()";
    let mut result: Vec<(u32, u32)> = Vec::new();
    let mut state: u8 = 0;
    let mut position = 0;
    let mut firstnum = String::from("");
    let mut secondnum = String::from("");
    for c in memory.chars() {
        loop {
            match state {
                0 => {
                    if c != starting.chars().nth(position).unwrap() {
                        let wascontinued = position > 0;
                        position = 0;
                        if c == disabler.chars().nth(0).unwrap() { state = 3; continue; }
                        if wascontinued { continue; }
                        break;
                    }
                    if position == starting.len() - 1 {
                        state = 1;
                        position = 0;
                        break;
                    }
                    position += 1;
                    break;
                },
                1 => {
                    if !c.is_digit(10) {
                        if c == delimiter.chars().nth(0).unwrap() {
                            state = 2;
                            break;
                        }
                        firstnum = String::from(""); secondnum = String::from(""); state = 0;
                        continue;
                    }
                    firstnum.push(c);
                    break;
                },
                2 => {
                    if !c.is_digit(10) {
                        if c == ending.chars().nth(0).unwrap() {
                            let firstval = match firstnum.parse::<u32>() {
                                Err(_e) => panic!("Number could not be parsed for some reason"),
                                Ok(v) => v
                            };
                            let secondval = match secondnum.parse::<u32>() {
                                Err(_e) => panic!("Number could not be parsed for some reason"),
                                Ok(v) => v
                            };
                            result.push((firstval, secondval))
                        }
                        firstnum = String::from(""); secondnum = String::from(""); state = 0;
                        continue;
                    }
                    secondnum.push(c);
                    break;
                },
                3 => {
                    if c != disabler.chars().nth(position).unwrap() {
                        state = 0;
                        position = 0;
                        continue;
                    }
                    if position == disabler.len() - 1 {
                        state = 4;
                        position = 0;
                        break;
                    }
                    position += 1;
                    break;
                },
                4 => {
                    if c != enabler.chars().nth(position).unwrap() {
                        let wascontinued = position > 0;
                        position = 0;
                        if wascontinued { continue };
                        break;
                    }
                    if position == enabler.len() - 1 {
                        state = 0;
                        position = 0;
                        break;
                    }
                    position += 1;
                    break;
                },
                _ => {}
            }
        }
    }
    println!("{:?}", result.iter().fold(0, |acc: u32, &x| -> u32 { acc + x.0 * x.1 }));
}