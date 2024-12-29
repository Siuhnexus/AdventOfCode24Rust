use std::{collections::HashMap, fmt::Debug, fs, usize};

use crate::helpers::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum NumpadButtons {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DirectionButtons {
    Top,
    Right,
    Bottom,
    Left,
    A
}

impl NumpadButtons {
    pub const DIM: (usize, usize) = (3, 4);
    pub const IMPOSSIBLE: (usize, usize) = (0, 3);

    pub fn from_char(c: char) -> NumpadButtons {
        match c {
            '0' => NumpadButtons::Zero,
            '1' => NumpadButtons::One,
            '2' => NumpadButtons::Two,
            '3' => NumpadButtons::Three,
            '4' => NumpadButtons::Four,
            '5' => NumpadButtons::Five,
            '6' => NumpadButtons::Six,
            '7' => NumpadButtons::Seven,
            '8' => NumpadButtons::Eight,
            '9' => NumpadButtons::Nine,
            'A' => NumpadButtons::A,
            _ => panic!("Given char is not on the numpad")
        }
    }

    pub fn from_position(pos: (usize, usize)) -> NumpadButtons {
        match pos {
            (1, 3) => NumpadButtons::Zero,
            (0, 2) => NumpadButtons::One,
            (1, 2) => NumpadButtons::Two,
            (2, 2) => NumpadButtons::Three,
            (0, 1) => NumpadButtons::Four,
            (1, 1) => NumpadButtons::Five,
            (2, 1) => NumpadButtons::Six,
            (0, 0) => NumpadButtons::Seven,
            (1, 0) => NumpadButtons::Eight,
            (2, 0) => NumpadButtons::Nine,
            (2, 3) => NumpadButtons::A,
            _ => panic!("This position does not correspond to a numpad button")
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            NumpadButtons::Zero => '0',
            NumpadButtons::One => '1',
            NumpadButtons::Two => '2',
            NumpadButtons::Three => '3',
            NumpadButtons::Four => '4',
            NumpadButtons::Five => '5',
            NumpadButtons::Six => '6',
            NumpadButtons::Seven => '7',
            NumpadButtons::Eight => '8',
            NumpadButtons::Nine => '9',
            NumpadButtons::A => 'A',
        }
    }

    pub fn to_position(&self) -> (usize, usize) {
        match self {
            NumpadButtons::Zero => (1, 3),
            NumpadButtons::One => (0, 2),
            NumpadButtons::Two => (1, 2),
            NumpadButtons::Three => (2, 2),
            NumpadButtons::Four => (0, 1),
            NumpadButtons::Five => (1, 1),
            NumpadButtons::Six => (2, 1),
            NumpadButtons::Seven => (0, 0),
            NumpadButtons::Eight => (1, 0),
            NumpadButtons::Nine => (2, 0),
            NumpadButtons::A => (2, 3)
        }
    }
}

impl DirectionButtons {
    pub const DIM: (usize, usize) = (3, 2);
    pub const IMPOSSIBLE: (usize, usize) = (0, 0);

    pub fn from_position(pos: (usize, usize)) -> DirectionButtons {
        match pos {
            (1, 0) => DirectionButtons::Top,
            (2, 1) => DirectionButtons::Right,
            (1, 1) => DirectionButtons::Bottom,
            (0, 1) => DirectionButtons::Left,
            (2, 0) => DirectionButtons::A,
            _ => panic!("This position does not correspond to a button")
        }
    }

    pub fn to_position(&self) -> (usize, usize) {
        match self {
            DirectionButtons::Top => (1, 0),
            DirectionButtons::Right => (2, 1),
            DirectionButtons::Bottom => (1, 1),
            DirectionButtons::Left => (0, 1),
            DirectionButtons::A => (2, 0)
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            DirectionButtons::Top => '^',
            DirectionButtons::Right => '>',
            DirectionButtons::Bottom => 'v',
            DirectionButtons::Left => '<',
            DirectionButtons::A => 'A'
        }
    }

    pub fn to_direction(&self) -> Direction {
        match self {
            DirectionButtons::Top => Direction::Top,
            DirectionButtons::Right => Direction::Right,
            DirectionButtons::Bottom => Direction::Bottom,
            DirectionButtons::Left => Direction::Left,
            DirectionButtons::A => panic!("The A button does not correspond to a direction")
        }
    }
}

fn printdirs(dirs: &Vec<DirectionButtons>) {
    println!("{}", dirs.iter().fold(String::new(), |prev, b| prev + &b.to_char().to_string()));
}

fn parse() -> Vec<Vec<NumpadButtons>> {
    let mut result = Vec::new();
    for code in fs::read_to_string("input/21.txt").expect("Input file not found").lines() {
        if code.trim() == "" { continue; }
        let mut current = Vec::new();
        for c in code.chars() {
            current.push(NumpadButtons::from_char(c))
        }
        result.push(current);
    }
    result
}

fn dir_from_signed(signed: i32, (negative, positive): (DirectionButtons, DirectionButtons)) -> DirectionButtons {
    match signed {
        -1 => negative,
        1 => positive,
        _ => positive
    }
}

fn try_path_raw(from: (usize, usize), to: (usize, usize), impossible: (usize, usize), vertical: bool) -> Option<Vec<DirectionButtons>> {
    let mut result = Vec::new();
    if vertical && (from.0 == to.0 || from.1 == to.1) { return None; }
    let fromfirst: usize;
    let fromsecond: usize;
    let tofirst: usize;
    let tosecond: usize;
    let dirsfirst: (DirectionButtons, DirectionButtons);
    let dirssecond: (DirectionButtons, DirectionButtons);
    if !vertical {
        fromfirst = from.0;
        fromsecond = from.1;
        tofirst = to.0;
        tosecond = to.1;
        dirsfirst = (DirectionButtons::Left, DirectionButtons::Right);
        dirssecond = (DirectionButtons::Top, DirectionButtons::Bottom);
        if (tofirst, fromsecond) == impossible { return None; }
    }
    else {
        fromfirst = from.1;
        fromsecond = from.0;
        tofirst = to.1;
        tosecond = to.0;
        dirsfirst = (DirectionButtons::Top, DirectionButtons::Bottom);
        dirssecond = (DirectionButtons::Left, DirectionButtons::Right);
        if (fromsecond, tofirst) == impossible { return None; }
    }

    let firstdiff = tofirst as i32 - fromfirst as i32;
    let firstdir = dir_from_signed(firstdiff.signum(), dirsfirst);
    for _ in 0..(firstdiff.abs()) {
        result.push(firstdir);
    }
    let seconddiff = tosecond as i32 - fromsecond as i32;
    let seconddir = dir_from_signed(seconddiff.signum(), dirssecond);
    for _ in 0..(seconddiff.abs()) {
        result.push(seconddir);
    }
    result.push(DirectionButtons::A);
    return Some(result);
}
type TryPathMemo = HashMap<((usize, usize), (usize, usize), (usize, usize), bool), Option<Vec<DirectionButtons>>>;
fn try_path(from: (usize, usize), to: (usize, usize), impossible: (usize, usize), vertical: bool, memo: &mut TryPathMemo) -> Option<Vec<DirectionButtons>> {
    if let Some(result) = memo.get(&(from, to, impossible, vertical)) {
        return result.clone();
    }
    let result = try_path_raw(from, to, impossible, vertical);
    
    memo.insert((from, to, impossible, vertical), result.clone());
    result
}


fn try_path_num(from: (usize, usize), to: (usize, usize), vertical: bool, memo: &mut TryPathMemo) -> Option<Vec<DirectionButtons>> {
    try_path(from, to, NumpadButtons::IMPOSSIBLE, vertical, memo)
}
fn try_path_dir(from: (usize, usize), to: (usize, usize), vertical: bool, memo: &mut TryPathMemo) -> Option<Vec<DirectionButtons>> {
    try_path(from, to, DirectionButtons::IMPOSSIBLE, vertical, memo)
}

fn dirs_from_nums(code: &Vec<NumpadButtons>, previous: Vec<DirectionButtons>, previouspos: NumpadButtons, memo: &mut TryPathMemo) -> Vec<Vec<DirectionButtons>> {
    if code.len() == 0 { return vec![previous]; }

    let mut results = Vec::new();

    let lastpos = previouspos.to_position();
    let b = code.get(0).unwrap();
    let nextpos = b.to_position();
    for order in vec![false, true] {
        if let Some(buttons) = try_path_num(lastpos, nextpos, order, memo) {
            let next: Vec<DirectionButtons> = previous.iter().copied().chain(buttons).collect();
            results.append(&mut dirs_from_nums(&code.iter().copied().skip(1).collect(), next, *b, memo))
        }
    }

    results
}

fn dirs_from_dirs(possibilities: &Vec<Vec<DirectionButtons>>, previous: Vec<DirectionButtons>, previouspos: DirectionButtons, memo: &mut TryPathMemo) -> Vec<Vec<DirectionButtons>> {
    let mut results = Vec::new();

    let lastpos = previouspos.to_position();
    for pattern in possibilities {
        if pattern.len() == 0 { return vec![previous]; }

        let dir = pattern.get(0).unwrap();
        let nextpos = dir.to_position();
        for order in vec![false, true] {
            if let Some(buttons) = try_path_dir(lastpos, nextpos, order, memo) {
                let next: Vec<DirectionButtons> = previous.iter().copied().chain(buttons).collect();
                let mut pattern_results = dirs_from_dirs(&vec![pattern.iter().copied().skip(1).collect()], next, *dir, memo);
                results.append(&mut pattern_results);
            }
        }
    }

    results
}

fn retrace_dirs_dirs(controller: &Vec<DirectionButtons>) -> Vec<DirectionButtons> {
    let mut result = Vec::new();
    let mut currentpos = DirectionButtons::A.to_position();
    for b in controller {
        match b {
            DirectionButtons::A => {
                result.push(DirectionButtons::from_position(currentpos));
            },
            dir => {
                currentpos = dir.to_direction().step(currentpos).expect("Moved out of bounds");
                if currentpos == DirectionButtons::IMPOSSIBLE { panic!("Was over impossible position") }
            }
        }
    }
    result
}

fn retrace_nums_dirs(controller: &Vec<DirectionButtons>) -> Vec<NumpadButtons> {
    let mut result = Vec::new();
    let mut currentpos = NumpadButtons::A.to_position();
    for b in controller {
        match b {
            DirectionButtons::A => {
                result.push(NumpadButtons::from_position(currentpos));
            },
            dir => {
                currentpos = dir.to_direction().step(currentpos).expect("Moved out of bounds");
                if currentpos == NumpadButtons::IMPOSSIBLE { panic!("Was over impossible position") }
            }
        }
    }
    result
}

fn min_dirs(options: &mut Vec<Vec<DirectionButtons>>) -> Vec<DirectionButtons> {
    options.sort_by_key(|dirs| dirs.len());
    options.get(0).expect("No options were available, which is impossible").clone()
}

pub fn part1() {
    let codes = parse();
    let mut sum = 0;

    let mut memo: TryPathMemo = HashMap::new();

    for code in codes {
        let firstdirs = dirs_from_nums(&code, Vec::new(), NumpadButtons::A,  &mut memo);
        let seconddirs = dirs_from_dirs(&firstdirs, Vec::new(), DirectionButtons::A, &mut memo);
        let mut thirddirs = dirs_from_dirs(&seconddirs, Vec::new(), DirectionButtons::A, &mut memo);
        let min = min_dirs(&mut thirddirs);
        printdirs(&min);
        println!("{}", retrace_nums_dirs(&retrace_dirs_dirs(&retrace_dirs_dirs(&min))).iter().fold(String::new(), |prev, b| prev + &b.to_char().to_string()));

        let shortestlength = min.len();
        let factor: usize = code[0..code.len() - 1].iter().fold(String::new(), |prev, b| prev + &b.to_char().to_string()).parse().expect("Unexpected press command not at the end of the pattern");
        println!("{code:?}: {shortestlength}, {factor}");
        sum += shortestlength * factor;
    }

    println!("{sum}");
}

type DepthMemo = HashMap<(DirectionButtons, DirectionButtons, usize), usize>;
fn dir_to_dir_depth(from: DirectionButtons, to: DirectionButtons, atomic: &HashMap<(DirectionButtons, DirectionButtons), Vec<Vec<DirectionButtons>>>, depth: usize, memo: &mut DepthMemo) -> usize {
    if let Some(memoized) = memo.get(&(from, to, depth)) {
        return *memoized;
    }
    let mut result = Vec::new();
    let min;
    match depth {
        1 => {
            if let Some(path) = try_path_raw(from.to_position(), to.to_position(), DirectionButtons::IMPOSSIBLE, false) {
                result.push(path);
            }
            if let Some(path) = try_path_raw(from.to_position(), to.to_position(), DirectionButtons::IMPOSSIBLE, true) {
                result.push(path);
            }
            min = result.iter().map(|pat| pat.len()).min().expect("Pattern must be constructible");
        },
        _ => {
            let mut optimal = usize::MAX;
            for option in atomic.get(&(from, to)).unwrap() {
                let mut length = 0;
                for (from, to) in vec![DirectionButtons::A].into_iter().chain(option.iter().copied()).zip(option.iter().copied()) {
                    length += dir_to_dir_depth(from, to, atomic, depth - 1, memo);
                }
                if length < optimal { optimal = length; }
            }
            min = optimal;
        }
    }
    memo.insert((from, to, depth), min);
    min
}

pub fn part2() {
    let codes = parse();
    let mut sum = 0;

    let mut secondarymemo: TryPathMemo = HashMap::new();
    let mut memo: DepthMemo = HashMap::new();

    let mut atomic = HashMap::new();
    let allbuttons = vec![DirectionButtons::A, DirectionButtons::Top, DirectionButtons::Right, DirectionButtons::Bottom, DirectionButtons::Left];
    for a in allbuttons.iter() {
        for b in allbuttons.iter() {
            atomic.insert((*a, *b), dirs_from_dirs(&vec![vec![*b]], Vec::new(), *a, &mut HashMap::new()));
        }
    }

    for code in codes {
        let mut firstdirs = dirs_from_nums(&code, Vec::new(), NumpadButtons::A,  &mut secondarymemo);
        let min: Vec<usize> = firstdirs.iter().map(|pat| pat.len()).collect();
        let minmin = *min.iter().min().expect("Pattern must be constructible");
        firstdirs = firstdirs.into_iter().enumerate().filter(|(i, _)| min[*i] == minmin).map(|(_, v)| v).collect();
        
        let mut optimal = usize::MAX;
        for dir in firstdirs {
            let mut length = 0;
            for (from, to) in vec![DirectionButtons::A].into_iter().chain(dir.iter().copied()).zip(dir.iter().copied()) {
                length += dir_to_dir_depth(from, to, &atomic, 25, &mut memo);
            }
            if length < optimal { optimal = length; }
        }

        let factor: usize = code[0..code.len() - 1].iter().fold(String::new(), |prev, b| prev + &b.to_char().to_string()).parse().expect("Unexpected press command not at the end of the pattern");
        println!("{code:?}: {optimal}, {factor}");
        sum += optimal * factor;
    }

    println!("{sum}");
}