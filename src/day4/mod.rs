use std::fs;
pub fn main(){
    let contents = fs::read_to_string("inputs/day4/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut answer: i32 = 0;
    for line in lines {
        let mut card_answer = 0;
        let split_line : Vec<&str> = line.split(" | ").collect();
        let mut winning_numbers =  Vec::new();
        for s in split_line[0].split(": ").collect::<Vec<&str>>()[1].trim().split(" ") {
            if !s.is_empty() {
                winning_numbers.push(s.trim());
            }
        }
        let mut my_nums = Vec::new();
        for s in split_line[1].trim().split(" ") {
            if !s.is_empty() {
                my_nums.push(s.trim());
            }
        }
        let dups = my_nums.iter().filter(|num| winning_numbers.contains(&num.trim())).count();
        if dups != 0 {
            card_answer = 2_i32.pow((dups-1) as u32);
        }
        println!("{dups} {}", card_answer);
        answer+=card_answer;
    }
    println!("Answer Day 4 = {}", {answer})
}
