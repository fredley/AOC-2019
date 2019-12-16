use crate::aoc::utils;

fn get_offset(input: &Vec<i64>) -> i64 {
    let mut offset = 0;
    let mut i = 0;
    let ten: i64 = 10;
    while i < 7 {
        offset += input[i] * ten.pow((6 - i) as u32);
        i += 1;
    }
    return offset
}

pub fn day_sixteen(input: String) -> () {
    let mut vector: Vec<i64> = input.chars().map(|c| utils::parse_int_64(&c.to_string())).collect();
    let mut rounds = 0;
    while rounds < 100 {
        vector = fft(vector);
        // println!("{:?}", vector);
        rounds += 1;
    }
    println!("First 8 chars:     {:?}", &vector[0..8]);

    // part 2
    let vec2: Vec<i64> = input.chars().map(|c| utils::parse_int_64(&c.to_string())).collect();
    let offset: usize = get_offset(&vec2) as usize;
    let len = vec2.len();
    let mut bigvec: Vec<i64> = vec2.iter().cycle().take(len * 10000).map(|e| *e).collect();
    let result = upper_half_fft(&mut bigvec, 100);
    println!("Specified 8 chars: {:?}", &result[offset..offset+8]);
}

fn upper_half_fft(input: &mut Vec<i64>, iterations: i64) -> Vec<i64>{
    let length = input.len();
    let mut i = 0;
    let mut data = input.clone();
    while i < iterations {
        let mut vector: Vec<i64> = vec![0; length];
        vector[length - 1] = data[length - 1];
        let mut j = length - 2;
        while j > length / 2 {
            vector[j] = (data[j] + vector[j+1]).abs() % 10;
            j -= 1;
        }
        data = vector.clone();
        i += 1;
    }
    return data;
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
