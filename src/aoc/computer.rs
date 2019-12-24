use std::collections::HashMap;

pub fn run_computer(memory: &mut Vec<i32>, input: Vec<i32>) -> i32 {
    let mut pc = 0;
    let mut output = 0;
    let mut input_pointer = 0;
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
            memory[target] = input[input_pointer];
            input_pointer += 1;
            pc += 2;
        } else if opcode == 4 {
            //output
            output = arg1;
            return output;
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

pub struct Computer {
    pub memory: HashMap<i64, i64>,
    pub input: Vec<i64>,
    pub pc: i64,
    pub input_pointer: usize,
    pub is_halted: bool,
    pub relative_base: i64,
    pub requires_input: bool,
}

impl Computer {

    pub fn new(input: Vec<i64>) -> Computer {
        return Computer{
            memory: HashMap::new(),
            pc: 0,
            input_pointer: 0,
            input: input.clone(),
            is_halted: false,
            relative_base: 0,
            requires_input: false,
        }
    }

    pub fn set_memory(&mut self, memory: Vec<i64>) -> () {
        let mut i: i64 = 0;
        while i < memory.len() as i64 {
            self.memory.insert(i, memory[i as usize]);
            i+=1;
        }
    }

    pub fn set_input(&mut self, input: Vec<i64>) -> () {
        self.input = input;
        self.input_pointer = 0;
        self.requires_input = false;
    }

    pub fn set_input_ascii(&mut self, letters: String) -> () {
        let mut input: Vec<i64> = Vec::new();
        for letter in letters.chars() {
            input.push(letter as u8 as i64);
        }
        self.set_input(input);
    }

    pub fn run(&mut self) -> i64 {
        let mut output = 0;
        loop {
            let opcode = self.memory[&self.pc] % 100;
            if opcode == 99 {
                self.is_halted = true;
                return output;
            }
            let arg1: i64;
            let mode1 = (self.memory[&self.pc] / 100) % 10;
            if mode1 == 1{
                arg1 = *self.memory.entry(self.pc + 1).or_insert(0);
            } else if mode1 == 2 {
                let rel = *self.memory.entry(self.pc + 1).or_insert(0);
                arg1 = *self.memory.entry(rel + self.relative_base).or_insert(0);
            } else {
                let rel = *self.memory.entry(self.pc + 1).or_insert(0);
                arg1 = *self.memory.entry(rel).or_insert(0);
            }
            let arg2: i64;
            let mode2 = (self.memory[&self.pc] / 1000) % 10;
            if opcode == 3 || opcode == 4 || opcode == 9 {
                arg2 = 0;
            } else if mode2 == 1 {
                arg2 = *self.memory.entry(self.pc + 2).or_insert(0);
            } else if mode2 == 2 {
                let rel = *self.memory.entry(self.pc + 2).or_insert(0);
                arg2 = *self.memory.entry(rel + self.relative_base).or_insert(0);
            } else {
                let rel = *self.memory.entry(self.pc + 2).or_insert(0);
                arg2 = *self.memory.entry(rel).or_insert(0);
            }
            let mode3 = (self.memory[&self.pc] / 10000) % 10;
            if opcode == 1 {
                // add
                let mut target = self.memory[&(self.pc + 3)];
                if mode3 == 2 {
                    target += self.relative_base;
                }
                self.memory.insert(target, arg2 + arg1);
                self.pc += 4;
            } else if opcode == 2 {
                //multiply
                let mut target = self.memory[&(self.pc + 3)];
                if mode3 == 2 {
                    target += self.relative_base;
                }
                self.memory.insert(target, arg2 * arg1);
                self.pc += 4;
            } else if opcode == 3 {
                //input
                if self.input.len() <= self.input_pointer {
                    self.requires_input = true;
                    return -999;
                }
                let mut target = *self.memory.entry(self.pc + 1).or_insert(0);
                if mode1 == 2 {
                    target += self.relative_base;
                }
                self.memory.insert(target, self.input[self.input_pointer]);
                self.input_pointer += 1;
                self.pc += 2;
            } else if opcode == 4 {
                //output
                output = arg1;
                self.pc += 2;
                return output;
            } else if opcode == 5 {
                // jump if true
                if arg1 != 0 {
                    self.pc = arg2;
                } else {
                    self.pc += 3;
                }
            } else if opcode == 6 {
                // jump if false
                if arg1 == 0 {
                    self.pc = arg2;
                } else {
                    self.pc += 3;
                }
            } else if opcode == 7 {
                // less than
                let mut target = self.memory[&(self.pc + 3)];
                if mode3 == 2 {
                    target += self.relative_base;
                }
                if arg1 < arg2 {
                    self.memory.insert(target, 1);
                } else {
                    self.memory.insert(target, 0);
                }
                self.pc += 4;
            } else if opcode == 8 {
                //equals
                let mut target = self.memory[&(self.pc + 3)];
                if mode3 == 2 {
                    target += self.relative_base;
                }
                if arg1 == arg2 {
                    self.memory.insert(target, 1);
                } else {
                    self.memory.insert(target, 0);
                }
                self.pc += 4;
            } else if opcode == 9 {
                self.relative_base += arg1;
                self.pc += 2;
            } else {
                println!("Invalid memory!");
                break;
            }
        }
        return output;
    }
}
