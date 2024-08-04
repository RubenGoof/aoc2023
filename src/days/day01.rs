use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////



pub fn solve() -> SolutionPair {
    // Your solution here...
    //https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    let test_input_1 = read_to_string("C:/ProgrammingProjects/Rust/aoc2023/input/day01_1").expect("Did not find file!");
    // println!("Test: {test_input_1}");

    let sol1: u64 = solution_1(test_input_1.clone());
    let sol2: u64 = solution_2(test_input_1);

    (Solution::from(sol1), Solution::from(sol2))
}


fn solution_1(input: String) -> u64 {
    //loop through all lines and combine first and last digit to create a number
    //store these in an array size of amount of lines
    //sum of all these values
    let mut sum: u64 = 0;
    for line in input.lines() {
        let digits = get_first_last_digit(&line);
        // println!("digits: {:?}", digits);
        sum += digits.0 * 10 +digits.1;
        // println!("Sum: {sum}")
    };
    sum

}

fn get_first_last_digit(line: &str) -> (u64, u64) {
    const RADIX: u32 = 10;
    let mut x: char = '0';
    let mut latestc: char = '0';
    let mut counter: u64 = 0;
    for c in line.chars().filter(|n| n.is_numeric()) {
        if counter == 0 {
            x = c;
        };
        latestc = c;
        counter +=1
    };
    // println!("Value: {} and {}", x, latestc);
    return (x.to_digit(RADIX).expect("No digit!") as u64,
     latestc.to_digit(RADIX).expect("No digit!") as u64)
}

fn solution_2(input: String) -> u64 {

    let mut sum: u64 = 0;
    for line in input.lines() {
        let replaced_line = replace_written_digits(line);
        let digits = get_first_last_digit(&replaced_line);
        // println!("digits: {:?}", digits);
        sum += digits.0 * 10 +digits.1;
        // println!("Sum: {sum}")
    };
    sum
}

fn replace_written_digits(input: &str) -> String {
    let mut res: String = input.to_string();
    res = res.replace("one", "o1e");
    res = res.replace("two", "t2o");
    res = res.replace("three", "t3ree");
    res = res.replace("four", "f4r");
    res = res.replace("five", "5e");
    res = res.replace("six", "6");
    res = res.replace("seven", "7en");
    res = res.replace("eight", "ei8ght");
    res = res.replace("nine", "9ne");
    return res;
}