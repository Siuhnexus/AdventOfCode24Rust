use std::{fs, collections::HashMap, cmp::Ordering, time::Instant};

fn parse() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut secondphase = false;
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut pages: Vec<Vec<u32>> = Vec::new();

    let infos = match fs::read_to_string("input/05.txt") { Err(_e) => panic!("No input file found."), Ok(v) => v };

    for line in infos.lines() {
        if line == "" { secondphase = true; continue; }

        if !secondphase {
            let nums: Vec<u32> = match line.split("|").map(|numstring| numstring.parse::<u32>()).collect() {
                Err(_e) => panic!("Rule was not formatted correctly"),
                Ok(v) => v
            };
            if nums.len() != 2 { panic!("Rule was not formatted correctly") }
            rules.push((nums[0], nums[1]));
        }
        else {
            let nums: Vec<u32> = match line.split(",").map(|numstring| numstring.parse::<u32>()).collect() {
                Err(_e) => panic!("Page was not formatted correctly"),
                Ok(v) => v
            };
            pages.push(nums);
        }
    }
    (rules, pages)
}

pub fn part1() {
    let (rules, pages) = parse();

    let mut result: u32 = 0;
    
    for page in pages {
        let mut valid = true;
        for rule in &rules {
            if !page.contains(&rule.0) || !page.contains(&rule.1) { continue; }
            if page.iter().position(|val| val == &rule.0).unwrap() >= page.iter().position(|val| val == &rule.1).unwrap() {
                valid = false;
                break;
            }
        }
        if valid {
            result += page[page.len() / 2];
        }
    }

    println!("{:?}", result);
}

fn addtomap(map: &mut HashMap<u32, HashMap<u32, Ordering>>, vals: &(u32, u32), order: Ordering) {
    match map.get_mut(&vals.0) {
        None => { let mut newmap = HashMap::new(); newmap.insert(vals.1, order); map.insert(vals.0, newmap); },
        Some(v) => { v.insert(vals.1, order); }
    }
}
fn addbidirectional(map: &mut HashMap<u32, HashMap<u32, Ordering>>, vals: &(u32, u32)) {
    addtomap(map, vals, Ordering::Less);
    addtomap(map, &(vals.1, vals.0), Ordering::Greater)
}
fn sortfaster(rules: &Vec<(u32, u32)>, page: &mut Vec<u32>) {
    let mut activerules: HashMap<u32, HashMap<u32, Ordering>> = HashMap::new();
    for rule in rules {
        if page.contains(&rule.0) && page.contains(&rule.1) {
            addbidirectional(&mut activerules, rule);
        }
    }
    page.sort_by(|a, b| {
        return match activerules.get(a) { None => Ordering::Equal, Some(v) => match v.get(b) { None => Ordering::Equal, Some(v) => v.clone() } };
    })
}
fn sort(rules: &Vec<(u32, u32)>, page: &mut Vec<u32>) {
    let mut activerules: Vec<(u32, u32)> = Vec::new();
    for rule in rules {
        if page.contains(&rule.0) && page.contains(&rule.1) {
            activerules.push(rule.clone());
        }
    }

    let mut valid = false;
    while !valid {
        valid = true;
        for rule in &activerules {
            let firstpos = page.iter().position(|val| val == &rule.0).unwrap();
            let secondpos = page.iter().position(|val| val == &rule.1).unwrap();
            if firstpos >= secondpos {
                page[firstpos] = rule.1;
                page[secondpos] = rule.0;
                valid = false;
                break;
            }
        }
    }
}
pub fn part2() {
    let timer = Instant::now();

    let (rules, pages) = parse();

    let mut result: u32 = 0;
    
    for mut page in pages {
        let mut valid = true;
        for rule in &rules {
            if !page.contains(&rule.0) || !page.contains(&rule.1) { continue; }
            if page.iter().position(|val| val == &rule.0).unwrap() >= page.iter().position(|val| val == &rule.1).unwrap() {
                valid = false;
                break;
            }
        }
        if valid { continue; }
        sortfaster(&rules, &mut page);
        result += page[page.len() / 2]
    }

    println!("{:?}", result);
    println!("{}", timer.elapsed().as_millis())
}