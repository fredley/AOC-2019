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

pub trait Computes {
    fn set_memory(&mut self, memory: Vec<i64>) -> ();
    fn set_input(&mut self, input: Vec<i64>) -> ();
    fn run(&mut self) -> i64;
    fn new(memory: Vec<i64>, input: Vec<i64>) -> Computer;
}

pub struct Computer {
    pub memory: Vec<i64>,
    pub input: Vec<i64>,
    pub pc: i64,
    pub input_pointer: usize,
    pub is_halted: bool,
    pub relative_base: i64,
}

impl Computes for Computer {

    fn new(memory: Vec<i64>, input: Vec<i64>) -> Computer {
        return Computer{
            memory: memory.clone(),
            pc: 0,
            input_pointer: 0,
            input: input.clone(),
            is_halted: false,
            relative_base: 0,
        }
    }

    fn set_memory(&mut self, memory: Vec<i64>) -> () {
        self.memory = memory;
    }

    fn set_input(&mut self, input: Vec<i64>) -> () {
        self.input = input;
        self.input_pointer = 0;
    }

    fn run(&mut self) -> i64 {
        let mut output = 0;
        loop {
            let opcode = self.memory[self.pc as usize] % 100;
            if opcode == 99 {
                self.is_halted = true;
                return output;
            }
            let arg1: i64;
            let mode1 = (self.memory[self.pc as usize] / 100) % 10;
            if mode1 == 1{
                arg1 = self.memory[(self.pc + 1) as usize];
            } else if mode1 == 2 {
                arg1 = self.memory[(self.memory[(self.pc + 1) as usize] + self.relative_base) as usize];
            } else {
                arg1 = self.memory[self.memory[(self.pc + 1) as usize] as usize];
            }
            let arg2: i64;
            let mode2 = (self.memory[self.pc as usize] / 1000) % 10;
            if opcode == 3 || opcode == 4 {
                arg2 = 0;
            } else if mode2 == 1 {
                arg2 = self.memory[(self.pc + 2) as usize];
            } else if mode2 == 2 {
                arg2 = self.memory[(self.memory[(self.pc + 2) as usize] + self.relative_base) as usize];
            } else {
                arg2 = self.memory[self.memory[(self.pc + 2) as usize] as usize];
            }
            if opcode == 1 {
                // add
                let target = self.memory[(self.pc + 3) as usize] as usize;
                self.memory[target] = arg2 + arg1;
                self.pc += 4;
            } else if opcode == 2 {
                //multiply
                let target = self.memory[(self.pc + 3) as usize] as usize;
                self.memory[target] = arg2 * arg1;
                self.pc += 4;
            } else if opcode == 3 {
                //input
                let target = self.memory[(self.pc + 1) as usize] as usize;
                self.memory[target] = self.input[self.input_pointer];
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
                let target = self.memory[(self.pc + 3) as usize] as usize;
                if arg1 < arg2 {
                    self.memory[target] = 1;
                } else {
                    self.memory[target] = 0;
                }
                self.pc += 4;
            } else if opcode == 8 {
                //equals
                let target = self.memory[(self.pc + 3) as usize] as usize;
                if arg1 == arg2 {
                    self.memory[target] = 1;
                } else {
                    self.memory[target] = 0;
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
