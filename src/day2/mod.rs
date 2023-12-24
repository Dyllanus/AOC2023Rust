use std::collections::HashMap;
use std::fs;
pub fn main(){
    let contents = fs::read_to_string("inputs/day2/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut answer: i64 = 0;
    for line in lines {
        let split_s = line.split(": ").collect::<Vec<&str>>();
        let sets = split_s[1].split("; ").collect::<Vec<&str>>();
        let mut cubes_map: HashMap<String, i32> = HashMap::new();
        for set in sets {
            for blocks in set.split(", ") {
                let block_values = blocks.split(" ").collect::<Vec<&str>>();
                let number = block_values[0].parse::<i32>().unwrap();
                let block = block_values[1].trim();

                if !cubes_map.contains_key(block) {
                    cubes_map.insert(String::from(block), number);
                } else if cubes_map.get(block).unwrap() < &number {
                    cubes_map.insert(String::from(block), number);
                }
            }
        }
        let mut answer_game : i64 = 1;
        for (_, val) in cubes_map {
            answer_game *= val as i64;
        }
        answer += answer_game;
        cubes_map = HashMap::new();
    }
    println!("Answer Day 2 = {}", answer)
}
