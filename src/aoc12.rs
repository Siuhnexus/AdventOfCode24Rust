use std::{collections::{HashMap, HashSet}, fs};

fn parse() -> HashMap<(u16, u16), char> {
    let mut result = HashMap::new();
    for (y, line) in fs::read_to_string("input/12.txt").expect("Input file not found").lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            result.insert((x as u16, y as u16), v);
        }
    }
    result
}

#[derive(PartialEq)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left
}
impl Direction {
    pub fn from_derivative(dir: (i32, i32)) -> Option<Direction> {
        match dir {
            (0, -1) => Some(Direction::Top),
            (1, 0) => Some(Direction::Right),
            (0, 1) => Some(Direction::Bottom),
            (-1, 0) => Some(Direction::Left),
            _ => None
        }
    }
    pub fn to_derivative(&self) -> (i32, i32) {
        match self {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0)
        }
    }

    pub fn from_positions(from: (u16, u16), to: (u16, u16)) -> Option<Direction> {
        Direction::from_derivative((to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32))
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Top => Direction::Left,
            Direction::Right => Direction::Top,
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Bottom
        }
    }
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top
        }
    }
    pub fn flip(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right
        }
    }

    pub fn step(&self, position: (u16, u16)) -> Option<(u16, u16)> {
        if (position.0 == 0 && *self == Direction::Left) || (position.1 == 0 && *self == Direction::Top) { return None; }
        let dir = self.to_derivative();
        Some(((position.0 as i32 + dir.0) as u16, (position.1 as i32 + dir.1) as u16))
    }

    pub fn sort(&self, positions: &mut Vec<(u16, u16)>) {
        let factors = self.to_derivative();
        positions.sort_by_key(|a| a.0 as i32 * factors.0 + a.1 as i32 * factors.1);
    }
}

fn sharededge(a: (u16, u16), b: (i32, i32)) -> Vec<(u16, u16)> {
    let edgesa = vec![(a.0 as i32, a.1 as i32), (a.0 as i32 + 1, a.1 as i32), (a.0 as i32, a.1 as i32 + 1), (a.0 as i32 + 1, a.1 as i32 + 1)];
    let edgesb = vec![b, (b.0 + 1, b.1), (b.0, b.1 + 1), (b.0 + 1, b.1 + 1)];
    edgesa.into_iter().collect::<HashSet<(i32, i32)>>().intersection(&edgesb.into_iter().collect::<HashSet<(i32, i32)>>()).map(|n| (n.0 as u16, n.1 as u16)).collect::<Vec<(u16, u16)>>()
}
fn isvertical(edge: &((u16, u16), (u16, u16))) -> bool {
    let diffx: i32 = edge.1.0 as i32 - edge.0.0 as i32;
    return diffx == 0
}
fn addedges(direction: &Direction, first: (u16, u16), second: (i32, i32), perimeter: &mut Vec<((u16, u16), (u16, u16))>) {
    let mut overlap = sharededge(first, second);
    if overlap.len() != 2 { panic!("Neighbouring cells share an edge") }
    direction.turn_right().sort(&mut overlap);
    perimeter.push((overlap[0], overlap[1]));
}
fn explore(map: &mut HashMap<(u16, u16), char>, position: (u16, u16), id: &char, area: &mut u16, perimeter: &mut Vec<((u16, u16), (u16, u16))>, coords: &mut Vec<(u16, u16)>) {
    let mut direction: Direction = Direction::Top;
    loop {
        match direction.step(position) {
            None => {
                let dir = direction.to_derivative();
                addedges(&direction, position, (position.0 as i32 + dir.0 as i32, position.1 as i32 + dir.1 as i32), perimeter);
            },
            Some(nextpos) => {
                let toremove = match map.get(&nextpos) {
                    Some(k) => {
                        if k != id {
                            addedges(&direction, position, (nextpos.0 as i32, nextpos.1 as i32), perimeter);
                            None
                        }
                        else {
                            *area += 1;
                            Some(nextpos)
                        }
                    },
                    None => {
                        if !coords.contains(&nextpos) {
                            addedges(&direction, position, (nextpos.0 as i32, nextpos.1 as i32), perimeter);
                        }
                        None
                    }
                };
                if let Some(nextpos) = toremove {
                    map.remove(&nextpos);
                    coords.push(nextpos.clone());
                    explore(map, nextpos, id, area, perimeter, coords);
                }
            }
        }
        direction = direction.turn_right();
        if direction == Direction::Top { break; }
    }
}
pub fn part1() {
    let mut map = parse();
    let mut costs: u32 = 0;
    while map.len() > 0 {
        let pivot = map.iter().next().expect("Map cannot be empty here");
        let id = pivot.1.clone();
        let mut area: u16 = 1;
        let mut perimeter = Vec::new();
        let position = pivot.0.clone();
        let mut coords = vec![position];
        map.remove(&position);
        explore(&mut map, position, &id, &mut area, &mut perimeter, &mut coords);
        costs += (area as u32) * (perimeter.len() as u32);
    }
    println!("{}", costs);
}

