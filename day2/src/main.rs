use std::{env, fs};

fn main() {
    println!("Day 02 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    // println!("Path: {}", path);
    let input = fs::read_to_string(path).expect("Error reading file");

    // parse games
    let mut games: Vec<Game> = vec![];
    for line in input.split('\n') {
        if line.is_empty() { continue }
        games.push(parse_game(line.to_string()))
    }

    // do the ID maths
    // limits are 12r, 13g, 14b
    let mut total_ids: usize = 0;
    'games: for game in &games {
        for draw in &game.draws {
            if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
                // game breaks limits
                println!("Game {}: Invalid", game.id);
                continue 'games
            }
        }

        println!("Game {}: Valid", game.id);
        total_ids += game.id
    }

    // do the power maths
    let mut total_power: usize = 0;
    for game in games {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for draw in game.draws {
            red = usize::max(red, draw.red);
            green = usize::max(green, draw.green);
            blue = usize::max(blue, draw.blue);
        }
        total_power += red * green * blue;
    }

    println!("Sum of IDs: {total_ids}");
    println!("Sum of Power: {total_power}")
}

struct Game {
    id: usize,
    draws: Vec<Draw>
}

struct Draw {
    red: usize,
    green: usize,
    blue: usize
}

fn parse_game(game: String) -> Game {
    let mut output = Game {
        id: parse_game_id(&game),
        draws: vec![]
    };

    // remove "Game N: "
    let game = game.split(':').nth(1).unwrap().to_string();

    // parse individual draws
    let draws: Vec<&str> = game.split(';').collect();
    output.draws = parse_draws(draws);

    output
}

fn parse_game_id(game: &String) -> usize {
    let split: Vec<&str> = game.split(' ').collect();
    let number = split[1].to_string();
    let number = number[..number.len()-1].to_string();

    number.parse::<usize>().expect("ID was not a number")
}

fn parse_draws(draws: Vec<&str>) -> Vec<Draw> {
    let mut output: Vec<Draw> = vec![];

    for draw in draws {
        let colors: Vec<&str> = draw.split(',').collect();
        let mut draw = Draw {red: 0, green: 0, blue: 0};
        for color_draw in colors {
            let split: Vec<&str> = color_draw.split(' ').collect();
            let count = split[1];
            let color = split[2];

            if color == "red" {
                draw.red = count.parse().expect(format!("Couldn't parse '{}' to int", count).as_str())
            } else if color == "green" {
                draw.green = count.parse().expect(format!("Couldn't parse '{}' to int", count).as_str())
            } else if color == "blue" {
                draw.blue = count.parse().expect(format!("Couldn't parse '{}' to int", count).as_str())
            } else {
                panic!("color '{}' wasn't recognised!", color)
            }
        }

        output.push(draw);
    }

    output
}