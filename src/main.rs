mod day1;
use std::fs;


fn main() {
    let mut input: String = fs::read_to_string("inputs/01a").unwrap();
    day1::solve_b(input);
}
