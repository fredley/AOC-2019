use crate::aoc::computer;
use crate::aoc::utils;

use std::io::{stdin,stdout,Write};

pub fn day_twenty_five(input: String) -> () {
    let memory: Vec<i64> = input.split(",").map(utils::parse_int_64).collect();
    let mut computer = computer::Computer::new(vec![]);
    computer.set_memory(memory.clone());
    let known_commands: Vec<&str> = vec![
        "\n",
        "north\n", 
        "take mutex\n",
        "east\n", 
        "east\n", 
        "east\n", 
        "west\n", 
        "west\n", 
        "west\n", 
        "south\n", 
        "west\n",
        "take space law space brochure\n",
        "north\n",
        "south\n",
        "south\n",
        "take hologram\n",
        "west\n",
        "take manifold\n",
        "east\n",
        "north\n",
        "east\n",
        "south\n",
        "west\n",
        "south\n",
        "south\n",
        "south\n",
    ];
    let mut command_pointer = 0;
    loop {
        let mut s = String::new();
        let mut valid_instruction = false;
        if known_commands.len() > command_pointer {
            s = known_commands[command_pointer].to_string();
            print!("{}", s);
            command_pointer += 1;
        } else {
            while !valid_instruction {
                print!("Next instruction: ");
                let _=stdout().flush();
                stdin().read_line(&mut s).expect("Did not enter a correct string");
                valid_instruction = match s.as_ref() {
                    "north\n" => true,
                    "south\n" => true,
                    "east\n" => true,
                    "west\n" => true,
                    "inv\n" => true,
                    _ => false,
                };
                if s[..4].to_string() == "drop".to_string() || s[..4].to_string() == "take".to_string() {
                    valid_instruction = true
                }
            }
        }
        computer.set_input_ascii(s);
        while !computer.requires_input{
            print!("{}", computer.run() as u8 as char);
        }
    }
}
