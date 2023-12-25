pub mod part2;
use std::fs;

struct Seeds {
    begin_seed: i64,
    range_seed: i64
}

pub fn main() {
    let mut collection_lines:Vec<&str> = Vec::new();
    let binding = fs::read_to_string("inputs/day5/input.txt").expect("Should have been able to read the file");
    let test = binding.split("\n").map(|s| String::from(s)).collect::<Vec<String>>();
    let mut seeds = test[0].split("seeds: ").map(|s| String::from(s)).collect::<Vec<String>>()[1].split(" ").map(|s| String::from(s)).collect::<Vec<String>>();
    for line in &test[2..] {
        if line.trim().is_empty() {
            seeds = map_inputs(collection_lines, &seeds);
            // println!();
            collection_lines = Vec::new();
            continue;
        }else if (line.as_bytes()[0] as char).is_numeric() {
            collection_lines.push(line)
        }
    }
    seeds = map_inputs(collection_lines, &seeds);
    let answer: i64 = seeds.iter().map(|s| s.trim().parse::<i64>().unwrap()).min().unwrap();
    println!("Answer Day 5 = {}", answer)
}

fn map_inputs(lines: Vec<&str>, sources:&Vec<String>) -> Vec<String> {
    let mut dests : Vec<String> = Vec::new();
    for source_s in sources {
        let source  = source_s.trim().parse::<i64>().unwrap();
        let mut dest = -1;
        'inner :for line in &lines {
            let split_line = line.split(" ").collect::<Vec<&str>>();
            let destination_range_start = split_line[0].trim().parse::<i64>().unwrap();
            let source_range_start = split_line[1].trim().parse::<i64>().unwrap();
            let range_length = split_line[2].trim().parse::<i64>().unwrap();
            let difference_source_dest = (source_range_start - destination_range_start);
            if !(source_range_start+range_length <= source || source_range_start > source) {
                dest = source-difference_source_dest;
                break 'inner;
            }
        }
        if dest == -1 {
            dest = source;
        }
        dests.push(dest.to_string());
    }
    dests
}