use std::{env, fs};
use std::time::Instant;

fn main() {
    println!("Day 06 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let now = Instant::now();

    let lines: Vec<&str> = input.split('\n').collect();
    let times = lines[0].split_whitespace().collect::<Vec<&str>>()[1..].to_vec();
    let distances = lines[1].split_whitespace().collect::<Vec<&str>>()[1..].to_vec();

    // (race time, distance record)
    let mut races: Vec<(usize, usize)> = Vec::new();
    for i in 0..times.len() {
        let time_limit = times[i].parse().unwrap();
        let distance_record = distances[i].parse().unwrap();
        races.push((time_limit, distance_record));
    }

    // oki doki boomer
    // *cracks knuckles
    let mut multiplied_races = 1;
    for (time_limit, distance_record) in races {
        let mut winning_times = 0;
        for hold_time in 0..time_limit {
            if get_distance(time_limit, hold_time) > distance_record {
                winning_times += 1;
            }
        }
        multiplied_races *= winning_times;
    }
    let part_1_us = now.elapsed().as_micros();
    let now = Instant::now();

    // part 2 uwu
    let mut long_time= String::new();
    let mut long_distance = String::new();
    for i in 0..times.len() {
        long_time += times[i];
        long_distance += distances[i];
    }
    let long_time: usize = long_time.parse().unwrap();
    let long_distance: usize = long_distance.parse().unwrap();

    let mut winning_times = 0;
    for hold_time in 1..long_time {
        if get_distance(long_time, hold_time) > long_distance {
            winning_times += 1;
        }
    }

    let part_2_us = now.elapsed().as_micros();

    println!("Margin for winning: {multiplied_races}");
    println!("Took {part_1_us}µs");
    println!("Ways to win: {winning_times}");
    println!("Took {part_2_us}µs");
}

fn get_distance(time_limit: usize, hold_length: usize) -> usize {
    // no underflow pls
    if hold_length > time_limit { return 0 }
    let travel_time = time_limit - hold_length;
    // hold_length is equivalent to speed
    return travel_time * hold_length;
}