use std::{env, fs};
use std::time::Instant;

fn main() {
    println!("Day 09 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    let input = fs::read_to_string(path).expect("Unable to read input");
    let time = Instant::now();

    let mut forecast_sum = 0;
    for  line in input.split('\n') {
        if line.is_empty() { continue }
        let mut sequences: Vec<Vec<isize>> = Vec::new();
        sequences.push(line.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect());
        // I show you how deep the rabbit hole goes
        let mut seq_index = 0;
        'sequences: loop {
            let mut subseq: Vec<isize> = Vec::new();
            let mut is_zero = true;
            // construct the subsequence
            for i in 0..sequences[seq_index].len()-1 {
                let diff = &sequences[seq_index][i + 1] - &sequences[seq_index][i];
                subseq.push(diff);
                if diff != 0 { is_zero = false }
            }
            sequences.push(subseq);
            if is_zero {
                // Martha, I'm coming home sweetie!
                break 'sequences
            }
            // I'm back in the fking building again!?
            seq_index += 1;
        }

        // We found the end of the rabbit hole.
        let mut predictions: Vec<isize> = vec![0];
        'forecast: for (i, seq) in sequences.iter().rev().enumerate() {
            if i == 0 {
                continue 'forecast
            }
            let last_prediction = predictions.last().unwrap();
            let prediction = last_prediction + seq.last().unwrap();
            predictions.push(prediction);
            if i == sequences.len() - 1 {
                forecast_sum += prediction;
            }
        }
    }
    let part1_us = time.elapsed().as_micros();

    println!("Sum: {forecast_sum}");
    println!("Took {part1_us}µs");
}