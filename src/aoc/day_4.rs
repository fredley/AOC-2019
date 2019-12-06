use crate::aoc::utils;

pub fn day_four(input: String) -> () {
    let parsed: Vec<i32> = input.split('-').map(utils::parse_int).collect();
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
