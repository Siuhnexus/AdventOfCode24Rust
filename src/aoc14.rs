use std::fs;

use image::{ImageBuffer, Rgb};

use crate::helpers::Wrapped;

#[derive(Debug)]
struct Robot {
    pos: (u16, u16),
    dir: (i32, i32)
}

fn parse() -> Vec<Robot> {
    let mut result = Vec::new();
    for line in fs::read_to_string("input/14.txt").expect("Input file not found").lines() {
        if line.trim() == "" { continue; }
        let parts: Vec<&str> = line.split(" v=").collect();
        if parts.len() != 2 { panic!("Wrong input format"); }
        let pos: Vec<u16> = parts[0].chars().skip(2).collect::<String>().split(",").map(|numval| numval.parse::<u16>().expect("Non-number found in input")).collect();
        if pos.len() != 2 { panic!("Wrong input format"); }
        let dir: Vec<i32> = parts[1].split(",").map(|numval| numval.parse::<i32>().expect("Non-number found in input")).collect();
        if dir.len() != 2 { panic!("Wrong input format"); }
        result.push(Robot { pos: (pos[0], pos[1]), dir: (dir[0], dir[1]) })
    }
    result
}

pub fn part1() {
    let robots = parse();
    let gridsize: (i32, i32) = (101, 103);

    let mut cells: Vec<Vec<u16>> = (0..gridsize.0).map(|_| (0..gridsize.1).map(|_| 0).collect()).collect();
    for robot in robots {
        let xresult = Wrapped::from(robot.pos.0 as i32 + robot.dir.0 * 100, gridsize.0);
        let yresult = Wrapped::from(robot.pos.1 as i32 + robot.dir.1 * 100, gridsize.1);
        cells[xresult.value as usize][yresult.value as usize] += 1;
    }

    let mut safety: u64 = 1;
    for quadrant in 0..4 {
        let offsetx: usize = if quadrant % 2 == 0 { 0 } else { (gridsize.0 / 2) as usize + 1 };
        let offsety: usize = if quadrant / 2 == 0 { 0 } else { (gridsize.1 / 2) as usize + 1 };

        let next= ((offsetx)..(offsetx + (gridsize.0 / 2) as usize)).fold(0, |prev, x| prev + (offsety..(offsety + (gridsize.1 / 2) as usize)).fold(0, |prev, y| prev + cells[x][y])) as u64;
        println!("{next}");
        safety *= next;
    }

    println!("{safety}");
}

fn printcells(cells: &Vec<Vec<u16>>, dimensions: &(i32, i32), fname: &str) {
    let mut img = ImageBuffer::new(dimensions.0 as u32, dimensions.1 as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let value: u8 = if cells[x as usize][y as usize] > 0 { 255 } else { 0 };
        *pixel = Rgb([value, value, value]);
    }
    match img.save(String::from("output/14/") + fname + ".png") {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    };
}
pub fn part2() {
    let mut robots = parse();
    let gridsize: (i32, i32) = (101, 103);

    let first: u32 = 71; // Worked out by trial and error
    let step: u32 = 101;
    for robot in robots.iter_mut() {
        robot.pos.0 = (robot.pos.0 as i64 + robot.dir.0 as i64 * first as i64).rem_euclid(gridsize.0 as i64) as u16;
        robot.pos.1 = (robot.pos.1 as i64 + robot.dir.1 as i64 * first as i64).rem_euclid(gridsize.1 as i64) as u16;
    }
    for n in 0..1000 {
        let current = first + n * step;
        let mut cells: Vec<Vec<u16>> = (0..gridsize.0).map(|_| (0..gridsize.1).map(|_| 0).collect()).collect();
        for robot in robots.iter_mut() {
            cells[robot.pos.0 as usize][robot.pos.1 as usize] += 1;
            robot.pos.0 = (robot.pos.0 as i64 + robot.dir.0 as i64 * step as i64).rem_euclid(gridsize.0 as i64) as u16;
            robot.pos.1 = (robot.pos.1 as i64 + robot.dir.1 as i64 * step as i64).rem_euclid(gridsize.1 as i64) as u16;
        }
        printcells(&cells, &gridsize, &current.to_string());
    }
}