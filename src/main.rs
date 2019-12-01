use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    static FILENAME:&'static str = "./src/inputs/1.txt";

    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");

    let inputs = contents.split_ascii_whitespace();

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