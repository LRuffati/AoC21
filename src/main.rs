mod days;
mod support;


use std::fs;
use std::fs::File;
use std::io::BufReader;
use days::*;


fn main() {
    let file = File::open("inputs/06").expect("Can't find file");
    let reader = BufReader::new(file);
    //let input: String = fs::read_to_string("inputs/01a").unwrap();
    solve_a(reader);
}
