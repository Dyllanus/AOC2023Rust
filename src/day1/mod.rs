use std::collections::HashMap;
use std::fs;

pub fn main() {
    let contents = fs::read_to_string("inputs/day1/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut total = 0;
    for line in lines {
        let mut reversed_line = String::from(line);
        reversed_line = reversed_line.chars().rev().collect::<String>();
        let mut all_maps : HashMap<usize, String> = HashMap::new();
        all_maps.extend(get_number_in_string(&line));
        all_maps.extend(get_number(line, false));
        all_maps.extend(get_number(&*reversed_line, true));
        let mut highest_index :usize = 0;
        let mut lowest_index:usize = 1000;
        for (key, _) in &all_maps{
            if key > &highest_index {
                highest_index = *key;
            }
            if key < &lowest_index {
                lowest_index = *key
            }
        }
        let high_s = all_maps.get(&highest_index).unwrap();
        let low_s = all_maps.get(&lowest_index).unwrap();
        let mut number = String::new();
        if low_s.len()<=1 {
            number.push(low_s.parse().unwrap());
        }else{
            number.push(word_to_char(low_s));
        }
        if high_s.len()<=1 {
            number.push(high_s.parse().unwrap());
        }else{
            number.push(word_to_char(high_s));
        }
        total += number.parse::<u32>().unwrap()
    }
    println!("Answer Day 1 = {total}");
}


fn get_number(s: &str, reverse : bool) -> HashMap<usize,String> {
    let mut ret_map : HashMap<usize, String> = HashMap::new();
    let mut index = 0;
    if reverse {
        index = s.len()-1
    }
    for char in s.chars() {
        if char.is_numeric() {
            ret_map.insert(index, String::from(char));
            return ret_map;
        }
        if reverse {
            if index as i32 - 1 > -1 {
                index -= 1
            }

        } else {
            index += 1
        }
    }
    return ret_map;
}

fn get_number_in_string(s: &str) -> HashMap<usize, String> {
    let array = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut map = HashMap::new();
    for number in array {
        let test = s.find(number);
        if test != None {
            map.insert(test.unwrap(),number);
        }
    }
    for number in array {
        let test = s.rfind(number);
        if test != None {
            map.insert(test.unwrap(),number);
        }
    }
    let mut highest_index = 0;
    let mut lowest_index = 1000;
    let mut ret_map: HashMap<usize, String> = HashMap::new();
    for (key, _value) in &map {
        if key > &highest_index {
            highest_index = *key;
        }
        if key < &lowest_index {
            lowest_index = *key
        }
    }
    if lowest_index != 1000 {
        ret_map.insert(lowest_index, String::from(*map.get(&lowest_index).unwrap()));
    }
    if highest_index != 0 {
        ret_map.insert(highest_index, String::from(*map.get(&highest_index).unwrap()));
    }
    return ret_map;
}

fn word_to_char(s : &str) -> char{
    return  match s {
        "one"=>'1',
        "two"=>'2',
        "three"=>'3',
        "four"=>'4',
        "five"=>'5',
        "six"=>'6',
        "seven"=>'7',
        "eight"=>'8',
        "nine"=>'9',
        _=>'0'
    };

}