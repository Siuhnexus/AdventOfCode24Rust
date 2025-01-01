use std::{collections::HashMap, fmt::Debug, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecisionOperation {
    AND,
    OR,
    XOR
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecisionRule {
    a: String,
    b: String,
    operation: DecisionOperation,
    result: String
}

impl DecisionOperation {
    pub fn from_str(text: &str) -> DecisionOperation {
        match text {
            "AND" => DecisionOperation::AND,
            "OR" => DecisionOperation::OR,
            "XOR" => DecisionOperation::XOR,
            _ => panic!("String does not correspond to an operation")
        }
    }

    pub fn execute(&self, a: bool, b: bool) -> bool {
        match self {
            DecisionOperation::AND => a & b,
            DecisionOperation::OR => a | b,
            DecisionOperation::XOR => a ^ b
        }
    }
}
impl DecisionRule {
    pub fn try_execute(&self, vars: &mut HashMap<String, Option<bool>>) -> bool {
        if let (Some(Some(first)), Some(Some(second))) = (vars.get(&self.a), vars.get(&self.b)) {
            *vars.entry(self.result.clone()).or_insert(None) = Some(self.operation.execute(*first, *second));
            return true;
        }
        return false;
    }
    pub fn equation_equals(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a) && self.operation == other.operation
    }
}

fn parse() -> (HashMap<String, Option<bool>>, Vec<DecisionRule>) {
    let mut vars = HashMap::new();
    let mut rules = Vec::new();
    let input = fs::read_to_string("input/24.txt").expect("Input file not found");
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();

    for line in parts[0].lines() {
        let vardef: Vec<&str> = line.split(": ").collect();
        if vardef.len() != 2 { panic!("Wrong format"); }
        vars.insert(String::from(vardef[0]), Some(vardef[1] == "1"));
    }
    for line in parts[1].lines() {
        if line.trim() == "" { continue; }
        let firstsplit: Vec<&str> = line.split(" -> ").collect();
        if firstsplit.len() != 2 { panic!("Wrong format"); }
        let secondsplit: Vec<&str> = firstsplit[0].split(" ").collect();
        if secondsplit.len() != 3 { panic!("Wrong format"); }
        let first = String::from(secondsplit[0]);
        let second = String::from(secondsplit[2]);
        let operation = DecisionOperation::from_str(secondsplit[1]);
        let target = String::from(firstsplit[1]);
        rules.push(DecisionRule { a: first.clone(), b: second.clone(), operation, result: target.clone() });
        vars.entry(first).or_insert(None);
        vars.entry(second).or_insert(None);
        vars.entry(target).or_insert(None);
    }

    (vars, rules)
}

pub fn part1() {
    let (mut vars, mut rules) = parse();
    
    let mut lastchanges = Vec::new();
    let mut changes = Vec::new();

    while rules.len() > 0 {
        for i in (0..rules.len()).rev() {
            let rule = &rules[i];
            if lastchanges.len() > 0 {
                if !lastchanges.contains(&rule.a) && !lastchanges.contains(&rule.b) { continue; }
            }
            if rule.try_execute(&mut vars) {
                changes.push(rule.result.clone());
                rules.remove(i);
            }
        }
        lastchanges = changes;
        changes = Vec::new();
    }
    let mut zs = vars.into_iter()
    .filter(|(k, _)| k.starts_with('z'))
    .collect::<Vec<_>>();
    zs.sort_by_key(|(k, _)| k.clone());
    let numstr = zs.into_iter().rev().fold(String::new(), |prev, (_, v)| prev + match v.unwrap() { true => "1", false => "0" });
    println!("{}", usize::from_str_radix(numstr.as_str(), 2).unwrap());
}

#[derive(Clone, Debug, PartialEq)]
enum RuleMatch {
    Ok,
    Faulty(DecisionRule, DecisionRule),
    None
}

fn make_atomic_rule(place: usize, op: DecisionOperation) -> DecisionRule {
    let formatted = format!("{:02}", place);
    DecisionRule { a: String::from("x") + &formatted, b: String::from("y") + &formatted, operation: op, result: String::new() }
}

fn is_atomic_sum(place: usize, pivot: &DecisionRule) -> RuleMatch {
    match pivot.equation_equals(&make_atomic_rule(place, DecisionOperation::XOR)) {
        true => RuleMatch::Ok,
        false => RuleMatch::None
    }
}
fn is_atomic_carryover(place: usize, pivot: &DecisionRule) -> RuleMatch {
    match pivot.equation_equals(&make_atomic_rule(place, DecisionOperation::AND)) {
        true => RuleMatch::Ok,
        false => RuleMatch::None
    }
}

type Atomic = dyn Fn(&DecisionRule) -> RuleMatch;
type Derived<'a> = dyn FnMut(&'a DecisionRule, &'a HashMap<&str, &DecisionRule>, &'a Vec<DecisionRule>, &mut HashMap<(usize, DecisionRule), RuleMatch>) -> RuleMatch;

