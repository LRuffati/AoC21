use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use regex::Regex;
use crate::day2::Command::{Down, Forward};


struct Position {
    depth: usize,
    forward: usize,
    aim: isize,
}

enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl Command {
    fn parse(input: String, re: &Regex) -> Self {
        let matches = re.captures(input.as_str()).unwrap();
        let number = matches.get(4).map(|num| {
            let txt = num.as_str();
            txt.parse::<usize>().unwrap()
        }).unwrap();
        if matches.get(1).is_some() {
            Forward(number)
        } else if matches.get(2).is_some() {
            Down(number)
        }else {
            Command::Up(number)
        }
    }
}

impl Add<Command> for Position {
    type Output = Self;

    fn add(self, rhs: Command) -> Self {
        match rhs {
            Forward(x) => Position {forward: self.forward+x, ..self},
            Command::Up(x) => Position {depth: self.depth-x, ..self},
            Down(x) => Position {depth: self.depth+x, ..self},
        }
    }
}

impl std::ops::Mul<Command> for Position {
    type Output = Self;

    fn mul(self, rhs: Command) -> Self {
        match rhs {
            Forward(x) => {
                let f = self.forward + x;
                let d = (self.depth as isize) + self.aim * (x as isize);
                Position{forward: f, depth: d as usize, ..self}
            },
            Command::Up(x) => Position{aim: self.aim-(x as isize), ..self},
            Down(x) => Position{aim: self.aim+(x as isize), ..self},
        }
    }
}

pub fn solve_a(input: BufReader<File>) {
    let re: Regex = Regex::new(r"(?:(forward)|(down)|(up)) ([0-9]+)").unwrap();
    let final_p: Position = input.lines()
        .map(|line| Command::parse(line.unwrap(), &re))
        .fold(Position{depth: 0, forward: 0, aim: 0}, |p, c| p + c);
    println!("{}", final_p.forward*final_p.depth)
}

pub fn solve_b(input: BufReader<File>) {
    let re: Regex = Regex::new(r"(?:(forward)|(down)|(up)) ([0-9]+)").unwrap();
    let final_p: Position = input.lines()
        .map(|line| Command::parse(line.unwrap(), &re))
        .fold(Position{depth: 0, forward: 0, aim: 0}, |p, c| p * c);
    println!("{}", final_p.forward*final_p.depth)
}
