use daytwo::solution_one;
use daytwo::solution_two;

// const to declare global variables
const SAMPLE: &str = "sample.txt";
const ACTUAL: &str = "input.txt";

fn main() {
    let sample_sol = solution_two(SAMPLE);
    println!("Sample = {}", sample_sol);
    assert_eq!(sample_sol, 465);
}
