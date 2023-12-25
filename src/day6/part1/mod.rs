use std::fs;
use log::debug;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64
}

pub fn main() {
    let binding = fs::read_to_string("inputs/day6/input.txt").expect("Should have been able to read the file");
    let contents = binding.split("\n").collect::<Vec<&str>>();
    let mut races : Vec<Race> = Vec::new();
    let time_line = contents[0].strip_prefix("Time: ").unwrap().trim()
        .split(" ").filter(|s|!s.is_empty())
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let distance_line = contents[1].strip_prefix("Distance: ").unwrap().trim()
        .split(" ").filter(|s|!s.is_empty())
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut index = 0;
    while index<time_line.len() {
        races.push(Race{
            time:time_line[index],
            distance:distance_line[index]
        });
        index+=1;
    }
    let mut answer:i64 = 1;
    for race in races {
        answer *= check_ways_to_beat_record(race);

    }
    println!("{answer}")
}

fn check_ways_to_beat_record(race:Race) -> i64 {
    let mut least_time_pos = 0;
    for i in 1..race.time {
        if (race.time - i) * i > race.distance {
            least_time_pos = i;
            break;
        }
    }
    (race.time - least_time_pos) + 1 - least_time_pos
}
