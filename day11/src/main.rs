use std::{env, fs};
use std::time::Instant;

fn main() {
    println!("Day 11 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    let pretty_output: bool = env::args().nth(2).unwrap_or("false".to_string()).parse().unwrap();
    let input = fs::read_to_string(path).expect("Unable to read file");
    let time = Instant::now();


    // parse to grid
    let lines: Vec<&str> = input.split('\n').collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if line.is_empty() { continue }
        let row: Vec<char> = line.chars().collect();
        grid.push(row)
    }

    if pretty_output {
        print_grid(&grid);
        println!("Correcting for cosmic expansion...");
    };

    // find empty rows and expand
    let mut indices: Vec<usize> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if !row.contains(&'#') {
            indices.push(i)
        }
    }
    let empty_row: Vec<char> = vec!['.'; grid[0].len()];
    for i in indices.iter().rev() {
        grid.insert(*i, empty_row.clone());
    }
    // find empty columns and expand
    let mut indices: Vec<bool> = vec![true; grid[0].len()];
    for row in grid.iter() {
        for (i, c) in row.iter().enumerate().rev() {
            if c == &'#' {
                indices[i] = false;
            }
        }
    }
    for row in grid.iter_mut() {
        for (i, b) in indices.iter().enumerate().rev() {
            if *b {
                row.insert(i, '.');
            }
        }
    }
    if pretty_output {
        print_grid(&grid);
        println!("Mapping galaxies...");
    }

    // map galaxies
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push(Galaxy{ index: galaxies.len()+1, x, y});
            }
        }
    }

    // print galaxies
    if pretty_output {
        for g in &galaxies {
            println!("{}: [{},{}]", g.index, g.x, g.y);
        }
        println!("Mapping routes between galaxies...");
    }

    // map routes between galaxies
    // make sure we dont double dip - only include a->b, not b->a
    let mut sum = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for j in i+1..galaxies.len() {
            let g2 = &galaxies[j];
            let dist = get_distance(g1, g2);
            // if pretty_output { println!("G{} -> G{}: {}", g1.index, g2.index, dist) }
            sum += dist;
        }
    }

    let p1_us = time.elapsed().as_micros();
    println!("Sum: {sum}");
    println!("Took {p1_us}µs")
}

struct Galaxy {
    index: usize,
    x: usize,
    y: usize
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();
    let h_padding = height.to_string().len();
    println!("{}{: ^width$}", " ".repeat(h_padding), "[ GRID ]", width = width + h_padding);
    for (i, row) in grid.iter().enumerate() {
        let beginning = if i % 2 == 0 {
            format!("{:>width$} ", i, width = h_padding)
        } else {
            format!("{:>width$}", " ", width = h_padding + 1)
        };
        let body: String = row.iter().collect();
        println!("{beginning}{body} {beginning}")
    }
}

fn get_distance(g1: &Galaxy, g2:& Galaxy) -> usize {
    let h_dist = usize::abs_diff(g1.x, g2.x);
    let v_dist = usize::abs_diff(g1.y, g2.y);
    return h_dist + v_dist;
}