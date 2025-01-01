use std::{collections::HashMap, fmt::Debug, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecisionOperation {
    AND,
    OR,
    XOR
}
#[derive(Debug, Clone)]
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
}

impl PartialEq for DecisionRule {
    fn eq(&self, other: &Self) -> bool {
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
enum RuleMatch<'a> {
    Ok,
    Faulty(&'a DecisionRule, &'a DecisionRule),
    None
}

fn make_atomic_rule(place: usize, op: DecisionOperation) -> DecisionRule {
    let formatted = format!("{:02}", place);
    DecisionRule { a: String::from("x") + &formatted, b: String::from("y") + &formatted, operation: op, result: String::new() }
}

fn is_atomic_sum(place: usize, pivot: &DecisionRule) -> RuleMatch {
    match *pivot == make_atomic_rule(place, DecisionOperation::XOR) {
        true => RuleMatch::Ok,
        false => RuleMatch::None
    }
}
fn is_atomic_carryover(place: usize, pivot: &DecisionRule) -> RuleMatch {
    match *pivot == make_atomic_rule(place, DecisionOperation::AND) {
        true => RuleMatch::Ok,
        false => RuleMatch::None
    }
}

fn carry_out<'a, 'b>(pivot: &DecisionRule, first: &'b mut CurriedDerived<'a>, second: &'b mut CurriedDerived<'a>, target_rules: &'a HashMap<&str, &DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &'a mut HashMap<(usize, &'a DecisionRule), RuleMatch<'a>>) -> RuleMatch<'a> {
    let mut result: RuleMatch<'a> = RuleMatch::Ok;
    let processor = |match_result: &RuleMatch<'a>, matches: &mut bool, result: &mut RuleMatch<'a>| -> Option<RuleMatch> {
        match match_result {
            RuleMatch::Ok => {
                *matches = true;
            },
            RuleMatch::Faulty(_, _) => {
                if *result != RuleMatch::Ok { return Some(RuleMatch::None); }
                *matches = true;
                *result = match_result.clone();
            },
            RuleMatch::None => {
                if *result != RuleMatch::Ok { return Some(RuleMatch::None); }
            }
        };
        None
    };
    let mut consumed = None;

    let mut first_match = false;
    if let Some(a) = target_rules.get(pivot.a.as_str()) {
        if let Some(v) = processor(&first(a, target_rules, rules, memo), &mut first_match, &mut result) { return v; };
        if first_match { consumed = Some(false); }
        else {
            if let Some(v) = processor(&second(a, target_rules, rules, memo), &mut first_match, &mut result) { return v; };
            if first_match { consumed = Some(true); }
        }
    }

    let mut second_match = false;
    if let Some(b) = target_rules.get(pivot.b.as_str()) {
        if let Some(false) = consumed {} else {
            if let Some(v) = processor(&first(b, target_rules, rules, memo), &mut second_match, &mut result) { return v; };
        }
        if second_match { consumed = Some(false); }
        else {
            if let Some(true) = consumed {} else {
                if let Some(v) = processor(&second(b, target_rules, rules, memo), &mut second_match, &mut result) { return v; };
                if second_match { consumed = Some(true); }
            }
        }
    }
    
    match (first_match, second_match) {
        (true, true) => result,
        (false, false) => RuleMatch::None,
        (true, false) => {
            let nonconsumed = if consumed.unwrap() { first } else { second };
            println!("Now finding new match for {pivot:?}");
            match rules.iter().find(move |rule| nonconsumed(rule, target_rules, rules, memo) == RuleMatch::Ok) {
                Some(v) => RuleMatch::Faulty(target_rules.get(pivot.b.as_str()).unwrap(), v),
                None => RuleMatch::None
            }
        },
        (false, true) => {
            let nonconsumed = if consumed.unwrap() { first } else { second };
            println!("Now finding new match for {pivot:?}");
            match rules.iter().find(|rule| nonconsumed(rule, target_rules, rules, memo) == RuleMatch::Ok) {
                Some(v) => RuleMatch::Faulty(target_rules.get(pivot.a.as_str()).unwrap(), v),
                None => RuleMatch::None
            }
        }
    }
}
type CurriedDerived<'a> = dyn FnMut(&'a DecisionRule) -> RuleMatch<'a>;
fn is_snowball<'a>(place: usize, pivot: &'a DecisionRule, target_rules: &'a HashMap<&'a str, &'a DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &'a mut HashMap<(usize, &'a DecisionRule), RuleMatch<'a>>) -> RuleMatch<'a> {
    if pivot.operation != DecisionOperation::AND { return RuleMatch::None; }
    carry_out(pivot, &mut move |pivot| is_atomic_sum(place, pivot), &mut |pivot| is_carryover(place - 1, pivot, target_rules, rules, memo), target_rules, rules, memo)
}
fn is_carryover<'a>(place: usize, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &'a mut HashMap<(usize, &'a DecisionRule), RuleMatch<'a>>) -> RuleMatch<'a> {
    if place == 0 { return is_atomic_carryover(0, pivot) }
    if pivot.operation != DecisionOperation::OR { return RuleMatch::None; }
    carry_out(pivot, &mut move |pivot| is_atomic_carryover(place, pivot), &mut move |pivot| is_snowball(place - 1, pivot, target_rules, rules, memo), target_rules, rules, memo)
}
fn is_sum<'a>(place: usize, pivot: &'a DecisionRule, target_rules: &'a HashMap<&str, &DecisionRule>, rules: &'a Vec<DecisionRule>, memo: &'a mut HashMap<(usize, &'a DecisionRule), RuleMatch<'a>>) -> RuleMatch<'a> {
    if place == 0 {
        is_atomic_sum(0, pivot)
    }
    else {
        if pivot.operation != DecisionOperation::XOR { return RuleMatch::None; }
        carry_out(pivot, &mut move |pivot, tr, r, memo| is_carryover(place - 1, pivot, tr, r, memo), &mut move |pivot, _, _, _| is_atomic_sum(place, pivot), target_rules, rules, memo)
    }
}

