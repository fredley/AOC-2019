use crate::aoc::computer;
use crate::aoc::utils;

pub fn day_five(input: String) -> () {
    let mut memory_one: Vec<i32> = input.split(",").map(utils::parse_int).collect();
    let result_one = computer::run_computer(&mut memory_one, 1);
    println!("Diagnostic Code 1: {}", result_one);
    let mut memory_five: Vec<i32> = input.split(",").map(utils::parse_int).collect();
    let result_five = computer::run_computer(&mut memory_five, 5);
    println!("Diagnostic Code 5: {}", result_five);
}
