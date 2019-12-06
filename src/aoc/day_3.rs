use std::collections::HashMap;
use std::i32;

use crate::aoc::utils;

pub fn day_three(input: String) -> () {
    let wires = input.split("\n");
    let mut world: HashMap<String, bool> = HashMap::new();
    let mut move_counter: HashMap<String, i32> = HashMap::new();
    let mut first_wire = true;
    let mut nearest_distance = 999999;
    let mut crossings: Vec<i32> = Vec::new();
    for wire in wires {
        let moves = wire.split(",");
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut total_moves: i32 = 0;
        for m in moves {
            let dir = m.chars().nth(0).unwrap();
            let size = utils::parse_int(&m[1..]);
            let mut dx = 0;
            let mut dy = 0;
            match dir {
                'R' => dx = 1,
                'L' => dx = -1,
                'U' => dy = -1,
                'D' => dy = 1,
                _ => println!("Invalid dir"),
            }
            let mut i = 0;
            while i < size {
                i += 1;
                total_moves += 1;
                x += dx;
                y += dy;
                let distance = x.abs() + y.abs();
                let value = world.entry(format!("{}{}", x, y)).or_insert(first_wire);
                if first_wire {
                    move_counter
                        .entry(format!("{}{}", x, y))
                        .or_insert(total_moves);
                }
                if !first_wire && *value {
                    if distance < nearest_distance {
                        //there's a nearer crossing
                        nearest_distance = distance;
                    }
                    crossings.push(
                        total_moves
                            + *move_counter
                                .entry(format!("{}{}", x, y))
                                .or_insert(i32::MAX),
                    );
                }
            }
        }
        first_wire = false;
    }
    crossings.sort();
    // Don't know why the first is invalid, and why the second is correct
    println!("Nearest intersection: {}", nearest_distance);
    println!("First intersection:   {}", crossings[1]);
}
