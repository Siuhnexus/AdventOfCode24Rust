use std::fs;
use std::ops::RangeInclusive;

fn parse() -> Vec<Vec<i32>> {
    let message: String = match fs::read_to_string("input/02.txt") {
        Err(_e) => panic!("No input file found"),
        Ok(v) => v
    };

    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in message.lines() {
        if line == "" { continue; }
        let mut report: Vec<i32> = Vec::new();
        for numtxt in line.split(" ") {
            let num: i32 = match numtxt.parse::<i32>() {
                Err(_e) => panic!("Non-number in input"),
                Ok(v) => v
            };
            report.push(num);
        }
        result.push(report);
    }
    result
}

pub fn part1() {
    let reports = parse();
    let mut safe: i32 = 0;
    for report in reports {
        let mut ascending: Option<bool> = None;
        let mut prevlevel: Option<i32> = None;
        let mut issafe = true;
        for level in report {
            if prevlevel == None { prevlevel = Some(level); continue }
            let prevlevelval = prevlevel.unwrap();
            match ascending {
                None => {
                    if level > prevlevelval {
                        ascending = Some(true);
                    }
                    else if level < prevlevelval {
                        ascending = Some(false);
                    }
                },
                Some(true) => {
                    if level < prevlevelval { issafe = false; break; }
                },
                Some(false) => {
                    if level > prevlevelval { issafe = false; break; }
                }
            }
            let diff: i32 = i32::abs(level - prevlevelval);
            if diff < 1 || diff > 3 { issafe = false; break; }
            prevlevel = Some(level);
        }
        if issafe { safe += 1; }
    }
    println!("{:?}", safe);
}

fn tryshorter(report: &Vec<i32>, totest: RangeInclusive<usize>) -> bool {
    for i in totest {
        let mut variant = report.clone();
        variant.remove(i);
        if issafe(variant, true) { return true; }
    }
    return false;
}
fn issafe(report: Vec<i32>, shortened: bool) -> bool {
    let mut ascending: Option<bool> = None;
    let mut prevlevel: Option<&i32> = None;
    for (level, i) in report.iter().zip(0..report.len()) {
        if prevlevel == None { prevlevel = Some(level); continue; }
        let prevlevelval = prevlevel.unwrap();
        match ascending {
            None => {
                if level > prevlevelval {
                    ascending = Some(true);
                }
                else if level < prevlevelval {
                    ascending = Some(false);
                }
            },
            Some(true) => {
                if level < prevlevelval {
                    if shortened { return false; }
                    if i == 2 { return tryshorter(&report, (i-2)..=i); } // When the first pair establishes a wrong direction, the first element could also be faulty
                    return tryshorter(&report, (i-1)..=i);
                }
            },
            Some(false) => {
                if level > prevlevelval {
                    if shortened { return false; }
                    if i == 2 { return tryshorter(&report, (i-2)..=i); } // When the first pair establishes a wrong direction, the first element could also be faulty
                    return tryshorter(&report, (i-1)..=i);
                }
            }
        }
        let diff: i32 = i32::abs(level - prevlevelval);
        if diff < 1 || diff > 3 {
            if shortened { return false; }
            return tryshorter(&report, (i-1)..=i);
        }
        prevlevel = Some(level);
    }
    return true;
}
pub fn part2() {
    let reports = parse();
    let mut safe: i32 = 0;
    for report in reports {
        if issafe(report, false) { safe += 1; }
    }
    println!("{:?}", safe);
}