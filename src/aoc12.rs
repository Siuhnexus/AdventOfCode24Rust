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

fn sharededge(a: (u16, u16), b: (i32, i32)) -> Option<((u16, u16), (u16, u16))> {
    let edgesa = vec![(a.0 as i32, a.1 as i32), (a.0 as i32 + 1, a.1 as i32), (a.0 as i32, a.1 as i32 + 1), (a.0 as i32 + 1, a.1 as i32 + 1)];
    let edgesb = vec![b, (b.0 + 1, b.1), (b.0, b.1 + 1), (b.0 + 1, b.1 + 1)];
    let overlap: Vec<(i32, i32)> = edgesa.into_iter().collect::<HashSet<(i32, i32)>>().intersection(&edgesb.into_iter().collect::<HashSet<(i32, i32)>>()).copied().collect::<Vec<(i32, i32)>>();
    if overlap.len() != 2 { None } else {
        Some(((overlap[0].0 as u16, overlap[0].1 as u16), (overlap[1].0 as u16, overlap[1].1 as u16)))
    }
}
fn isvertical(edge: &((u16, u16), (u16, u16))) -> bool {
    let diffx: i32 = edge.1.0 as i32 - edge.0.0 as i32;
    return diffx == 0
}
fn explore(map: &mut HashMap<(u16, u16), char>, position: (u16, u16), id: &char, area: &mut u16, perimeter: &mut Vec<((u16, u16), (u16, u16))>, coords: &mut Vec<(u16, u16)>) {
    let mut direction: (i8, i8) = (0, -1);
    loop {
        if position.0 == 0 && direction.0 < 0 || position.1 == 0 && direction.1 < 0 {
            perimeter.push(sharededge(position, (position.0 as i32 + direction.0 as i32, position.1 as i32 + direction.1 as i32)).expect("Neighbouring cells share an edge"));
        }
        else {
            let nextpos = ((position.0 as i32 + direction.0 as i32) as u16, (position.1 as i32 + direction.1 as i32) as u16);
            let toremove = match map.get(&nextpos) {
                Some(k) => {
                    if k != id {
                        perimeter.push(sharededge(position, (nextpos.0 as i32, nextpos.1 as i32)).expect("Neighbouring cells share an edge")); None
                    }
                    else {
                        *area += 1;
                        Some(nextpos)
                    }
                },
                None => {
                    if !coords.contains(&nextpos) {
                        perimeter.push(sharededge(position, (nextpos.0 as i32, nextpos.1 as i32)).expect("Neighbouring cells share an edge"));
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
        direction = (-direction.1, direction.0);
        if direction.1 == -1 { break; }
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
    println!("{}", (0..141).fold(String::new(), |prev, y| { prev + &(0..141).fold(String::new(), |prev, x| {
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

            let mut firstorientation = prevorientation;
            let mut isfirst = true;
            let mut alternative = None;

            let mut walked = vec![prevedge.clone()];
            let mut edgecount: u32 = 1;

            while perimeter.len() > 0 {
                let ways: Vec<((u16, u16), (u16, u16))> = perimeter.iter().filter(|(f, s)| { let v = vec![*f, *s]; v.contains(&prevedge.0) || v.contains(&prevedge.1) }).copied().collect();
                let chosen = match ways.len() {
                    0 => break, // Region has > 1 disconnected perimeters
                    1 => {
                        ways[0]
                    },
                    l => {
                        if isfirst && l == 2 {
                            alternative = Some(ways[1]);
                            ways[0]
                        }
                        else {
                            let val = match alternative {
                                None => break, // Region has other disconnected perimeters affecting the same junction
                                Some(a) => {
                                    let temp = firstorientation;
                                    firstorientation = prevorientation;
                                    prevorientation = temp;
                                    a
                                }
                            };
                            alternative = None;
                            val
                        }
                    }
                };
                let orientation = isvertical(&chosen);
                if orientation != prevorientation { edgecount += 1; }
                perimeter.remove(perimeter.iter().position(|(f, s)| f == &chosen.0 && s == &chosen.1).unwrap());
                prevedge = chosen;
                walked.push(chosen);
                prevorientation = orientation;

                isfirst = false;
            }
            if prevorientation == firstorientation { edgecount -= 1; }

            if !iscontinuous(&mut walked) { println!("WARNING: Non-closing border detected"); }
            //printedges(walked);
            //println!("{}", edgecount);

            edgetotal += edgecount;
        }
        costs += (area as u32) * edgetotal;
    }
    println!("{}", costs);
}