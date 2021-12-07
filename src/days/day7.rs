use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_a(input: BufReader<File>) {
    let mut vec: Vec<usize> = input.lines().next().unwrap().unwrap().split(',')
        .map(|u| u.parse::<usize>().unwrap()).collect();

    vec.sort();
    let targ = vec.get(vec.len() / 2).unwrap().clone();
    let res = vec.iter().fold(0, |acc, el|{
        if *el>targ {acc+(el-targ)}
        else {acc+(targ-el)}
    });
    println!("{}", res)
}

fn dist(el: isize, targ: isize) -> isize {
    let i = (el-targ).abs();
    (i*(i+1))/2
}

pub fn solve_b(input: BufReader<File>) {
    let mut vec: Vec<usize> = input.lines().next().unwrap().unwrap().split(',')
        .map(|u| u.parse::<usize>().unwrap()).collect();

    let sum: usize = vec.iter().sum();
    let avg = sum/vec.len();
    let check = (avg as isize, (avg+1) as isize);
    let res = vec.iter().fold((0,0), |acc, el|{
        let el = *el as isize;
        (acc.0 + dist(el, check.0), acc.1 + dist(el, check.1))
    });
    println!("{}, {}", res.0, res.1); // return both, based on approximation one will be the smallest
}
