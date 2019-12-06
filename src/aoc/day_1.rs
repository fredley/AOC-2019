pub fn day_one(input: String) -> () {
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