fn match_derived<'a>(atomic: &Atomic, derived: &mut Derived<'a>, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &mut HashMap<(usize, DecisionRule), RuleMatch>) -> RuleMatch {
    let (mut firstmatch, mut secondmatch) = (RuleMatch::None, RuleMatch::None);
    if let Some(a) = target_rules.get(pivot.a.as_str()) { firstmatch = atomic(a); }
    if let Some(b) = target_rules.get(pivot.b.as_str()) { secondmatch = atomic(b); }
    let left = match (firstmatch, secondmatch) {
        (RuleMatch::None, RuleMatch::None) => {
            let (mut thirdmatch, mut fourthmatch) = (RuleMatch::None, RuleMatch::None);
            if let Some(a) = target_rules.get(pivot.a.as_str()) { thirdmatch = derived(a, target_rules, rules, memo); }
            if let Some(b) = target_rules.get(pivot.b.as_str()) { fourthmatch = derived(b, target_rules, rules, memo); }
            let left = match (thirdmatch, fourthmatch) {
                (RuleMatch::Ok, _) => {
                    pivot.b.as_str()
                }
                (_, RuleMatch::Ok) => {
                    pivot.a.as_str()
                }
                _ => {
                    return RuleMatch::None;
                }
            };
            let replacement = rules.iter().find(|rule| atomic(rule) == RuleMatch::Ok);
            match replacement {
                None => return RuleMatch::None,
                Some(v) => return RuleMatch::Faulty((*target_rules.get(left).unwrap()).clone(), v.clone())
            }
        },
        (RuleMatch::Ok, _) => {
            pivot.b.as_str()
        },
        (_, RuleMatch::Ok) => {
            pivot.a.as_str()
        }
        _ => panic!("This result is impossible for atomic operations")
    };
    let mut other_match = None;
    if let Some(v) = target_rules.get(left) { other_match = Some(derived(v, target_rules, rules, memo)) }
    match other_match {
        None | Some(RuleMatch::None) => {
            let replacement = rules.iter().find(|rule| derived(rule, target_rules, rules, memo) == RuleMatch::Ok);
            match replacement {
                None => return RuleMatch::None,
                Some(v) => return RuleMatch::Faulty((*target_rules.get(left).unwrap()).clone(), v.clone())
            }
        },
        Some(v) => return v
    }
}

fn is_snowball<'a>(place: usize, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &'a DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &mut HashMap<(usize, DecisionRule), RuleMatch>) -> RuleMatch {
    if pivot.operation != DecisionOperation::AND { return RuleMatch::None; }
    match_derived(&move |pivot| is_atomic_sum(place, pivot), &mut move |pivot, tr, r, memo| is_carryover(place - 1, pivot, tr, r, memo), pivot, target_rules, rules, memo)
}
fn is_carryover<'a>(place: usize, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &'a DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &mut HashMap<(usize, DecisionRule), RuleMatch>) -> RuleMatch {
    let key = (place, pivot.clone());
    if memo.contains_key(&key) { return memo.get(&key).unwrap().clone(); }
    if place == 0 { let result = is_atomic_carryover(0, pivot); memo.insert((place, pivot.clone()), result.clone()); return result; }
    if pivot.operation != DecisionOperation::OR { memo.insert((place, pivot.clone()), RuleMatch::None); return RuleMatch::None; }
    let result = match_derived(&move |pivot| is_atomic_carryover(place, pivot), &mut move |pivot, tr, r, memo| is_snowball(place, pivot, tr, r, memo), pivot, target_rules, rules, memo);
    memo.insert((place, pivot.clone()), result.clone());
    result
}
fn is_sum<'a>(place: usize, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &'a DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &mut HashMap<(usize, DecisionRule), RuleMatch>) -> RuleMatch {
    if place == 0 { return is_atomic_sum(0, pivot) }
    if pivot.operation != DecisionOperation::XOR { return RuleMatch::None; }
    match_derived(&move |pivot| is_atomic_sum(place, pivot), &mut move |pivot, tr, r, memo| is_carryover(place - 1, pivot, tr, r, memo), pivot, target_rules, rules, memo)
}

