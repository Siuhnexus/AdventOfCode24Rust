use std::fs;

fn parse() -> Vec<(u64, Vec<u64>)> {
    let equations: String = fs::read_to_string("input/07.txt").expect("Input file not found");

    let mut result: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in equations.lines() {
        if line.trim() == "" { continue; }
        let sides: Vec<&str> = line.split(": ").collect();
        if sides.len() != 2 { panic!("Equations were given in wrong format"); }
        result.push((sides[0].parse::<u64>().expect("Non-number found in equation"), sides[1].split(" ").map(|numstr| -> u64 { numstr.parse::<u64>().expect("Non-number found in equation") }).collect()));
    }
    result
}

fn add(a: u64, b: u64) -> u64 { a + b }
fn mult(a: u64, b: u64) -> u64 { a * b }
const U10: u64 = 10;
fn concat(a: u64, b: u64) -> u64 { a * U10.pow(((b as f32).log10().floor() as u32) + 1) + b }

pub fn ispossible(numbers: &mut Vec<u64>, target: u64, operators: &Vec<fn(u64, u64) -> u64>) -> bool {
    if numbers.len() == 1 { return numbers[0] == target; }
    let first = numbers[0];
    let second = numbers[1];
    for op in operators {
        let result = op(first, second);
        if result <= target {
            numbers.splice(0..=1, [result]);
            if ispossible(numbers, target, operators) { return true; }
            numbers.splice(0..=0, [first, second]);
        }
    }
    return false;
}

pub fn part1() {
    let equations = parse();

    let mut possible: u64 = 0;
    for mut eq in equations {
        if ispossible(&mut eq.1, eq.0, &Vec::from([add, mult])) { possible += eq.0; };
    }

    println!("{:?}", possible);
}

pub fn part2() {
    let equations = parse();

    let mut possible: u64 = 0;
    for mut eq in equations {
        if ispossible(&mut eq.1, eq.0, &Vec::from([add, mult, concat])) { possible += eq.0; };
    }

    println!("{:?}", possible);
}