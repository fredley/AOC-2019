use std::collections::HashMap;

use crate::aoc::computer;
use crate::aoc::utils;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

struct PaintingResult {
    top_left: Coord,
    bottom_right: Coord,
    hull: HashMap<Coord, i64>,
}

pub fn day_eleven(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let black_results = paint_hull(memory.clone(), false);
    println!("Number of panels painted: {}", black_results.hull.len());
    let mut white_results = paint_hull(memory.clone(), true);
    println!("Registration Identifier:");
    let mut x: i64;
    let mut y = white_results.top_left.y;
    while y <= white_results.bottom_right.y{
        x = white_results.top_left.x;
        while x <= white_results.bottom_right.x {
            if *white_results.hull.entry(Coord{x: x, y: y}).or_insert(0) == 1 {
                print!("#")
            } else {
                print!(" ")
            }
            x += 1;
        }
        print!("\n");
        y += 1;
    }

}

fn paint_hull(memory: Vec<i64>, start_white: bool) -> PaintingResult {
    let mut computer = computer::Computer::new(vec![1]);
    computer.set_memory(memory);
    let mut hull: HashMap<Coord, i64> = HashMap::new();
    // black -> 0, white -> 1
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut dir: i64 = 0;
    let mut top_left = Coord{x: 0, y: 0};
    let mut bottom_right = Coord{x: 0, y: 0};
    if start_white {
        // starting white square
        hull.entry(Coord{x: x, y: y}).or_insert(1);
    }
    // 0 -> N, 1 -> E, 2 -> S, 3 -> W
    while !computer.is_halted {
        let color = hull.entry(Coord{x: x, y: y}).or_insert(0);
        computer.set_input(vec![*color]);
        let new_color = computer.run();
        hull.insert(Coord{x: x, y: y}, new_color);
        let movement = computer.run();
        match movement {
            0 => dir = (dir + 3) % 4,
            1 => dir = (dir + 5) % 4,
            _ => (),
        };
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            _ => println!("Unknown dir {}", dir),
        };
        if x < top_left.x {
            top_left.x = x;
        }
        if y < top_left.y {
            top_left.y = y;
        }
        if x > bottom_right.x {
            bottom_right.x = x;
        }
        if y > bottom_right.y {
            bottom_right.y = y;
        }
    }
    return PaintingResult{
        top_left: top_left,
        bottom_right: bottom_right,
        hull: hull,
    }
}