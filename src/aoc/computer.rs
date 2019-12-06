pub fn run_computer(memory: &mut Vec<i32>, input: i32) -> i32 {
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
