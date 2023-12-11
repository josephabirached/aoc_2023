pub fn calibrate(input: String) -> Vec<String> {
    let mut calibration_values = vec![];
    
    for line in input.lines() {
        let line_numbers: Vec<&str> = line.matches(char::is_numeric).collect();
        let first_number = line_numbers.first().unwrap();
        let last_number = line_numbers.last().unwrap();

        calibration_values.push(format!("{}{}", first_number, last_number));
    }

    calibration_values

}

struct LineNumbers{
    index: usize,
    value: String
}

pub fn calibrate_v2(input: String) -> Vec<String> {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut calibration_values = vec![];

    
    for line in input.lines() {
        let mut line_numbers: Vec<LineNumbers> = Vec::new();

        for n in 0..9{
            let index_number = line.find(&(n+1).to_string());
            let last_index_number = line.rfind(&(n+1).to_string());

            let index_char = line.find(numbers[n]);
            let last_index_char = line.rfind(numbers[n]);
                
            if index_number.is_some(){
                line_numbers.push(LineNumbers { index: index_number.unwrap(), value: (n+1).to_string() });
                if last_index_number.is_some() && last_index_number != index_number{
                    line_numbers.push(LineNumbers { index: last_index_number.unwrap(), value: (n+1).to_string() });
                }
            }
            if index_char.is_some(){
                line_numbers.push(LineNumbers { index: index_char.unwrap(), value: (n+1).to_string() });
                if last_index_char.is_some() && last_index_char != index_char{
                    line_numbers.push(LineNumbers { index: last_index_char.unwrap(), value: (n+1).to_string() });
                }
            }
        }
        line_numbers.sort_by_key(|k| k.index);

        let first_number = line_numbers.first().unwrap();
        let last_number = line_numbers.last().unwrap();

        calibration_values.push(format!("{}{}", first_number.value, last_number.value));
    }

    calibration_values
}

pub fn answer(calibrations: Vec<String>) -> u32 {
    let mut sum_calibration: u32 = 0;
    for calib in calibrations {
        sum_calibration += calib.parse::<u32>().unwrap();
    }
    sum_calibration
}

