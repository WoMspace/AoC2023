use std::{env, fs};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("Day 08 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    let input = fs::read_to_string(path).expect("Unable to read file");
    let lines: Vec<&str> = input.split('\n').collect();

    let time = Instant::now();

    let directions: Vec<char> = lines[0].chars().collect();
    let mut tree: HashMap<&str, Node> = HashMap::new();
    for line in lines[2..].iter() {
        if line.is_empty() { continue }
        // "ABC = (DEF, GHI)"
        let segments: Vec<&str> = line.split_whitespace().collect();
        let name = segments[0];
        let left = segments[2].replace('(',"").replace(',', "");
        let right = segments[3].replace(')',"").replace(',', "");
        tree.insert(name, Node { left, right });
    }

    let mut steps = 0;
    let mut current_node = &tree["AAA"];
    let mut direction_index = 0;
    loop {
        // this should just work
        let next_direction = directions[direction_index % directions.len()];
        direction_index += 1;
        let next_node = match next_direction {
            'R' => current_node.right.as_str(),
            'L' => current_node.left.as_str(),
            _ => unreachable!()
        };
        steps += 1;
        if next_node == "ZZZ" { break }
        current_node = &tree[next_node];
    }
    let part1_us = time.elapsed().as_micros();

    println!("Steps: {steps}");
    println!("Took {part1_us}µs");
}

struct Node {
    left: String,
    right: String
}