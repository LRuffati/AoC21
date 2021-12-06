use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Swarm {
    alive: [usize; 9],
    days: usize,
}

pub fn solve_a(input: BufReader<File>) {
    let init = input.lines().next().unwrap().unwrap().split(',').fold(
        [0usize;9],
        |mut arr, el| {
            arr[el.parse::<usize>().unwrap()] += 1;
            arr
        }
    );
    let mut swarm = Swarm {alive: init, days:0};
    while swarm.days<256 {
        let mut new = [0usize; 9];
        new[8] = swarm.alive[0];
        new[7] = swarm.alive[8];
        new[6] = swarm.alive[7] + swarm.alive[0];
        new[5] = swarm.alive[6];
        new[4] = swarm.alive[5];
        new[3] = swarm.alive[4];
        new[2] = swarm.alive[3];
        new[1] = swarm.alive[2];
        new[0] = swarm.alive[1];

        swarm.alive = new;
        swarm.days += 1;
    }
    println!("{}", swarm.alive.iter().sum::<usize>());
}
