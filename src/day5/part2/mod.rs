use std::fs;

#[derive(Debug)]
struct Seeds {
    begin_seed: i64,
    range_seed: i64
}

pub fn main() {
    let mut collection_lines:Vec<&str> = Vec::new();
    let binding = fs::read_to_string("inputs/day5/sample.txt").expect("Should have been able to read the file");
    let test = binding.split("\n").map(|s| String::from(s)).collect::<Vec<String>>();
    let mut seeds = load_seeds(&test[0]);
    dbg!(seeds);
    for line in &test[2..] {
        if line.trim().is_empty() {
            seeds = map_inputs(collection_lines, &seeds);
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
fn load_seeds(s:&String) -> Vec<Seeds> {
    let seeds = s.split("seeds: ").map(|s| String::from(s)).collect::<Vec<String>>()[1].split(" ").map(|s| String::from(s)).collect::<Vec<String>>();
    let mut seeds_vec: Vec<Seeds> = Vec::new();
    for index in (0..seeds.len()).step_by(2){
        let seed = Seeds{
            begin_seed:seeds[index].parse::<i64>().unwrap(),
            range_seed:seeds[index+1].trim().parse::<i64>().unwrap()
        };
        seeds_vec.push(seed)
    }
    seeds_vec
}

fn map_inputs(lines: Vec<&str>, sources:&Vec<Seeds>) -> Vec<Seeds> {
    let mut dests : Vec<Seeds> = Vec::new();
    for source_s in sources {
        for line in lines {

        }
    }
    dests
}