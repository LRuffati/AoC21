use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_a(input: BufReader<File>) {
    let mut queue: Vec<char> = Vec::new();
    let res: i32 = input.lines().fold(0, |sum, line|{
        let line_res = line.unwrap().chars().fold(None, |acc: Option<i32>, chr|{
            if acc.is_some() {
                return acc;
            }
            match chr {
                '{'|'('|'['|'<' => {queue.push(match chr {
                    '{' => '}',
                    '(' => ')',
                    '[' => ']',
                    '<' => '>',
                    _ => {unreachable!();}
                }); acc}
                _ => {
                    let right = queue.pop().unwrap();
                    if chr == right {
                        acc
                    } else {
                        match chr {
                            ')' => Some(3),
                            ']' => Some(57),
                            '}' => Some(1197),
                            '>' => Some(25137),
                            _ => unreachable!("Whelp"),
                        }
                    }
                }
            }
        });
        queue = Vec::new();
        if let Some(x) = line_res {
            sum + x
        } else {
            sum
        }
    });
    println!("{}", res);

}

pub fn solve_b(input: BufReader<File>) {
    let mut queue: Vec<char> = Vec::new();
    let mut scores: Vec<usize> = Vec::new();
    input.lines().fold((), |_, line|{
        let line_res = line.unwrap().chars().fold(None, |acc, chr|{
            if acc.is_some() {
                return acc;
            }
            match chr {
                '{'|'('|'['|'<' => {queue.push(match chr {
                    '{' => '}',
                    '(' => ')',
                    '[' => ']',
                    '<' => '>',
                    _ => {unreachable!();}
                }); acc}
                _ => {
                    let right = queue.pop().unwrap();
                    if chr == right {
                        acc
                    } else {
                        Some(right)
                    }
                }
            }
        });
        if line_res.is_none() {
            let mut score = 0;
            let mut last = queue.pop();
            while let Some(x) = last {
                score *= 5;
                score += match x {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                };
                last = queue.pop();
            }
            scores.push(score);
        }
        queue = Vec::new();
    });
    scores.sort();
    let res = scores.get(scores.len()/2).unwrap();
    println!("{}", *res);

}