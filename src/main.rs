use std::fs;

use aoc_2023::days::day1;

fn main() {
    let contents = fs::read_to_string("resource.txt").expect("Should have been able to read file");
    
    dbg!(answer(day1::calibrate_v2(contents)));
}

fn answer(calibrations: Vec<String>) -> u32 {
    let mut sum_calibration: u32 = 0;
    for calib in calibrations {
        sum_calibration += calib.parse::<u32>().unwrap();
    }
    sum_calibration
}
