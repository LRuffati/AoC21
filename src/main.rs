mod day2;

use std::fs;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("inputs/02a").expect("Can't find file");
    let reader = BufReader::new(file);
    //let input: String = fs::read_to_string("inputs/01a").unwrap();
    day2::solve_b(reader);
}
