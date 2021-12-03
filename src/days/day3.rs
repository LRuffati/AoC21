use std::fs::File;
use std::io::{BufRead, BufReader};

struct BitTracker<const WIDTH: usize> {
    numb: usize,
    tracker: [usize; WIDTH],
}

impl<const W: usize> BitTracker<W> {
    const fn new() -> Self {
        BitTracker {
            numb: 0,
            tracker: [0; W],
        }
    }
    fn add_bits(self, bits: String) -> Self {
        let mut bits_new: [usize; W] = self.tracker;

        for (i, c) in bits.chars().enumerate() {
            if c=='1' {
                bits_new[i]+=1;
            }
        }
        let ret =  BitTracker{numb: self.numb+1, tracker: bits_new};
        return ret;
    }

    fn solve(&self) -> usize {
        let thresh = self.numb / 2;
        let mut ones = 0;
        let mut zeroes = 0;
        for i in self.tracker.into_iter() {
            ones *= 2;
            zeroes *= 2;

            if i>thresh {
                ones += 1;
            } else {zeroes += 1}

        };
        return ones*zeroes;
    }
}

pub fn solve_a(input: BufReader<File>) {
    let ini: BitTracker<12> = BitTracker::new();
    let out = input.lines()
        .map(|l| l.unwrap())
        .fold(ini, |l, r| l.add_bits(r)).solve();
    println!("{}", out);

}

struct Sorter {
    zeroes: Vec<(usize, usize)>,
    ones: Vec<(usize, usize)>,
}

fn reverse_usize(input: String) -> (usize, usize) {
    let straight= usize::from_str_radix(input.as_str(), 2).unwrap();
    let mut r = 0;
    for i in input.chars().rev(){
        r *= 2;
        if i=='1'{
            r+=1
        }
    }
    (r, straight)
}

impl Sorter {
    fn add(&mut self, input: (usize, usize)) {
        let (input, straight) = input;
        let (n, r) = (input/2, input%2);
        if r==0 {
            self.zeroes.push((n, straight));
        } else {
            self.ones.push((n,straight));
        }
    }

    fn split(self) -> (Vec<(usize, usize)>, Vec<(usize,usize)>) {
        if self.ones.len() >= self.zeroes.len() {
            (self.ones, self.zeroes)
        } else {
            (self.zeroes, self.ones)
        }
    }

    fn new() -> Sorter {
        Sorter{zeroes: Vec::new(), ones: Vec::new()}
    }
}

pub fn solve_b(input: BufReader<File>) {
    let uint_inp = input.lines().map(|l| reverse_usize(l.unwrap()));
    let mut init = Sorter::new();
    for i in uint_inp{
        init.add(i);
    }
    let mut high = 0;
    let mut low = 0;

    let (mut high_vec, mut low_vec) = init.split();
    // Here a step has been done, 11 left

    while high_vec.len() > 1 {
        let mut sorter = Sorter::new();
        for i in high_vec{
            sorter.add(i);
        }
        let (mut h, _) = sorter.split();
        high_vec = h;
    }
    high = high_vec.pop().unwrap().1;


    while low_vec.len() > 1 {
        let mut sorter = Sorter::new();
        for i in low_vec{
            sorter.add(i);
        }
        let (_, mut l) = sorter.split();
        low_vec = l;
    }
    low = low_vec.pop().unwrap().1;

    println!("{} {} {}", high, low, low*high);
}