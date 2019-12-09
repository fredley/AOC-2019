use crate::aoc::computer;
use crate::aoc::utils;

pub fn day_nine(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut computer = computer::Computer::new(vec![1]);
    computer.set_memory(memory.clone());
    let mut output: i64;
    let mut boost: i64 = 0;
    loop {
        output = computer.run();
        if computer.is_halted {
            break
        }
        boost = output;
    }
    println!("BOOST keycode: {}", boost);
    let mut computer_2 = computer::Computer::new(vec![2]);
    computer_2.set_memory(memory.clone());
    println!("Coordinates  : {}", computer_2.run());
}