fn match_or_find<'a>(matcher: &CurriedDerived<'a>, pivot: &'a DecisionRule, tr: &'a HashMap<&str, &DecisionRule>, rules: &'a Vec<DecisionRule>) -> Vec<&'a str> {
    let mut result = Vec::new();
    match matcher(pivot, tr, rules) {
        RuleMatch::Ok => {},
        RuleMatch::None => {
            println!("{:?}", pivot);
            let betterfit = rules.iter().find(|rule| matcher(rule, tr, rules) != RuleMatch::None).unwrap();
            result.push(pivot.result.as_str());
            result.push(betterfit.result.as_str());
            if let RuleMatch::Faulty(rule, replacement) = matcher(betterfit, tr, rules) {
                result.push(rule.result.as_str());
                result.push(replacement.result.as_str());
            }
        },
        RuleMatch::Faulty(rule, replacement) => {
            result.push(rule.result.as_str());
            result.push(replacement.result.as_str());
        }
    }
    result
}

fn rules_by_target(rules: &Vec<DecisionRule>) -> HashMap<&str, &DecisionRule> {
    let mut result = HashMap::new();
    for rule in rules {
        result.insert(rule.result.as_str(), rule);
    }
    result
}

pub fn part2() {
    let (vars, rules) = parse();
    let target_rules = rules_by_target(&rules);
    let input_size = vars.iter().filter(|(k, _)| k.starts_with("x") || k.starts_with("y")).count() / 2;
    let mut faulty = Vec::new();

    let remainder = target_rules.get((String::from("z") + &format!("{:02}", input_size)).as_str()).unwrap();
    faulty.append(&mut match_or_find(&move |pivot, tr, r| is_carryover(input_size - 1, pivot, tr, r), remainder, &target_rules, &rules));

    for i in 0..input_size {
        println!("{i}");
        let place_rule = target_rules.get((String::from("z") + &format!("{:02}", i)).as_str()).unwrap();
        faulty.append(&mut match_or_find(&move |pivot, tr, r| is_sum(i, pivot, tr, r), place_rule, &target_rules, &rules));
    }

    faulty.sort();
    println!("{}", faulty.join(","));
}