fn swap_faulty(first: &DecisionRule, second: &DecisionRule, faulty: &mut Vec<DecisionRule>, replacements: &mut Vec<DecisionRule>) {
    let mut newfirst = first.clone();
    newfirst.result = second.result.clone();
    let mut newsecond = second.clone();
    newsecond.result = first.result.clone();
    faulty.push(first.clone());
    faulty.push(second.clone());
    replacements.push(newfirst);
    replacements.push(newsecond);
}

fn collect_faultys<'a, 'b: 'a>(matcher: &mut Derived<'a>, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &'a DecisionRule>, rules: &'b Vec<DecisionRule>, memo: &mut HashMap<(usize, DecisionRule), RuleMatch>) -> (Vec<DecisionRule>, Vec<String>) {
    let mut faulty = Vec::new();
    let mut replacements = Vec::new();
    match matcher(pivot, target_rules, rules, memo) {
        RuleMatch::Ok => {},
        RuleMatch::Faulty(rule, replacement) => {
            swap_faulty(&rule, &replacement, &mut faulty, &mut replacements);
        },
        RuleMatch::None => {
            let betterfit = rules.iter().find(|rule| matcher(rule, target_rules, rules, memo) != RuleMatch::None).unwrap();
            swap_faulty(pivot, betterfit, &mut faulty, &mut replacements);
            if let RuleMatch::Faulty(rule, replacement) = matcher(betterfit, target_rules, rules, memo) {
                swap_faulty(&rule, &replacement, &mut faulty, &mut replacements);
            }
        },
    }
    let mut result = Vec::new();
    let mut newrules = Vec::new();
    for i in (0..rules.len()).rev() {
        if !faulty.contains(&rules[i]) {
            newrules.push(rules[i].clone());
        }
    }
    for replacement in replacements {
        result.push(replacement.result.clone());
        newrules.push(replacement);
    }
    (newrules, result)
}

fn rules_by_target<'a>(rules: &'a Vec<DecisionRule>) -> HashMap<&'a str, &'a DecisionRule> {
    let mut result = HashMap::new();
    for rule in rules {
        result.insert(rule.result.as_str(), rule);
    }
    result
}

pub fn part2() {
    let (vars, mut rules) = parse();
    let mut target_rules = rules_by_target(&rules);
    let input_size = vars.iter().filter(|(k, _)| k.starts_with("x") || k.starts_with("y")).count() / 2;
    let mut memo = HashMap::new();
    let mut faulty = Vec::new();

    for i in 0..input_size {
        println!("{i}");
        let place_rule = target_rules.get((String::from("z") + &format!("{:02}", i)).as_str()).unwrap();
        let (newrules, mut result) = collect_faultys(&mut move |pivot, tr, r, memo| is_sum(i, pivot, tr, r, memo), place_rule, &target_rules, &rules, &mut memo);
        faulty.append(&mut result);
        rules = newrules;
        target_rules = rules_by_target(&rules);
    }

    let remainder = target_rules.get((String::from("z") + &format!("{:02}", input_size)).as_str()).unwrap();
    let (newrules, mut result) = collect_faultys(&mut move |pivot, tr, r, memo| is_carryover(input_size - 1, pivot, tr, r, memo), remainder, &target_rules, &rules, &mut memo);
    faulty.append(&mut result);
    rules = newrules;
    target_rules = rules_by_target(&rules);

    faulty.sort();
    println!("{}", faulty.join(","));
}