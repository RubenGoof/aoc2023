use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, vec};

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    given_numbers: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Card {
        println!("input: {:?}", input);
        let card_str_split = match input.split_once(":"){
            Some(x) => x.1,
            None => "none",
        };
    
        println!("card_str_split: {:?}", card_str_split);
        let numbers_split = match card_str_split.split_once("|") {
            Some(x) => x,
            None => ("none", "none"),
        };
    
        println!("numbers_split: {:?}", numbers_split);
        let mut winning_numbers: Vec<u32> = Vec::new();
        for x in numbers_split.0.split(" "){
            match x.parse::<u32>() {
                Ok(y) => winning_numbers.push(y),
                Err(_) => println!("Nan!"),
            };    
        }
    
        println!("winning_numbers: {:?}", numbers_split);

        let mut given_numbers: Vec<u32> = Vec::new();
        for x in numbers_split.1.split(" "){
            match x.parse::<u32>() {
                Ok(y) => given_numbers.push(y),
                Err(_) => println!("Nan!"),
            }; 
        };

        Card { winning_numbers: winning_numbers, given_numbers: given_numbers }
        
        // Card { winning_numbers: winning_numbers, given_numbers: given_numbers}
    }

    fn get_matching_numbers (&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        let winnings = &self.winning_numbers;
        let givens = &self.given_numbers;
        for winner in winnings.into_iter() {
            for given in givens.into_iter() {
                if winner == given {
                    result.push(winner.clone());
                }
            }
        }
        result
    }

    fn get_value (&self) -> u32 {
        let mut power = self.get_matching_numbers().len();
        if power == 0{
            0
        } else {
            let mut value: u32 = 1;
            while power > 1 {
                value = value * 2;
                power -= 1;
            }
            value
        }

        
    }
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("C:/ProgrammingProjects/Rust/aoc2023/input/day04").expect("Did not find file!");
    let sol1: u32 = solution1(&input);
    let sol2: u32 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution1(input: &str) -> u32{
    //first we split by : to disregard the first part.
    // then we split by | to separate winning and given numbers
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let card = Card::new(line);
        println!("We created a Card! {:?}", card);
        cards.push(card);
    }


    let mut score: u32 = 0;
    for card in cards {
        let value = card.get_value();
        println!("Here we are adding {value}!");
        score+=value
    }
    score
    
    //then we split by whitespace to get the numbers\

    //then we transform Vec<&str> into Vec<u32>

    //we create a card

    //and call our functions
}

fn solution2(input: &str) -> u32{
    0
}

