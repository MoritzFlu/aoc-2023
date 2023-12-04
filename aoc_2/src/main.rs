use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

fn color_to_index(color: &str) -> i32 {
    match color {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => -1,
    }
}

#[derive(Debug)]
struct Game {
    data: String,
    id: i32,
    draws: Vec<Draw>,
}

impl Game {
    const GAME_REG: &str = r"Game\s([0-9]{1,}):";
    const DRAWS_REG: &str = r"[:;][a-z0-9,\s]*";
    const DRAW_REG: &str = r"([0-9]{1,})\s";

    pub fn from_str(input: String) -> Game {
        // Get regex for game id
        let re = Regex::new(Game::GAME_REG).unwrap();

        // Apply regex, panic if not match (Fatal Error in Input data!)
        let Some(caps) = re.captures(&input) else { panic!("Could not read game ID: {}",input)};

        // Get ID from string
        let id: i32 = caps[1].parse().unwrap();


        let draws_re = Regex::new(Game::DRAWS_REG).unwrap();
        let draws_matches = draws_re.find_iter(&input);

        let mut draws = Vec::new();

        // Game string has three other groups with ids 2,3,4
        for draw in draws_matches {   

            let draw_obj = Draw {
                red: Game::get_draw_num("red", draw.as_str()),
                green: Game::get_draw_num("green", draw.as_str()),
                blue: Game::get_draw_num("blue", draw.as_str()),
            };

            draws.push(draw_obj);
        }

        Game {
            data: input,
            id: id,
            draws: draws,
        }
    }

    fn get_draw_num(color: &str, input: &str) -> i32 {
        let mut reg_string = Game::DRAW_REG.to_owned();
        reg_string.push_str(color);

        let re = Regex::new(&reg_string).unwrap();

        let caps = re.captures(input);

        if caps.is_none() {
            return 0;
        } else {
            return caps.unwrap()[1].parse().unwrap();
        }
    }

    pub fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {

        for draw in self.draws.iter() {
            let mut possible = true;

            possible = possible && draw.red <= red;
            possible = possible && draw.green <= green;
            possible = possible && draw.blue <= blue;

            // possible = false if draw not possible with the given number of balls
            if !possible {
                return false
            }

        }

        return true
    }

    pub fn get_power(&self) -> i32 {

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for draw in self.draws.iter() {
            if draw.red > min_red  {
                min_red = draw.red
            }
            if draw.blue > min_blue {
                min_blue = draw.blue
            }
            if draw.green > min_green {
                min_green = draw.green
            }
        }
        println!("{}, {}, {}",min_red,min_green,min_blue);
        return min_blue * min_green * min_red
    }


}

fn main() {

    let mut result = 0;

    for line in read_to_string("src/data.txt").unwrap().lines() {
        // Instantiate line object
        let obj = Game::from_str(line.to_string());

        // TASK 1
        //let possible = obj.is_possible(12,13,14);

        //if possible {
        //    result = result + obj.id;
        //}

        //result = result + num;

        // TASK 2
        let power = obj.get_power();

        result = result + power;
    }

    println!("Result: {}", result);
}
