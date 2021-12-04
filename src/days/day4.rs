use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::rc::Rc;

struct Board {
    sum: usize,
    won: bool,
}

struct Line {
    sum: usize,
    row: bool,
    board: Rc<RefCell<Board>>,
}

struct Accumulator {
    board: Rc<RefCell<Board>>,
    columns: Vec<Rc<RefCell<Line>>>,
}

impl Accumulator {
    fn new() -> Self {
        let mut board = Rc::new(RefCell::new(Board { sum: 0, won: false }));
        let mut vec = Vec::with_capacity(5);
        for _ in 0..5 {
            vec.push(Rc::new(RefCell::new(Line {
                sum: 0,
                row: false,
                board: board.clone(),
            })));
        }
        Accumulator {
            columns: vec,
            board: board,
        }
    }
}

fn bind_line(line: Rc<RefCell<Line>>, int: usize, map: &mut BTreeMap<usize, Vec<Rc<RefCell<Line>>>>) {
    if map.contains_key(&int) {
        let vec = map.get_mut(&int).unwrap();
        vec.push(line.clone())
    } else {
        let mut vec = Vec::new();
        vec.push(line.clone());
        map.insert(int, vec);
    }

}

fn parse_boards(input: Lines<BufReader<File>>, ) -> BTreeMap<usize, Vec<Rc<RefCell<Line>>>>{
    /// Parses a stream of lines into bingo boards and records the lines in the btreemap

    let mut numbers: BTreeMap<usize, Vec<Rc<RefCell<Line>>>> = BTreeMap::new();
    let mut acc = Accumulator::new();

    let acc = input.fold(acc, |mut board, line| {
        /// This fold reads line by line. If it encounters an empty line it converts the columns it
        /// collected into lines, registers them in the treemap and binds the board to them
        /// Then it creates a new board
        ///
        /// If it's a normal line it reads the numbers and creates a row. When creating a row I
        /// increment the board total

        let line = line.unwrap();
        if line.is_empty() {
            Accumulator::new()
        } else {
            let mut row: Rc<RefCell<Line>> = Rc::new(RefCell::new(Line{
                sum: 0,
                row: true,
                board: board.board.clone(),
            }));
            let row_sum = line.split(' ') // Split at spaces
                .flat_map(|e| e.parse::<usize>()) // parse
                .enumerate() // (idx, number)
                .fold(0usize, |sum_row, (idx, e)| {
                    bind_line(row.clone(), e, &mut numbers);
                    let col = board.columns.get(idx).unwrap().clone();
                    bind_line(col.clone(), e, &mut numbers);
                    col.as_ref().borrow_mut().sum += e;
                    sum_row+e
                });
            row.as_ref().borrow_mut().sum = row_sum;
            board.board.as_ref().borrow_mut().sum += row_sum;
            board
        }
    });

    numbers
}

pub fn solve_a(input: BufReader<File>) {
    let mut iter = input.lines();
    let first_line = iter.next().unwrap().unwrap();
    let _ = iter.next();
    let mut numbers = parse_boards(iter);

    // println!("{}", numbers.len());
    let res: Option<usize> = first_line.split(',')
        .flat_map(|e| e.parse::<usize>())
        .fold(None, |opt: Option<usize>, e|{
            if opt.is_some(){
                return opt;
            }
            let vec = numbers.get_mut(&e);
            if vec.is_none() {
                None
            } else {
                let vec = vec.unwrap();
                let board: Option<Rc<RefCell<Board>>> = vec.iter().fold(
                    None,
                    |board, line| {
                        line.as_ref().borrow_mut().sum-=e;
                        if line.as_ref().borrow().row {
                            line.as_ref().borrow().board.as_ref().borrow_mut().sum-=e;
                        }
                        if line.as_ref().borrow().sum == 0{
                            Some(line.as_ref().borrow().board.clone())
                        }else { board }
                    }
                );
                if board.is_some(){
                    Some(board.unwrap().as_ref().borrow().sum * e)
                } else {
                    None
                }
            }
            });
    if res.is_some(){
        println!("{}", res.unwrap());
    } else {
        println!("Uh oh")
    }

}

pub fn solve_b(input: BufReader<File>) {
    let mut iter = input.lines();
    let first_line = iter.next().unwrap().unwrap();
    let _ = iter.next();
    let mut numbers = parse_boards(iter);

    // println!("{}", numbers.len());
    let res: Option<usize> = first_line.split(',')
        .flat_map(|e| e.parse::<usize>())
        .fold(None, |opt: Option<usize>, e|{ // Iterate over numbers drawn
            /// opt is the previous result, if it's some then there has been some previous result
            /// and we're looking for later wins
            ///
            /// We look for the vector stored in the map, if the vector is none then we keep the
            /// previous win and go to the later number
            ///
            /// If the vector has some elements then we iterate over the lines
            let vec = numbers.get_mut(&e);
            if vec.is_none() { // If there's no line with that number then return the previous result
                opt
            } else {
                let vec = vec.unwrap();
                let boards: Vec<Rc<RefCell<Board>>> = vec.iter().fold( // Iterate over the lines
                    Vec::new(),
                    |mut boards, line| {
                        /// We fold over the lines in the vector
                        /// For each line we decrease the amount, if the line is a row then we
                        /// decrease the total of the board as well
                        ///
                        /// If the line's total is zero we add it to the vector of wins
                        if line.as_ref().borrow().board.as_ref().borrow().won {
                            return boards // If the board already won I can ignore it
                        } else {
                            line.as_ref().borrow_mut().sum -= e;
                            if line.as_ref().borrow().row {
                                line.as_ref().borrow().board.as_ref().borrow_mut().sum -= e;
                                if line.as_ref().borrow().board.as_ref().borrow().sum == 0 {
                                    println!("Uh oh zero");
                                }
                            }
                            if line.as_ref().borrow().sum == 0 {
                                line.as_ref().borrow().board.as_ref().borrow_mut().won = true;
                                //println!("Found a new row");
                                boards.push(line.as_ref().borrow().board.clone());
                            }
                            boards
                        }
                    }
                );
                let mut ret = None;
                for board in boards {
                    board.as_ref().borrow_mut().won = true;
                    ret = Some(board);
                }
                if ret.is_some(){
                    let act = ret.unwrap();
                    let r = act.as_ref().borrow().sum * e;
                    println!("later win:  {} = {} * {}", r, act.as_ref().borrow().sum, e);
                    Some(r)
                } else {
                    opt // No new wins equal we want the old win
                }
            }
        });
    if res.is_some(){
        println!("{}", res.unwrap());
    } else {
        println!("Uh oh")
    }

}
