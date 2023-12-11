use std::fs;
use std::env;

use aoc_2023::day1::day1;
use aoc_2023::day2::day2;

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
            "2" => {
                // Part 1
                let content_test = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");            

                dbg!(day2::answer(day2::playable_games(&content_test)));

                let contents = fs::read_to_string("src/day2/resource.txt").expect("Should have been able to read file");
                dbg!(day2::answer(day2::playable_games(&contents)));

                // Part 2
                dbg!(day2::answer(day2::minimum_dice_required(&content_test)));
                dbg!(day2::answer(day2::minimum_dice_required(&contents)));
            }
            _ => {
                println!("Please select a valid day to execute");
            }
        }
    }
}

