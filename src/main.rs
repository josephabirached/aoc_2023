use std::fs;
use std::env;

use aoc_2023::day1::day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please select a valid day to execute");
    } else {

        dbg!(&args);
        let query: &str = &args[1];


        match query {
            "1" => {
                let contents = fs::read_to_string("src/day1/resource.txt").expect("Should have been able to read file");
                dbg!(day1::answer(day1::calibrate_v2(contents)));
            }
            _ => {
                println!("Please select a valid day to execute");
            }
        }
    }
}

