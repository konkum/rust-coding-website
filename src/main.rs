//use crate::advent_of_code_2022::*;
use crate::leetcode::*;

#[path = "advent_of_code_2022/advent_of_code_2022.rs"]
pub mod advent_of_code_2022;
pub mod leetcode;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    println!("{}", roman_to_int("IV".to_string()));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