fn printedges(walked: Vec<((u16, u16), (u16, u16))>) {
    println!("{}", (0..7).fold(String::new(), |prev, y| { prev + &(0..7).fold(String::new(), |prev, x| {
        let hashorizontal = |(f, s): (&(u16, u16), &(u16, u16))| -> bool { (*f == (x, y) && *s == (x + 1, y)) || (*f == (x + 1, y) && *s == (x, y)) || x > 0 && ((*f == (x, y) && *s == (x - 1, y)) || (*f == (x - 1, y) && *s == (x, y))) };
        let hasvertical = |(f, s): (&(u16, u16), &(u16, u16))| (*f == (x, y) && *s == (x, y + 1)) || (*f == (x, y + 1) && *s == (x, y)) || y > 0 && ((*f == (x, y) && *s == (x, y - 1)) || (*f == (x, y - 1) && *s == (x, y)));
        prev + match (walked.iter().find(|(f, s)| hashorizontal((f, s))), walked.iter().find(|(f, s)| hasvertical((f, s)))) {
            (None, None) => ".",
            (None, Some(_)) => "|",
            (Some(_), None) => "-",
            (Some(_), Some(_)) => "+"
        }
    }) + "\n" }));
}
fn iscontinuous(walked: &mut Vec<((u16, u16), (u16, u16))>) -> bool {
    let first = walked[0];
    let mut current = first;
    while walked.len() > 0 {
        current = match walked.iter().find(|(f, s)| { let v = vec![*f, *s]; v.contains(&current.0) || v.contains(&current.1) }) {
            None => return false,
            Some(v) => { *v }
        };
        walked.remove(walked.iter().position(|(f, s)| f == &current.0 && s == &current.1).unwrap());
    }
    return { let v = vec![first.0, first.1]; v.contains(&current.0) || v.contains(&current.1) };
}
pub fn part2() {
    let mut map = parse();
    let mut costs: u32 = 0;
    while map.len() > 0 {
        let pivot = map.iter().next().expect("Map cannot be empty here");
        let id = pivot.1.clone();
        let mut area: u16 = 1;
        let mut perimeter = Vec::new();
        let position = pivot.0.clone();
        let mut coords = vec![position];
        map.remove(&position);
        explore(&mut map, position, &id, &mut area, &mut perimeter, &mut coords);
        
        let mut edgetotal: u32 = 0;

        while perimeter.len() > 0 {
            let mut prevedge = perimeter.pop().expect("Every region must have edges");
            let mut prevorientation = isvertical(&prevedge);

            let firstorientation = prevorientation;

            let mut walked = vec![prevedge.clone()];
            let mut edgecount: u32 = 1;

            while perimeter.len() > 0 {
                let ways: Vec<((u16, u16), (u16, u16))> = perimeter.iter().filter(|(f, _)| {
                    return *f == prevedge.1
                }).copied().collect();
                let chosen = match ways.len() {
                    0 => break, // Region has > 1 disconnected perimeters
                    1 => {
                        ways[0]
                    },
                    _ => { // Always turn inwards at junctions
                        let second = Direction::from_positions(prevedge.0, prevedge.1).unwrap().turn_right().step(prevedge.1).unwrap();
                        (prevedge.1, second)
                    }
                };
                let orientation = isvertical(&chosen);
                if orientation != prevorientation { edgecount += 1; }
                perimeter.remove(perimeter.iter().position(|(f, s)| f == &chosen.0 && s == &chosen.1).unwrap());
                prevedge = chosen;
                walked.push(chosen);
                prevorientation = orientation;
            }
            if prevorientation == firstorientation { edgecount -= 1; }

            if !iscontinuous(&mut walked.clone()) { printedges(walked); println!("{id}"); panic!("WARNING: Non-closing border detected"); }
            //printedges(walked);
            //println!("{}", edgecount);

            edgetotal += edgecount;
        }
        costs += (area as u32) * edgetotal;
    }
    println!("{}", costs);
}