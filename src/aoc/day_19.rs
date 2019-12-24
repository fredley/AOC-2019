use crate::aoc::computer;
use crate::aoc::utils;

pub fn day_nineteen(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut x:i64;
    let mut y = 0;
    let mut pulled_count = 0;
    while y < 50 {
        x = 0;
        while x < 50 {
            let mut computer = computer::Computer::new(vec![]);
            computer.set_memory(memory.clone());
            computer.set_input(vec![x,y]);
            let output = computer.run();
            pulled_count += output;
            x += 1
        }
        y += 1;
    }
    println!("Pulled Count: {}", pulled_count);
    y = 100;
    x = 100;
    let mut done = false;
    while !done{
        let mut drop = false;
        x -= 5;
        while !done{
            let output = send_drone(&memory, x, y);
            if output == 1 {
                drop = true;
            } else if output == 0 && drop {
                if send_drone(&memory, x - 100, y + 99) == 1 {
                    // it fits!
                    println!("Ship XY      : {}", (x-100)*10000 + y);
                    done = true;
                } else {
                    break;
                }
            }
            x += 1
        }
        y += 1;
    }
}

fn send_drone(memory: &Vec<i64>, x: i64, y: i64) -> i64 {
    let mut computer = computer::Computer::new(vec![]);
    computer.set_memory(memory.clone());
    computer.set_input(vec![x,y]);
    return computer.run();
}
