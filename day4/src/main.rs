use std::{env, fs};
use std::time::Instant;

fn main() {
    println!("Day 04 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    let input = fs::read_to_string(path).expect("Error reading file");

    let now = Instant::now();

    // parse games
    let mut games: Vec<Game> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() { continue }
        // Game X: XX XX XX XX XX | XX XX XX XX XX XX XX XX
        let id = get_game_id(line);
        let line = line.split_once(": ").unwrap().1;
        let (first_half, second_half) = line.split_once(" | ").expect("Error splitting line");
        let winning_numbers = get_numbers(first_half.to_string());
        let have_numbers = get_numbers(second_half.to_string());

        games.push(Game { id, winning_numbers, have_numbers, points: 0 })
    }

    // get wins
    let mut total_points: usize = 0;
    for mut game in games {
        let mut matches = 0;
        for num in game.winning_numbers {
            if game.have_numbers.contains(&num) {
                matches += 1;
            }
        }
        if matches > 0 {
            game.points = usize::pow(2, matches - 1);
            total_points += game.points
        }

        // println!("Game {}: {} points", game.id, game.points)
    }

    let time = now.elapsed().as_micros();
    println!("Total points: {total_points}");
    println!("Took {time}µs");
}

struct Game {
    id: usize,
    winning_numbers: Vec<usize>,
    have_numbers: Vec<usize>,
    points: usize
}

fn get_game_id(line: &str) -> usize {
    let beginning = line.split(": ").nth(0).expect("Error parsing line ID");
    let digits = beginning.replace("Card", "");
    let digits = digits.split_whitespace().nth(0).unwrap();
    return digits.parse().expect("Error parsing int from digits")
}

fn get_numbers(winning_nums: String) -> Vec<usize> {
    let mut winning_numbers: Vec<usize> = Vec::new();
    let winning_digits: Vec<&str> = winning_nums.split_whitespace().collect();
    for digits in winning_digits {
        let number: usize = digits.parse().expect("Unable to parse digits");
        winning_numbers.push(number);
    }

    return winning_numbers
}