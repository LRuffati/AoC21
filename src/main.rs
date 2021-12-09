mod days;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use days::*;


fn main() {
    let file = File::open("inputs/09").expect("Can't find file");
    let reader = BufReader::new(file);
    //let input: String = fs::read_to_string("inputs/01a").unwrap();
    solve_b(reader);
}
