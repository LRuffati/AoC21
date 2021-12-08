use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_a(input: BufReader<File>) {
    let res = input.lines().fold(0, |acc: usize, inp|{
        inp.unwrap().split(' ').skip_while(|el| *el!="|").fold(acc, |tot, el|{
            match el.len(){
                2 | 4 | 3 | 7 => {println!("{}",el); tot+1},
                _ => tot,
            }})
        });

    println!("{}", res);
}

struct Guess {
    bc1: char,
    bc2: char,
    fg1: char,
    fg2: char,
}

pub fn solve_b(input: BufReader<File>) {
    let tot = input.lines().map(|a | {
        let s = a.unwrap();
        println!("{}", s);
        let mut dig_4 = "";
        let mut dig_1 = "";
        let mut iters = s.split('|');
        for digit in iters.next().unwrap().split(' '){
            match digit.len() {
                2 => dig_1 = digit.clone(),
                4 => dig_4 = digit.clone(),
                _ => {},
            };
        }
        let mut bc_1 = ' ';
        let mut bc_2 = ' ';
        let mut fg_1 = ' ';
        let mut fg_2 = ' ';

        for c in dig_1.chars(){
            if bc_1 == ' '{
                bc_1 = c;
            } else {
                bc_2 = c;
            }
        }

        for c in dig_4.chars(){
            if (c==bc_1) || (c==bc_2) {continue}
            else {if fg_1 == ' ' {fg_1 = c} else {fg_2 = c}}
        }

        let res = iters.next().unwrap().split(' ').fold(0, |acc, digit|{
            if digit.len() == 0 {
                return acc;
            }
            println!("> {}", digit);
            let dig_new = match digit.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {if digit.contains(bc_1) && digit.contains(bc_2) {3}
                else if digit.contains(fg_1) && digit.contains(fg_2) {5} else { 2 }}
                6 => {if digit.contains(fg_1) && digit.contains(fg_2) {
                    if digit.contains(bc_1) && digit.contains(bc_2) {9} else {6}
                } else {0}}
                _ => panic!("{}",digit),
            };
            acc*10+dig_new
        });
        return res;
    }).fold(0, |acc, el| {acc+el});
    println!("{}", tot);
}
