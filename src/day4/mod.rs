use std::collections::{BTreeMap};
use std::fs;
pub fn main(){
    let contents = fs::read_to_string("inputs/day4/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut answer: u32 = 0;
    let mut all_cards : BTreeMap<u32, u32> = BTreeMap::new();
    let mut total_cards : BTreeMap<u32,u32> = BTreeMap::new();
    let mut card_number : u32 = 1;
    for line in lines {
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
        all_cards.insert(card_number, dups as u32);
        total_cards.insert(card_number, 1);
        card_number+=1;
    }

    for (k,v) in all_cards {
        // println!("card_number {k} : value {v}");
        for _ in 0..*total_cards.get(&k).unwrap(){
            for cards in k + 1..k + v + 1 {
                // println!("cardnr {cards} iets:{}", total_cards.get(&cards).unwrap() + 1);
                total_cards.insert(cards, total_cards.get(&cards).unwrap() + 1);
            }
        }
    }

    for (k,v)  in total_cards {
        // println!("card_number {k} : value {v}");
        answer+=v;
    }
    println!("Answer Day 4 = {}", {answer});
}
