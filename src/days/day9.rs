use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::id;

struct Candidate {
    val: u32,
    idx: usize,
}

#[derive(Debug)]
struct Element {
    val: u32,
    local_min: bool
}

pub fn solve_a(input: BufReader<File>) {
    let mut prev_line: Vec<Element> = Vec::new();

    let tot_to_pre_last = input.lines().fold(0, |tot, el|{
        let (tot_line, last_el, min_last) = el.unwrap().chars().map(|c| c.to_digit(10).unwrap()).enumerate().fold(
            (0, None, false),
            |(mut tot_line, prev, mut prev_min), (idx, el)| {
                let above = prev_line.get(idx);
                let mut this_min = true;
                if let Some(x) = above { // Check the above line if it exists
                    println!("Comparing {} with {:?} at position {}", el, x, idx);
                    if x.val < el {
                        if x.local_min { // If the above is less than this and was a local min
                            println!("{} min, column {}", x.val, idx);
                            tot_line += x.val + 1
                        }
                        this_min &= false; // If the above is less then this element can't be a local min
                    } else if x.val == el { // If the above is equal neither can be minimums
                        this_min &= false;
                    }
                } else { // push a fake element which will be overridden later
                    println!("Pushing fake");
                    prev_line.push(Element {val: el, local_min: this_min});
                }

                if let Some(x) = prev { // If the prev exists
                    println!("Comparing {} with {} (left)", el, x);
                    if x<el { // And is less than this one
                        this_min &= false;
                    } else if x==el { // if they're equal they're both false
                        prev_min &= false;
                        this_min &= false;
                    } else {
                        prev_min &= false;
                    }
                    prev_line[idx-1] = Element { val: x, local_min: prev_min }; // can't be zero because on zero prev is None
                }
                (tot_line, Some(el), this_min)
                });
        prev_line.pop(); //remove the last element
        prev_line.push(Element {val: last_el.unwrap(), local_min: min_last});
        tot + tot_line
        });
    let res = prev_line.iter().fold(tot_to_pre_last, |acc, el|{
        if el.local_min{
            acc + el.val + 1
        } else {
            acc
        }
    });
    println!("{}", res);
}

struct ElemBasin {
    elem: u32,
    basin: Option<usize>,
}

fn get_idx(vector: &Vec<usize>, mut start: usize) -> Option<usize>{
    let mut el = vector.get(start);
    loop {
        if let Some(x) = el {
            if *x==start {
                break Some(*x);
            } else {
                start = *x;
                el = vector.get(start)
            }
        } else {
            break None;
        }
    }
}

pub fn solve_b(input: BufReader<File>) {
    /*
    I don't care about the sinks, just about the basins, I can scan them linearly and keep
    two arrays, one with the sizes of the basin and one with the basin it drains into

    */
    let mut vec_sizes: Vec<usize> = Vec::new(); // vec_sizes[idx] = size of basin idx
    let mut drainage: Vec<usize> = Vec::new(); // drainage[x] = y means x drains into y, y<x

    let mut prev_line: Vec<Option<usize>> = Vec::new(); // None => doesn't drain (9), Some(x) drains in basin x

    input.lines().fold(
        (),
        |_, el| {
            el.unwrap().chars().map(|c| c.to_digit(10).unwrap()).enumerate().fold(
                None,
                |bas_left, (idx, el)| {
                    /*
                    If el is 9 then return none
                    If el is not 9 assign it the lowest basin number between the one above and
                    the one to the left. If both are none create a new basin

                    If above and left are different then map
                     */
                    let mut basin_tmp = drainage.len();
                    let mut new = true;
                    let mut remap: Option<usize> = None;

                    if let Some(x) = prev_line.get(idx) { // Check above
                        if let Some(y) = *x {
                            basin_tmp = get_idx(&drainage, y).expect("Error in finding a basin");
                            new &= false;
                        }
                    } else {
                        prev_line.push(None);
                    }

                    if let Some(x) = bas_left { // check left
                        let x = get_idx(&drainage, x).expect("Error in left basin");
                        if x < basin_tmp {
                            if new {
                                new &= false;
                            } else { // it's not the new one so tmp is from above
                                remap = Some(basin_tmp);
                            }
                            basin_tmp = x;
                        } else if x > basin_tmp {
                            remap = Some(x);
                        }
                    }

                    if el == 9 {
                        prev_line[idx] = None;
                        None
                    } else {
                        if let Some(x) = remap {
                            drainage[x] = basin_tmp
                        }
                        if new {
                            drainage.push(basin_tmp);
                            vec_sizes.push(0)
                        }
                        vec_sizes[basin_tmp] += 1;
                        prev_line[idx] = Some(basin_tmp);
                        Some(basin_tmp)
                    }
                });
        });

    let mut final_sizes: Vec<usize> = Vec::new();
    for el in (0..drainage.len()).rev() {
        let targ = get_idx(&drainage, el).unwrap();
        if targ == el { // fixed point
            let size = vec_sizes.get(targ).unwrap();
            final_sizes.push(*size);
            println!("Final size {} for drainage {}", *size, targ)
        } else {
            vec_sizes[targ] += vec_sizes[el];
            println!("{} ==> {}", el, targ)
        }
    }
    final_sizes.sort();
    final_sizes.reverse();
    let res = final_sizes.iter().take(3).fold(1, |a, e|{a*(*e)});
    println!("{}", res)
}
