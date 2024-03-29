//use crate::advent_of_code_2022::*;
//use crate::leetcode::*;
use crate::project_euler::digit_factorials;

#[path = "advent_of_code_2022/advent_of_code_2022.rs"]
pub mod advent_of_code_2022;
pub mod leetcode;
#[path = "project_euler/project_euler_problem_34.rs"]
pub mod project_euler;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    digit_factorials();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
