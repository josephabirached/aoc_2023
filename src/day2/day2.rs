// 12 Red
// 13 Green
// 14 Blue
pub fn playable_games(input: &str) -> Vec<u32> {
    let mut games_index: Vec<u32> = Vec::new();
    let mut i = 0;
    for line in input.lines() {
        i += 1;
        let mut can_play = true;
        let game_lines = line.split(":").last().unwrap(); 

       for game_line in game_lines.split(";"){
            for dice_value in game_line.split(','){
                if dice_value.contains("red"){
                    let sum_red = dice_value.replace("red", "").replace(" ", "").parse::<u32>().unwrap();
                    if sum_red > 12{
                        can_play = false; 
                        break;
                    }
                }
                if dice_value.contains("green"){
                    let sum_green = dice_value.replace("green", "").replace(" ", "").parse::<u32>().unwrap(); 
                    if sum_green > 13{
                        can_play = false;
                        break;
                    }
                }
                if dice_value.contains("blue"){
                    let sum_blue = dice_value.replace(" blue", "").replace(" ", "").parse::<u32>().unwrap(); 
                    if sum_blue > 14{
                        can_play = false;
                        break;
                    }
                }
            }
       }

       if can_play == true{
          games_index.push(i);
       }
    }
    games_index
}

pub fn minimum_dice_required(input: &str) -> Vec<u32>{
    let mut minimum_dice: Vec<u32> = Vec::new();
    for line in input.lines() {
        let game_lines = line.split(":").last().unwrap(); 
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

       for game_line in game_lines.split(";"){
            for dice_value in game_line.split(','){
                if dice_value.contains("red"){
                    let red = dice_value.replace("red", "").replace(" ", "").parse::<u32>().unwrap();
                    if red > min_red {
                        min_red = red;                         
                    }
                }
                if dice_value.contains("green"){
                    let green = dice_value.replace("green", "").replace(" ", "").parse::<u32>().unwrap(); 
                    if green > min_green {
                        min_green = green;
                    }               
                }
                if dice_value.contains("blue"){
                    let blue = dice_value.replace(" blue", "").replace(" ", "").parse::<u32>().unwrap(); 
                    if blue > min_blue {
                       min_blue = blue; 
                    }
                }
            }
       }
        minimum_dice.push(min_red * min_green * min_blue);
    }

    minimum_dice
}

pub fn answer(ids: Vec<u32>) -> u32 {
    let mut sum_ids: u32 = 0;
    for id in ids {
        sum_ids += id;
    }
    sum_ids
}

