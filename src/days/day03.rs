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

//Check if the number has a symbol next to it
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


/*
Let's try something totally different for solution 2.
Starting from scatch.
Sorry for this monstrosity, I am sure it can be shortened with recusion/function mapping by like a lot.
*/

fn solution2(input: &str) -> u32{
    let engine: Vec<Vec<char>> = transform_str_charvector(input);

    let gears = collect_gears(&engine);
    // println!("Gears: {:?}", gears);
    // println!("Engine: {:?}", engine);
    let mut sum_ratios: u32 = 0;
    for gear in gears {
        sum_ratios += get_ratios(&gear, &engine);
    }
    
  
    sum_ratios
}

fn collect_gears(engine: &Vec<Vec<char>>) -> Vec<(usize, usize)>  {
    let mut positions_all: Vec<(usize, usize)> = Vec::new();
    for row in engine.into_iter().enumerate(){
        for cell in row.1.into_iter().enumerate() {
            if cell.1 == &'*'{
                positions_all.push((row.0, cell.0));
            }
        }
    }
    positions_all
}

fn get_ratios(gear: &(usize, usize) , engine: &Vec<Vec<char>>) -> u32{


    //sorry for this, i dont know how to do it differenlty yet lol
    let mut validated = vec![vec![false; engine[0].len()+1];engine.len()+1];
    // let mut slice = vec![vec; 3];
    // println!("Slice: {:?} for rngx {:?} and rngy {:?} ", slice, rngx, rngy);
    let mut ratio: Vec<u32> = Vec::new();

    let xs = gear.0-1..=gear.0+1;
    for x in xs {
        let ys = gear.1-1..=gear.1+1;
        for y in ys{
            let cell = engine[x][y];
            // println!("We are at x:{x} and y: {y} with value {cell}");
            let mut num: u32;
            num = match cell.to_digit(10){
                Some(x) => x,
                None => 10,
            };
            
            //Now, we expand the number!
            if num != 10 && !validated[x][y] {
                validated[x][y] = true;
                //first we check left...
                let dig = get_digit((x,y-1), engine);
                // println!("This means we have digit {dig}");
                if dig != 10 && !validated[x][y-1] {
                    
                    validated[x][y-1] = true;
                    num = dig*10 + num;
                    // println!("We found one +1! We're at {num}.");
                    let dig = get_digit((x,y-2), engine);
                    if dig != 10 && !validated[x][y-2] {
                        
                        validated[x][y-2] = true;
                        num = dig*100 + num;
                        // println!("We found one +2! We're at {num}.");
                    }
                }
                //now we check right
                let dig = get_digit((x,y+1), engine);
                if dig != 10 && !validated[x][y+1] {
                    
                    validated[x][y+1] = true;
                    num = dig + 10*num;
                    // println!("We found one +1! We're at {num}.");
                    let dig = get_digit((x,y+2), engine);
                    if dig != 10 && !validated[x][y+2] {
                        
                        validated[x][y+2] = true;
                        num = dig + 10*num;
                        // println!("We found one +2! We're at {num}.");
                    }
                }
                ratio.push(num);
            } else {
                // println!("No number found.")
            }
            
        }
    }

    println!("Return! {}", factor(&ratio));
    factor(&ratio)
}

fn factor(nums: &Vec<u32>) -> u32 {
    let mut result: u32 = 1;
    
    if nums.len() == 1 || nums.len() == 0 { //also at len 1 because then it is not a valid gear.
        return 0;
    }

    for num in nums{
        result = result * num;
    }
    println!("Multiplying {:?} resulted in {result}", nums);
    result
}

fn get_digit(position: (usize, usize), engine: &Vec<Vec<char>>) -> u32 {
    match engine[position.0][position.1].to_digit(10) {
        Some(x) => x,
        None => 10, //which is not possible if it were just 1 digit...
    }
}
