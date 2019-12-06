use std::env;
use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod computer;
mod utils;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].clone().parse::<isize>().unwrap();
    if day == 0 {
        run_all();
    } else {
        run_day(day);
    }
}

fn run_day(day: isize) {
    let filename = format!("./src/inputs/{:?}.txt", day);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file").trim().to_string();
    println!("Day {}:", day);
    match day {
        1 => day_1::day_one(input),
        2 => day_2::day_two(input),
        3 => day_3::day_three(input),
        4 => day_4::day_four(input),
        5 => day_5::day_five(input),
        6 => day_6::day_six(input),
        _ => println!("Specify a day, or 0 to run all"),
    }
}

fn run_all() {
    let mut day: isize = 1;
    while day <= 6 {
        run_day(day);
        day += 1;
    }
}
