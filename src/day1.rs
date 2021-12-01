use std::fs::File;
use std::io::{BufRead, BufReader};

struct Acc {
    prev: Option<usize>,
    count: usize,
}

pub fn solve_a(input: BufReader<File>) {
    let res: Acc = input
        .lines()
        .map(|num| num.unwrap().parse::<usize>().unwrap())
        .fold(
            Acc {
                prev: None,
                count: 0,
            },
            |acc: Acc, n| {
                if acc.prev.is_none() {
                    Acc {
                        prev: Some(n.clone()),
                        count: acc.count,
                    }
                } else {
                    Acc {
                        prev: Some(n.clone()),
                        count: if n > acc.prev.unwrap() {
                            acc.count + 1
                        } else {
                            acc.count
                        },
                    }
                }
            },
        );
    println!("{}", res.count);
}

pub fn solve_b(input: BufReader<File>) {
    let nums: Vec<usize> = input
        .lines()
        .map(|num| num.unwrap().parse().unwrap())
        .collect();
    let first = &nums[0..];
    let sec = &nums[1..];
    let third = &nums[2..];
    let res: Acc = first.iter().zip(sec.iter()).zip(third.iter()).fold(
        Acc {
            prev: None,
            count: 0,
        },
        |acc: Acc, ((a, b), c)| {
            let sum = a + b + c;
            if acc.prev.is_none() {
                Acc {
                    prev: Some(sum),
                    count: acc.count,
                }
            } else {
                Acc {
                    prev: Some(sum),
                    count: if sum > acc.prev.unwrap() {
                        acc.count + 1
                    } else {
                        acc.count
                    },
                }
            }
        },
    );

    println!("{}", res.count);
}
