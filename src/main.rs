use std::collections::HashMap;
use std::env;
use std::fs;
use std::i32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].clone().parse::<isize>().unwrap();

    let filename = format!("./src/inputs/{:?}.txt", day);

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file").trim().to_string();
    println!("Day {}:", day);
    match day {
        1 => day_one(input),
        2 => day_two(input),
        3 => day_three(input),
        4 => day_four(input),
        5 => day_five(input),
        6 => day_six(input),
        _ => println!("Specify a day"),
    }
}

fn day_six(input: String) -> () {
    let orbits = input.split("\n");
    let mut orbit_map: HashMap<&str, &str> = HashMap::new();
    for orbit in orbits {
        let parts: Vec<&str> = orbit.split(")").collect();
        if parts.len() == 2 {
            orbit_map.insert(parts[1], parts[0]);
        }
    }
    let mut checksum = 0;
    for orbit in orbit_map.iter() {
        let mut step = orbit.1;
        loop {
            checksum += 1;
            match step {
                &"COM" => break,
                _ => (),
            }
            step = &orbit_map[step];
        }
    }
    println!("Checksum:        {}", checksum);
    let mut you_path: Vec<&str> = Vec::new();
    let mut san_path: Vec<&str> = Vec::new();
    let mut step = orbit_map["YOU"];
    loop {
        match step {
            "COM" => break,
            _ => (),
        }
        you_path.push(orbit_map[step]);
        step = &orbit_map[step];
    }
    step = orbit_map["SAN"];
    let target: &str;
    let mut transfers = 0;
    loop {
        if you_path.contains(&step) {
            target = step;
            break;
        }
        transfers += 1;
        san_path.push(orbit_map[step]);
        step = &orbit_map[step];
    }
    for you in you_path {
        transfers += 1;
        if you == target {
            break;
        }
    }
    println!("Total Transfers: {}", transfers);
}

fn day_five(input: String) -> () {
    let mut memory_one: Vec<i32> = input.split(",").map(parse_int).collect();
    let result_one = run_computer(&mut memory_one, 1);
    println!("Diagnostic Code 1: {}", result_one);
    let mut memory_five: Vec<i32> = input.split(",").map(parse_int).collect();
    let result_five = run_computer(&mut memory_five, 5);
    println!("Diagnostic Code 5: {}", result_five);
}

fn day_four(input: String) -> () {
    let parsed: Vec<i32> = input.split('-').map(parse_int).collect();
    let start = parsed[0];
    let end = parsed[1];
    let mut test = start;
    let mut easy_counter = 0;
    let mut hard_counter = 0;
    while test < end {
        if passes_easy_test(test) {
            easy_counter += 1;
            if passes_hard_test(test) {
                hard_counter += 1;
            }
        }
        test += 1;
    }
    println!("Easy: {}", easy_counter);
    println!("Hard: {}", hard_counter);
}

fn passes_easy_test(n: i32) -> bool {
    let string = format!("{}", n);
    let mut current_char = string.chars().nth(0).unwrap();
    let mut found_double = false;
    let size = 6;
    let mut i = 1;
    while i < size {
        let character = string.chars().nth(i).unwrap();
        if character < current_char {
            return false;
        }
        if character == current_char {
            found_double = true;
        }
        current_char = character;
        i += 1;
    }
    return found_double;
}

