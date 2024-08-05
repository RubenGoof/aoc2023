use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, vec};
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

#[derive(Debug)]
struct Throw {
    blue: u64,
    green: u64,
    red: u64
}

impl Throw {
    fn new(blue: u64, green: u64, red: u64) -> Throw{
        Throw { blue, green, red}
    }

    fn invalid(&self, bag: &Throw) -> bool {
        bag.blue < self.blue || bag.green < self.green || bag.red < self.red
    }
}

#[derive(Debug)]
struct Game {
    game_id: u64,
    throws: Vec<Throw>
}

impl Game {
    fn valid(&self, bag: &Throw) -> bool {
        for throw in &self.throws{
            if throw.invalid(&bag) {
                // println!("invalid throw: {:?}", &self.game_id);
                return false;
            }
        }
        return true;
    }

    fn get_power(&self) -> u64 {
        let mut max_throws = Throw::new(0, 0, 0);
        for throw in &self.throws{
            // println!("throw: {:?}", throw);
            if throw.red > max_throws.red {
                max_throws.red = throw.red;
            } 
            if throw.green > max_throws.green {
                max_throws.green = throw.green;
            } 
            if throw.blue > max_throws.blue {
                max_throws.blue = throw.blue;
            }
            // println!("Max throws: {:?}", max_throws);
        }
        
        max_throws.red*max_throws.blue*max_throws.green
    }
}


pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("C:/ProgrammingProjects/Rust/aoc2023/input/day02_1").expect("Did not find file!");
    let sol1: u64 = solution1(&input, Throw::new(14, 13, 12));
    let sol2: u64 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}


fn solution1(input:&str, bag:Throw) -> u64 {
    //create vector with structure (gameID, [[try][try][try]] )
    //we over-engineer it a little because who knows what we need in solution 2...
    let all_games = get_all_games(&input);
    let mut result: u64 = 0;
    for game in all_games{

        if game.valid(&bag) {
            
            result+= game.game_id;
        }
    };
    result
}

fn solution2(input:&str) -> u64 {
    //loop over all games
    let all_games = get_all_games(&input);
    let mut sum: u64 = 0;
    for game in all_games{     
        sum+= game.get_power();
    };
    sum
}

fn get_all_games(input: &str) -> Vec<Game>{
    let mut all: Vec<Game> = Vec::new();

    //now we populate it by parsing our input.
    for line in input.lines(){
        // println!("Line: {}", line);
        let game_id = get_gameid(line);
        let throws = get_throws(line);
        all.push(Game { game_id: game_id, throws: throws });
    };

    return all;
}

fn get_gameid(line: &str) -> u64{
    let re_str = Regex::new(r"Game (\d+):").unwrap();
    if let Some(captures) = re_str.captures(line) {
        let mut _partitions: Vec<String> = Vec::new();

        //first, we get the game number, which we sadly have to inefficiently convert to String and then to the appropriate type.
        if let Some(game_number) = captures.get(1) {
            return game_number.as_str().parse().unwrap();
        };
    };

    return 0;

}


fn get_throws(line: &str) -> Vec<Throw>{

    let mut result: Vec<Throw> = vec![];
    if let Some(splitline) = line.split_once(":"){
        for throw in splitline.1.split(";"){
            let mut blue: u64 = 0;
            let mut green: u64 = 0;
            let mut red: u64 = 0;

            for colour in throw.split(","){
                let values = colour.trim().split_whitespace().collect::<Vec<&str>>();
                match values[1] {
                    BLUE => blue = values[0].parse().expect("oops"),
                    GREEN => green = values[0].parse().expect("oops"),
                    RED => red = values[0].parse().expect("oops"),
                    _ => ()
                }
                // println!("values: {:?}", values)
            }
            result.push(Throw::new(blue, green, red))
        }
    };

    result
}