use crate::aoc::computer;
use crate::aoc::utils;

pub fn day_two(input: String) -> () {
    let mut memory: Vec<i32> = input.clone().split(",").map(utils::parse_int).collect();

    memory[1] = 12;
    memory[2] = 2;

    computer::run_computer(&mut memory, 0);

    println!("1202 Result:        {}", memory[0]);
    let mut done = false;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut test_memory: Vec<i32> = input.clone().split(",").map(utils::parse_int).collect();
            test_memory[1] = noun;
            test_memory[2] = verb;
            computer::run_computer(&mut test_memory, 0);
            if test_memory[0] == 19690720 {
                println!("Correct Error Code: {:?}", verb + noun * 100);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
}
