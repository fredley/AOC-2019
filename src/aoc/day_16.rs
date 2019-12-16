use crate::aoc::utils;

fn get_offset(input: &Vec<i64>) -> i64 {
    let mut offset = 0;
    let mut i = 0;
    let ten: i64 = 10;
    while i < 7 {
        offset += input[i] * ten.pow((6 - i) as u32);
    }
    return offset
}

pub fn day_sixteen(input: String) -> () {
    let mut vector: Vec<i64> = input.chars().map(|c| utils::parse_int_64(&c.to_string())).collect();
    println!("Len: {}", vector.len());
    let mut rounds = 0;
    while rounds < 100 {
        vector = fft(vector);
        // println!("{:?}", vector);
        rounds += 1;
    }
    println!("First 8 chars: {:?}", &vector[0..8]);

    // part 2
    let mut vec2: Vec<i64> = input.chars().map(|c| utils::parse_int_64(&c.to_string())).collect();
    let offset = get_offset(&vec2);
    println!("Offest: {}", offset);
    rounds = 0;
    while rounds < 100 {
        vec2 = fft(vec2);
    }
    println!("First 8 of result: {:?}", &vec2[0..8]);
}

fn fft (input: Vec<i64>) -> Vec<i64> {
    let base_pattern = vec![0,1,0,-1];
    let mut output: Vec<i64> = Vec::new();
    let mut i = 1;
    let len = input.len();
    while i <= len {
        let mut idx = 0;
        let mut output_num = 0;
        let mut j = 1;
        for num in &input {
            if j % i == 0 {
                idx = (idx + 1) % 4;
            }
            output_num += num * base_pattern[idx];
            // print!("{} x {} + ", num, base_pattern[idx]);
            j += 1;
        }
        output_num = ((output_num.abs()% 10) + 10) % 10;
        output.push(output_num);
        // print!(" => {}\n", output_num);
        i += 1;
    }
    return output;
}