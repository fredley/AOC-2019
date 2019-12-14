use crate::aoc::computer;
use crate::aoc::utils;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

pub fn day_thirteen(input: String) -> () {
    let mut memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    memory[0] = 2;
    let mut computer = computer::Computer::new(vec![]);
    computer.set_memory(memory);
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut first_pass = true;
    while !computer.is_halted {
        // display the whole screen and score
        let mut block_count = 0;
        loop {
            let x = computer.run();
            if computer.requires_input || computer.is_halted {
                break
            }
            let y = computer.run();
            if computer.requires_input || computer.is_halted {
                break
            }
            if x == -1 && y == 0 {
                score = computer.run();
                continue;
            }
            let tile_id = computer.run();
            if computer.requires_input || computer.is_halted {
                break
            }
            match tile_id {
                2 => block_count += 1,
                3 => paddle_x = x,
                4 => ball_x = x,
                _ => (),
            };
        }
        if first_pass {
            println!("Block count: {}", block_count);
            first_pass = false;
        }
        let mut joystick_input = 0;
        if ball_x < paddle_x { 
            joystick_input = -1;
        } else if paddle_x < ball_x { 
            joystick_input = 1;
        }
        computer.set_input(vec![joystick_input]);
    }
    println!("Final score: {}", score);
}