use crate::aoc::computer;
use crate::aoc::utils;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn param(&self) -> usize {
        return self.x * self.y;
    }
}

pub fn day_seventeen(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut computer = computer::Computer::new(vec![]);
    let mut map:HashMap<Coord, char> = HashMap::new();
    let mut intersections: Vec<Coord> = Vec::new();
    computer.set_memory(memory.clone());
    let mut x = 0;
    let mut y = 0;
    while !computer.is_halted {
        let output = computer.run() as u8;
        map.insert(Coord{x: x, y:y}, output as char);
        if y > 1 && x > 0 {
            if  map.get(&Coord{x: x-1, y: y-1}).unwrap() == &'#' &&
                map.get(&Coord{x: x, y: y-1}).unwrap() == &'#' &&
                map.get(&Coord{x: x + 1, y: y-1}).unwrap() == &'#' &&
                map.get(&Coord{x: x, y: y-2}).unwrap() == &'#' {
                    // found an intersection at (x, y-1)
                    intersections.push(Coord{x: x, y: y-1})
                }
        }
        if output == 10 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    let sum: usize = intersections.iter().map(|c| c.param()).sum();
    println!("Sum of parameters: {}", sum);
    let mut memory2 = memory.clone();
    memory2[0] = 2;
    let mut c = computer::Computer::new(vec![]);
    c.set_memory(memory2);
    let inputs = vec![
        65,44,67,44,65,44,67,44,66,44,66,44,67,44,65,44,67,44,66,10,
        76,44,49,50,44,76,44,49,48,44,82,44,56,44,76,44,49,50,10,
        76,44,49,48,44,82,44,49,50,44,82,44,56,10,
        82,44,56,44,82,44,49,48,44,82,44,49,50,10,
        110,10
    ];
    c.set_input(inputs);
    let mut output: i64 = 0;
    while !c.is_halted {
        let val = c.run();
        if val > 0 {
            output = val;
        }
    }
    println!("Dust:              {}", output);
}