fn passes_hard_test(n: i32) -> bool {
    let string = format!("{}", n);
    let mut current_char = string.chars().nth(1).unwrap();
    let mut previous_char = string.chars().nth(0).unwrap();
    if previous_char > current_char {
        return false;
    }
    let mut found_double = false;
    let size = 6;
    let mut i = 2;
    while i < size {
        let character = string.chars().nth(i).unwrap();
        if character < current_char {
            return false;
        }
        if i == 2 && previous_char == current_char && character != current_char {
            found_double = true;
        }
        if character == current_char && current_char != previous_char {
            if i < (size - 1) {
                found_double |= character != string.chars().nth(i + 1).unwrap()
            } else {
                found_double = true;
            }
        }
        previous_char = current_char;
        current_char = character;
        i += 1;
    }
    return found_double;
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

fn parse_int(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}

fn day_two(input: String) -> () {
    let mut memory: Vec<i32> = input.clone().split(",").map(parse_int).collect();

    memory[1] = 12;
    memory[2] = 2;

    run_computer(&mut memory, 0);

    println!("1202 Result:        {}", memory[0]);
    let mut done = false;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut test_memory: Vec<i32> = input.clone().split(",").map(parse_int).collect();
            test_memory[1] = noun;
            test_memory[2] = verb;
            run_computer(&mut test_memory, 0);
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

fn run_computer(memory: &mut Vec<i32>, input: i32) -> i32 {
    let mut pc = 0;
    let mut output = 0;
    loop {
        let opcode = memory[pc as usize] % 100;
        if opcode == 99 {
            break;
        }
        let arg1: i32;
        if (memory[pc as usize] / 100) % 10 == 1 {
            arg1 = memory[(pc + 1) as usize];
        } else {
            arg1 = memory[memory[(pc + 1) as usize] as usize];
        }
        let arg2: i32;
        if opcode == 3 || opcode == 4 {
            arg2 = 0;
        } else if (memory[pc as usize] / 1000) % 10 == 1 {
            arg2 = memory[(pc + 2) as usize];
        } else {
            arg2 = memory[memory[(pc + 2) as usize] as usize];
        }
        if opcode == 1 {
            // add
            let target = memory[(pc + 3) as usize] as usize;
            memory[target] = arg2 + arg1;
            pc += 4;
        } else if opcode == 2 {
            //multiply
            let target = memory[(pc + 3) as usize] as usize;
            memory[target] = arg2 * arg1;
            pc += 4;
        } else if opcode == 3 {
            //input
            let target = memory[(pc + 1) as usize] as usize;
            memory[target] = input;
            pc += 2;
        } else if opcode == 4 {
            //output
            output = arg1;
            pc += 2;
        } else if opcode == 5 {
            // jump if true
            if arg1 != 0 {
                pc = arg2;
            } else {
                pc += 3;
            }
        } else if opcode == 6 {
            // jump if false
            if arg1 == 0 {
                pc = arg2;
            } else {
                pc += 3;
            }
        } else if opcode == 7 {
            // less than
            let target = memory[(pc + 3) as usize] as usize;
            if arg1 < arg2 {
                memory[target] = 1;
            } else {
                memory[target] = 0;
            }
            pc += 4;
        } else if opcode == 8 {
            //equals
            let target = memory[(pc + 3) as usize] as usize;
            if arg1 == arg2 {
                memory[target] = 1;
            } else {
                memory[target] = 0;
            }
            pc += 4;
        } else {
            println!("Invalid memory!");
            break;
        }
    }
    return output;
}

fn day_one(input: String) -> () {
    let first_sum = input.split_ascii_whitespace().map(get_fuel_basic).sum::<i32>();
    let second_sum = input.split_ascii_whitespace().map(get_fuel).sum::<i32>();
    println!("Basic calculation:    {:?}", first_sum);
    println!("Advanced calculation: {:?}", second_sum);
}

fn get_fuel_basic(input: &str) -> i32 {
    let mass = input.parse::<i32>().unwrap();
    return (mass / 3) - 2;
}

fn get_fuel(input: &str) -> i32 {
    let mass = input.parse::<i32>().unwrap();
    let mut to_add = (mass / 3) - 2;
    let mut total_to_add = to_add;
    let mut done = false;
    while !done {
        to_add = (to_add / 3) - 2;
        if to_add <= 0 {
            done = true
        } else {
            total_to_add += to_add
        }
    }

    return total_to_add;
}
