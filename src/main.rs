use std::collections::HashMap;
use std::env;
use std::fs;
use std::i32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].clone().parse::<isize>().unwrap();

    let filename = format!("./src/inputs/{:?}.txt", day);

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match day {
        1 => day_one(input),
        2 => day_two(input),
        3 => day_three(input),
        _ => println!("Specify a day")
    }
}

fn day_three(input: String) -> () {
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
            let size = parse_int(&m[1..]);
            let mut dx = 0;
            let mut dy = 0;
            match dir {
                'R' => dx = 1,
                'L' => dx = -1,
                'U' => dy = -1,
                'D' => dy = 1,
                _ => println!("Invalid dir")
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
                    move_counter.entry(format!("{}{}", x, y)).or_insert(total_moves);
                }
                if !first_wire && *value {
                    if distance < nearest_distance {
                        //there's a nearer crossing
                        nearest_distance = distance;
                    }
                    crossings.push(total_moves + *move_counter.entry(format!("{}{}", x, y)).or_insert(i32::MAX));
                }
            }
        }
        first_wire = false;
    }
    crossings.sort();
    // Don't know why the first is invalid, and why the second is correct
    println!("Nearest intersection: {}", nearest_distance);
    println!("First intersection: {}", crossings[1]);
}

fn parse_int(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}

fn day_two(input: String) -> () {
    let mut done = false;
    for noun in 0..100 {
        for verb in 0..100 {
            if run_computer(input.clone(), noun, verb) == 19690720 {
                println!("{:?}", verb + noun * 100);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
}

fn run_computer(input: String, noun: i32, verb: i32) -> i32 {
    let mut memory: Vec<i32> = input.split(",").map(parse_int).collect();
    memory[1] = noun;
    memory[2] = verb;
    let mut done = false;
    let mut pc = 0;
    while !done {
        if memory[pc] == 99 {
            done = true
        } else if memory[pc] == 1 {
            let target = memory[pc + 3] as usize;
            memory[target] = memory[memory[pc + 2] as usize] + memory[memory[pc + 1] as usize];
        } else if memory[pc] == 2 {
            let target = memory[pc + 3] as usize;
            memory[target] = memory[memory[pc + 2] as usize] * memory[memory[pc + 1] as usize];
        } else {
            println!("Invalid memory!");
            done = true;
        }
        pc += 4;
    }
    return memory[0]
}

fn day_one(input: String) -> () {
    let inputs = input.split_ascii_whitespace();
    println!("{:?}", inputs.map(get_fuel).sum::<i32>());
}

fn get_fuel(input: &str) -> i32 {
    let mass = input.parse::<i32>().unwrap();
    let mut to_add = (mass / 3) - 2;
    let mut total_to_add = to_add;
    let mut done = false;
    while !done{
        to_add = (to_add / 3) - 2;
        if to_add <= 0 {
            done = true
        } else {
            total_to_add += to_add
        }
    }
    
    return total_to_add;
}