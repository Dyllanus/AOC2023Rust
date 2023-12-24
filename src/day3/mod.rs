use std::fs;

pub fn main(){
    let symbols = ['!','@','#','$','%','^','&','*','(',')','-', '+','?','_','=','<','>','/' ];
    let contents = &fs::read_to_string("inputs/day3/sample.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut answer: i64 = 0;
    let mut all_numbers = load_all_numbers(contents.parse().unwrap());
    for line in lines {
        for char in line.chars() {
            if symbols.contains(&char)  {
                let mut total_symbol_value = 0;
                println!("X: {}, Y: {} symbol: {}", x,y,char);
                // check above
                for index in x-3..x+2{
                    // print!("{}", all_numbers[y -1][index ])
                }

                //check left side
                if all_numbers[y][x-1].is_numeric() {
                    let mut number = String::new();
                    // should be a while loop
                    'check_left_side : for index in (x-3..x).rev() {
                        if all_numbers[y][index].is_numeric(){
                            number.insert(0, all_numbers[y][index]);
                        }else{
                            break 'check_left_side;
                        }
                    }
                    if !number.is_empty() {
                        total_symbol_value += number.parse::<i32>().unwrap();
                    }
                    println!("{number}");
                }
                //check right side
                if all_numbers[y][x+1].is_numeric() {
                    let mut number = String::new();
                    // should be a while loop
                    'check_right_side : for index in (x+1..x+4) {
                        if all_numbers[y][index].is_numeric(){
                            number.push(all_numbers[y][index]);
                        }else{
                            break 'check_right_side
                        }
                    }
                    if !number.is_empty() {
                        total_symbol_value += number.parse::<i32>().unwrap();
                    }
                    println!("{number}")
                }
                // check bottom


            }
            x+=1
        }
        x=0;
        y+=1;

    }
    println!("Answer Day 3 = {answer}")
}

fn load_all_numbers(contents: String) -> Vec<Vec<char>>{
    let mut ret_vec : Vec<Vec<char>> = Vec::new();
    let lines = contents.split("\n");;
    for line in lines {
        let mut line_vec : Vec<char> = Vec::new();
        let trim_line = line.trim();
        for char in trim_line.chars() {
            line_vec.push(char)
        }
        ret_vec.push(line_vec);
    }
    return ret_vec;
}

