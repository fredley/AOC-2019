use crate::aoc::computer;
use crate::aoc::computer::Computes;
use crate::aoc::utils;

pub fn day_nine(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut computer = computer::Computer::new(memory, vec![1]);
    let boost = computer.run();
    println!("BOOST keycode: {}", boost);

}