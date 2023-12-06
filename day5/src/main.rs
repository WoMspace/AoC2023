use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use std::ops::Range;

fn main() {
    println!("Day 05 | Rust");
    let path = env::args().nth(1).expect("Missing required argument");
    let input = read_to_string(path).expect("Error reading file");

    let now = Instant::now();

    // parse seed ids
    let lines: Vec<&str> = input.split('\n').collect();
    let seeds: Vec<u64> = lines[0].replace("seeds: ", "").split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

    let groups: Vec<&str> = input.split("\n\n").collect();
    // remove seed ids
    let groups = groups[1..groups.len()].to_vec();

    // parse maps
    let mut seed_soil = Map { ranges: vec![] };
    let mut soil_fertilizer= Map { ranges: vec![] };
    let mut fertilizer_water= Map { ranges: vec![] };
    let mut water_light= Map { ranges: vec![] };
    let mut light_temp= Map { ranges: vec![] };
    let mut temp_humidity= Map { ranges: vec![] };
    let mut humidity_location = Map { ranges: vec![] };
    for group in groups {
        let lines: Vec<&str> = group.split('\n').collect();
        let len = lines.len();
        match lines[0] {
            "seed-to-soil map:" => {
                // println!("Calculating Seed -> Soil map");
                seed_soil = parse_map(lines[1..len].to_vec());
            },
            "soil-to-fertilizer map:" => {
                // println!("Calculating Soil -> Fertilizer map");
                soil_fertilizer = parse_map(lines[1..len].to_vec());
            },
            "fertilizer-to-water map:" => {
                // println!("Calculating Fertilizer -> Water map");
                fertilizer_water = parse_map(lines[1..len].to_vec());
            },
            "water-to-light map:" => {
                // println!("Calculating Water -> Light map");
                water_light = parse_map(lines[1..len].to_vec());
            },
            "light-to-temperature map:" => {
                // println!("Calculating Light -> Temperature map");
                light_temp = parse_map(lines[1..len].to_vec());
            },
            "temperature-to-humidity map:" => {
                // println!("Calculating Temperature -> Humidity map");
                temp_humidity = parse_map(lines[1..len].to_vec());
            },
            "humidity-to-location map:" => {
                // println!("Calculating Humidity -> Location map");
                humidity_location = parse_map(lines[1..len].to_vec());
            },
            _ => panic!("Found unexpected line!")
        }
    }

    let mut locations: Vec<u64> = Vec::new();
    let mut closest_location = 0;
    for seed in seeds {
        let soil = lookup(seed, &seed_soil);
        let fertilizer = lookup(soil, &soil_fertilizer);
        let water = lookup(fertilizer, &fertilizer_water);
        let light = lookup(water, &water_light);
        let temperature = lookup(light, &light_temp);
        let humidity = lookup(temperature, &temp_humidity);
        let location = lookup(humidity, &humidity_location);

        // println!("Seed {seed}: soil {soil}, fertilizer {fertilizer}, water {water}, light {light}, temperature {temperature}, humidity {humidity}, location {location}");

        locations.push(location);
        closest_location = u64::max(closest_location, location);
    }

    for location in locations {
        closest_location = u64::min(closest_location, location);
    }

    let part1_us = now.elapsed().as_micros();

    println!("Closest location: {closest_location}");
    println!("Took {part1_us}µs")
}

fn parse_map(lines: Vec<&str>) -> Map {
    let mut map = Map { ranges: Vec::new() };

    for line in lines {
        if line.is_empty() { continue }
        let nums: Vec<u64> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
        let (dst_start, src_start, length) = (nums[0], nums[1], nums[2]);

        let src_end = src_start + length;
        let offset = dst_start as i64 - src_start as i64;
        map.ranges.push((src_start..src_end, offset))
    }

    return map;
}

struct Map {
    // source range, and offset of the range
    ranges: Vec<(Range<u64>, i64)>
}

fn lookup(value: u64, map: &Map) -> u64 {
    for (range, offset) in &map.ranges {
        if range.contains(&value) {
            return (value as i64 + offset) as u64
        }
    }

    return value;
}