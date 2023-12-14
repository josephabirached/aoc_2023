use std::{char, collections::HashMap, usize};

pub fn get_isolated_numbers(content: String) -> Vec<String> {
    let mut valid_numbers = vec![];
    
    let content = content.replace(" ", "");
    let max_height = content.lines().count()-1;
    let max_width = content.lines().max().unwrap().chars().count()-1;

    let mut current_height = 0;
    let lines: Vec<&str> = content.split("\n").collect();
    for line in content.lines(){
        let line_numbers: HashMap<usize ,String> = get_line_numbers(line);

        for (index, line_number) in line_numbers.into_iter(){
            let number_size = line_number.len();
            for i in 0..number_size{
                let current_index = index + i;
                let mut is_number_valid = false;
                if current_height != 0 {
                    // Check top
                    let top_line = lines[current_height-1];
                    is_number_valid = is_number_valid || is_current_char_symbol(top_line.chars().nth(current_index).unwrap());
                         
                    if current_index != 0{
                        // Check top diagonal left
                        is_number_valid = is_number_valid || is_current_char_symbol(top_line.chars().nth(current_index-1).unwrap());
                    }
                    if current_index != max_width{
                        // Check top diagonal right
                        is_number_valid = is_number_valid || is_current_char_symbol(top_line.chars().nth(current_index+1).unwrap());
                    }
                } 
                
                if current_index != 0{
                    // Check left
                    is_number_valid = is_number_valid || is_current_char_symbol(line.chars().nth(current_index-1).unwrap());
                }
                if current_index != max_width{
                    // Check right
                    is_number_valid = is_number_valid || is_current_char_symbol(line.chars().nth(current_index+1).unwrap());
                }
                if current_height != max_height{
                       // Check bottom
                    let bottom_line = lines[current_height+1];
                    is_number_valid = is_number_valid || is_current_char_symbol(bottom_line.chars().nth(current_index).unwrap());

                    if current_index != 0{
                        // Check bottom diagonal left
                        is_number_valid = is_number_valid || is_current_char_symbol(bottom_line.chars().nth(current_index-1).unwrap());
                    }
                    if current_index != max_width{
                        // Check bottom diagonal right
                        is_number_valid = is_number_valid || is_current_char_symbol(bottom_line.chars().nth(current_index+1).unwrap());
                    }
                }

                if is_number_valid{
                    valid_numbers.push(line_number.clone());
                    break;
                }
            }
        }
        current_height += 1; 
    }
    valid_numbers
}

pub fn answer(calibrations: Vec<String>) -> u32 {
    let mut sum_calibration: u32 = 0;
    for calib in calibrations {
        sum_calibration += calib.parse::<u32>().unwrap();
    }
    sum_calibration
}

fn is_current_char_symbol(current_char: char) -> bool{
    return current_char != '.' && !current_char.is_digit(10);
}

fn get_line_numbers(content: &str) -> HashMap<usize ,String>{
    let mut line_numbers = HashMap::new();
    let mut current_number: String = "".to_string();
    let mut current_index: Option<usize> = None;
    
    for (index, element) in content.chars().enumerate(){
        if element.is_digit(10){
            current_number = format!("{}{}", current_number, element);
            if current_index.is_none() {
                current_index = Some(index);
            }
        }else{
            if current_number != "" {
                line_numbers.insert(current_index.unwrap(), current_number);
                current_number = "".to_string();
                current_index = None;
            }
        }
    }
    line_numbers
}

