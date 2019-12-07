use crate::aoc::computer;
use crate::aoc::computer::Computes;
use crate::aoc::utils;

pub fn day_seven(input: String) -> () {
    let memory: Vec<i32> = input.split(",").map(utils::parse_int).collect();
    run_without_feedback(&memory);
    run_with_feedback(&memory);
}

fn run_with_feedback(memory: &Vec<i32>) -> () {
    let mut max_output = 0;
    for a in vec![5,6,7,8,9] {
        for b in vec![5,6,7,8,9] {
            if b == a {
                continue;
            }
            for c in vec![5,6,7,8,9] {
                if c == a || c == b {
                    continue;
                }
                for d in vec![5,6,7,8,9] {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in vec![5,6,7,8,9] {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let output = run_amplifiers_with_feedback(&memory, a, b, c, d, e);
                        if output > max_output {
                            max_output = output;
                        }
                    }
                }
            }
        }
    }
    println!("Max output with feedback: {}", max_output);

}

fn run_amplifiers_with_feedback(memory: &Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let mut input_value: i32 = 0;
    let mut computer_a = computer::Computer{
        memory: memory.clone(),
        pc: 0,
        input_pointer: 0,
        input: vec![a, input_value],
        is_halted: false,
    };
    let mut computer_b = computer::Computer{
        memory: memory.clone(),
        pc: 0,
        input_pointer: 0,
        input: vec![b, input_value],
        is_halted: false,
    };
    let mut computer_c = computer::Computer{
        memory: memory.clone(),
        pc: 0,
        input_pointer: 0,
        input: vec![c, input_value],
        is_halted: false,
    };
    let mut computer_d = computer::Computer{
        memory: memory.clone(),
        pc: 0,
        input_pointer: 0,
        input: vec![d, input_value],
        is_halted: false,
    };
    let mut computer_e = computer::Computer{
        memory: memory.clone(),
        pc: 0,
        input_pointer: 0,
        input: vec![e, input_value],
        is_halted: false,
    };
    let mut first_run = true;
    let mut output_value = 0;
    loop {
        if first_run{
            computer_a.set_input(vec![a, input_value]);
        } else {
            computer_a.set_input(vec![input_value]);
        }
        input_value = computer_a.run();
        if computer_a.is_halted {
            return output_value;
        }
        if first_run{
            computer_b.set_input(vec![b, input_value]);
        } else {
            computer_b.set_input(vec![input_value]);
        }
        input_value = computer_b.run();
        if first_run{
            computer_c.set_input(vec![c, input_value]);
        } else {
            computer_c.set_input(vec![input_value]);
        }
        input_value = computer_c.run();
        if first_run{
            computer_d.set_input(vec![d, input_value]);
        } else {
            computer_d.set_input(vec![input_value]);
        }
        input_value = computer_d.run();
        if first_run{
            computer_e.set_input(vec![e, input_value]);
        } else {
            computer_e.set_input(vec![input_value]);
        }
        input_value = computer_e.run();
        output_value = input_value;
        first_run = false;
    }
}

fn run_without_feedback(memory: &Vec<i32>) -> () {
    let mut max_output = 0;
    for a in vec![0,1,2,3,4] {
        for b in vec![0,1,2,3,4] {
            if b == a {
                continue;
            }
            for c in vec![0,1,2,3,4] {
                if c == a || c == b {
                    continue;
                }
                for d in vec![0,1,2,3,4] {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in vec![0,1,2,3,4] {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let output = run_amplifiers(&memory, a, b, c, d, e);
                        if output > max_output {
                            max_output = output;
                        }
                    }
                }
            }
        }
    }
    println!("Max output:               {}", max_output);
}

fn run_amplifiers(memory: &Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let mut memory_a = memory.clone();
    let mut memory_b = memory.clone();
    let mut memory_c = memory.clone();
    let mut memory_d = memory.clone();
    let mut memory_e = memory.clone();
    let mut input_value: i32 = 0;
    input_value = computer::run_computer(&mut memory_a, vec![a, input_value]);
    input_value = computer::run_computer(&mut memory_b, vec![b, input_value]);
    input_value = computer::run_computer(&mut memory_c, vec![c, input_value]);
    input_value = computer::run_computer(&mut memory_d, vec![d, input_value]);
    input_value = computer::run_computer(&mut memory_e, vec![e, input_value]);
    return input_value;
}
