use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, vec};

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("C:/ProgrammingProjects/Rust/aoc2023/input/day03").expect("Did not find file!");
    let sol1: u32 = solution1(&input);
    let sol2: u32 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution1(input: &str) -> u32{
    //Get our 2D vector
    let engine = transform_str_charvector(input);
    // println!("Char vector: {:?}", charactervec);
    collect_numbers(&engine)
}


fn solution2(input: &str) -> u32{

    0
}


/*
args: all lines of the puzzle
returns: a dot padded 2D vector of characters
Method achieves this by first creating a 2D vector and then transforming this into an array.
*/
fn transform_str_charvector(input: &str) -> Vec<Vec<char>> {

    let lines = input.lines();
    let mut charactervec: Vec<Vec<char>> = Vec::new();
    let mut y: usize = 0;
    let mut first: bool = true;
    for line in lines {
        if first {
            y = line.len();
            charactervec.push(vec!['.'; y]); //pad first line
            first = false;
        };

        let mut characters: Vec<char> = Vec::new();
        characters.push('.');
        characters.extend(line.chars());
        characters.push('.');
        charactervec.push(characters);
    };

    charactervec.push(vec!['.'; y]); //pad last line
    charactervec
}

/*

*/
fn collect_numbers(engine: &Vec<Vec<char>>) -> u32 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut total:u32 = 0;
    for characters in engine.into_iter() {
        let mut num: u32 = 0;
        let mut in_digit: bool = false;
        let mut valid: bool = false;
        for character in characters.into_iter(){
            if !character.is_digit(10) && in_digit {
                //we are no longer in the digit so we will have to reset, and we can add the number to the total if it is a valid one.
                if valid {
                    // println!("Valid number! {num}");
                    total += num;
                }
                num = 0;
                in_digit = false;
                valid = false;
            }
            if character.is_digit(10){ //we are in a digit!
                // println!("Found a digit! {character}");
                num = num*10 + character.to_digit(10).expect("Did not find a digit but was adding a digit");
                // println!("So we now have: {num}");
                in_digit = true;
                if !valid {
                    valid = check_valid(x, y, engine);
                }
            }
            y+=1;
        };
        y=0;
        x+=1;
    };
    total
}


fn check_valid (x: usize, y: usize, engine: &Vec<Vec<char>>) -> bool {

    let range_x = [x-1, x, x+1];
    let range_y = [y-1, y, y+1];
    for x in range_x {
        for y in range_y {
            if !engine[x][y].is_digit(10) && engine[x][y] != '.' {
                return true;
            }
        };
    };
    false
}