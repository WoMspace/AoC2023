use std::{env, fs};
use std::time::Instant;

fn main() {
    println!("Day 01 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    // println!("Path: {}", path);
    let input = fs::read_to_string(path).expect("Error reading file");

    let before = Instant::now();
    let input = find_replace(input);

    let mut total = 0;
    for line in input.split('\n') {

        let chars: Vec<char> = line.chars().collect();
        if chars.len() == 0 { continue }

        // get first digit of line
        let mut i = 0;
        let first_num: char = 'inner: loop {
            let c = chars[i];
            if c.is_ascii_digit() {
                break 'inner c
            }
            i += 1;
        };

        // get last digit of line
        i = chars.len() - 1;
        let last_num: char = 'inner: loop {
            let c: char = chars[i];
            if c.is_ascii_digit() {
                break 'inner c
            }
            i -= 1;
        };

        // sum digits and continue
        let calibration_code = format!("{}{}", first_num, last_num);
        total += calibration_code.parse::<usize>().unwrap();
    }
    let duration = before.elapsed();

    println!("Sum: {}", total);
    println!("Took {} seconds", duration.as_secs_f32())
}

fn find_replace(input: String) -> String {
    let mut replaced: String = String::new();
    for i in 0..input.len()-1 {

        // println!("{}", input.len() - i > 3);
        // println!("{}", &input[i..i+3]);

        let digit: char;
        if input.len() - i > 3 && &input[i..i+3] == "one" { digit = '1' }
        else if input.len() - i > 3 && &input[i..i+3] == "two" { digit = '2' }
        else if input.len() - i > 5 && &input[i..i+5] == "three" { digit = '3' }
        else if input.len() - i > 4 && &input[i..i+4] == "four" { digit = '4' }
        else if input.len() - i > 4 && &input[i..i+4] == "five" { digit = '5' }
        else if input.len() - i > 3 && &input[i..i+3] == "six" { digit = '6' }
        else if input.len() - i > 5 && &input[i..i+5] == "seven" { digit = '7' }
        else if input.len() - i > 5 && &input[i..i+5] == "eight" { digit = '8' }
        else if input.len() - i > 4 && &input[i..i+4] == "nine" { digit = '9' }
        else { digit = input.clone().chars().nth(i).unwrap() }

        replaced = format!("{}{}", replaced, digit);
    }

    replaced
}