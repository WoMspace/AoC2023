use std::{env, fs};
use std::time::Instant;

fn main() {
    println!("Day 03 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    // println!("Path: {}", path);
    let input = fs::read_to_string(path).expect("Error reading file");

    let now = Instant::now();

    // split input into lines
    let mut lines: Vec<String> = Vec::new();
    for line in input.split('\n') {
        lines.push(line.to_string())
    }

    // put chars in grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for y in 0..&lines.len() - 0 {
        grid.push(lines[y].chars().collect());
    }

    // print grid
    // for line in &grid {
    //     let mut string: String = String::new();
    //     for c in line {
    //         string += c.to_string().as_str();
    //     }
    //     println!("{string}")
    // }

    // get a list of numbers and their starting coords
    let mut nums: Vec<Number> = Vec::new();
    let width = grid[0].len();
    let height = grid.len();
    let mut i = 0;
    // Iterate over every number
    while i < width*height {
        let (x, y) = get_coord(i, width, height);
        let c = grid[y][x];
        if !c.is_ascii_digit() {
            i += 1;
            continue
        }
        // if you got here, you have a number
        let mut num = Number { value: 0, x, y };
        let mut digits = String::from(c);
        'same_num: loop {
            i += 1;
            let (dx, dy) = get_coord(i, width, height);
            if dy != y { break 'same_num }
            let ch = grid[dy][dx];
            if !ch.is_ascii_digit() { break 'same_num }
            digits += ch.to_string().as_str()
        }
        num.value = digits.parse::<usize>().expect(format!("Couldn't parse '{digits}' to int").as_str());
        nums.push(num);
    }

    // check each number sequence for a symbol
    let mut sum = 0;
    'nums: for n in nums {
        let len = n.value.to_string().len();
        for x in n.x..n.x+len {
            if check_around(x, n.y, &grid) {
                // quick maffs
                sum += n.value;
                // println!("{}: Valid", n.value);
                continue 'nums
            }
        }
        // println!("{}: Invalid", n.value);
    }

    let time = now.elapsed().as_micros();

    println!("Sum of part numbers: {sum}");
    println!("Took {time}µs")
}

struct Number {
    value: usize,
    x: usize,
    y: usize
}

// check the 3x3 around the given coordinate (and transparently handle out-of-bounds)
fn check_around(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    let xmin = isize::max(x as isize - 1, 0) as usize;
    let xmax = usize::min(x + 1, grid[0].len() - 1);
    let ymin = isize::max(y as isize - 1, 0) as usize;
    let ymax = usize::min(y + 1, grid.len() - 1);

    for dx in xmin..=xmax {
        for dy in ymin..=ymax {
            if valid_symbol(grid[dy][dx]) { return true }
        }
    }

    return false
}

fn valid_symbol(c: char) -> bool {
    // return match c {
    //     '*' | '#' | '+' | '$' | '-' | '/' | '%' | '=' | '@' => true,
    //     _ => false
    // }
    return !c.is_ascii_digit() && c != '.'
}

fn get_coord(index: usize, width: usize, height: usize) -> (usize, usize) {
    let x = index % width;
    let y = (index - x) / height;
    return (x, y);
}