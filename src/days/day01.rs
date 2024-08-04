use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let test_input_1 = read_to_string("C:/ProgrammingProjects/Rust/aoc2023/input/day01_1_test").expect("Did not find file!");
    // println!("Test: {test_input_1}");

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}