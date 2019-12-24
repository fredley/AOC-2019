use crate::aoc::computer;
use crate::aoc::utils;

pub fn day_twenty_one(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut computer = computer::Computer::new(vec![]);
    computer.set_memory(memory.clone());
    computer.set_input_ascii("NOT B T\nNOT C J\nAND T J\nAND D J\nNOT C T\nAND D T\nOR T J\nNOT A T\nAND D T\nOR T J\nWALK\n".to_string());
    let mut damage = 0;
    while !computer.is_halted {
        let output = computer.run();
        if output > 0 {
            damage = output;
        }
    }
    println!("Walk: {}", damage);
    computer = computer::Computer::new(vec![]);
    computer.set_memory(memory.clone());
    computer.set_input_ascii("NOT A J\nAND D J\nNOT B T\nAND D T\nAND H T\nOR T J\nNOT C T\nAND D T\nAND E T\nOR T J\nNOT C T\nAND D T\nAND H T\nOR T J\nRUN\n".to_string());
    let mut damage = 0;
    while !computer.is_halted {
        let output = computer.run();
        if output > 0 {
            damage = output;
        }
    }
    println!("Run:  {}", damage);
}
