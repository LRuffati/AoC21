mod structs {
    use std::path::Iter;

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    impl Point {
        pub fn parse(x: &str, y: &str) -> Self {
            println!("{} {}", x, y);
            Point {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }
        }

        pub fn less_x(&self, otr: &Self) -> bool {
            return self.x < otr.x
        }

        pub fn less_y(&self, otr: &Self) -> bool {
            return self.y < otr.y
        }
    }

    #[derive(Clone, Copy)]
    pub enum Line {
        ///
        Vertical(usize, Point, usize),
        // x coordinate, point with lower y, len (y2-y1)
        Horizontal(usize, Point, usize),
        // y coordinate, point with smaller x, len (x2-x1)
        Forward(usize, Point, usize),
        // x+y, point with smaller x, len x2-x1
        Backward(isize, Point, usize), // x-y, point with smaller x, len x2-x1
    }

    impl Line {
        pub fn parse(point_a: Point, point_b: Point) -> Option<Line> {
            if point_a.x == point_b.x {
                if point_a.less_y(&point_b) {
                    Some(Line::Vertical(point_a.x, point_a, point_b.y - point_a.y))
                } else {
                    Some(Line::Vertical(point_a.x, point_b, point_a.y - point_b.y))
                }
            } else if point_a.y == point_b.y {
                if point_a.less_x(&point_b) {
                    Some(Line::Horizontal(point_a.y, point_a, point_b.x - point_a.x))
                } else {
                    Some(Line::Horizontal(point_a.y, point_b, point_a.x - point_b.x))
                }
            } else {
                let dx = (point_b.x as isize) - (point_a.x as isize);
                let dy = (point_b.y as isize) - (point_a.y as isize);

                let (p1, p2) = if dx > 0 {(point_a, point_b)} else {(point_b, point_a)};

                if dx == dy {
                    Some(Line::Backward(((p1.x as isize) - (p1.y as isize)), p1, (dx.abs() as usize)))
                } else if dx == -dy {
                    Some(Line::Forward(p1.x+p1.y, p1, (dx.abs() as usize)))
                } else {
                    None
                }
            }
        }
    }

    pub struct LineIter {
        line: Option<Line>,
        index: usize,
    }

    impl IntoIterator for Line {
        type Item = Point;
        type IntoIter = LineIter;

        fn into_iter(self) -> Self::IntoIter {
            LineIter {
                line: Some(self.clone()),
                index: 0,
            }
        }
    }

    impl Iterator for LineIter {
        type Item = Point;

        fn next(&mut self) -> Option<Self::Item> {
            self.index += 1;
            if let Some(x) = self.line {
                match x {
                    Line::Vertical(a, p, l) => {
                        // Return the point and update the line with the successor, if line has len 0
                        // then update line is none

                        if l == 0 {
                            self.line = None
                        } else {
                            self.line = Some(Line::Vertical(a,
                                                            Point {x: p.x, y:p.y+1},
                                                            l-1));
                        }
                        Some(p)
                    }
                    Line::Horizontal(a, p, l) => {
                        // Return the point and update the line with the successor, if line has len 0
                        // then update line is none

                        if l == 0 {
                            self.line = None
                        } else {
                            self.line = Some(Line::Horizontal(a,
                                                            Point {x: p.x+1, y:p.y},
                                                            l-1));
                        }
                        Some(p)
                    }
                    Line::Forward(a, p, l) => {
                        // Return the point and update the line with the successor, if line has len 0
                        // then update line is none

                        if l == 0 {
                            self.line = None
                        } else {
                            self.line = Some(Line::Forward(a,
                                                              Point {x: p.x+1, y:p.y-1},
                                                              l-1));
                        }
                        Some(p)
                    }
                    Line::Backward(a, p, l) => {
                        // Return the point and update the line with the successor, if line has len 0
                        // then update line is none

                        if l == 0 {
                            self.line = None
                        } else {
                            self.line = Some(Line::Backward(a,
                                                           Point {x: p.x+1, y:p.y+1},
                                                           l-1));
                        }
                        Some(p)
                    }
                }
            } else {
                None
            }
        }
    }
}

pub mod solution {
    use std::cmp::Ordering;
    use std::cmp::Ordering::Less;
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use regex::Regex;
    use crate::days::day5::structs::{Line, Point};

    fn compare_intervals(p1: &(usize, usize), p2: &(usize, usize)) -> Ordering{
        if p1.0 < p2.0 {
            Ordering::Less
        } else if p1.0 > p2.0 {
            Ordering::Greater
        } else {
            if p1.1 < p2.1 {
                Ordering::Less
            } else if p1.1 > p2.1 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

    }

    #[derive(Eq, PartialEq, Clone, Copy)]
    enum CellState {
        Once,
        More,
    }

    pub fn solve_a(input: BufReader<File>) {
        let rgx = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let lines_input = input.lines()
            .map(|ostr| {
                let ostr = ostr.unwrap();
                let captures = rgx.captures(ostr.as_str()).unwrap();
                let p1 = Point::parse(captures.get(1).unwrap().as_str(),
                                      captures.get(2).unwrap().as_str());
                let p2 = Point::parse(captures.get(3).unwrap().as_str(),
                                      captures.get(4).unwrap().as_str());
                Line::parse(p1, p2)
            }).filter(|x| x.is_some()).map(|x| x.unwrap());

        let mut map_cells: BTreeMap<Point, CellState> = BTreeMap::new();
        let mut doubles: usize = 0;
        for l in lines_input {
            for point in l.into_iter(){
                if let Some(x) = map_cells.get(&point) {
                    if *x == CellState::Once {
                        map_cells.insert(point, CellState::More);
                        doubles+=1
                    }
                } else {
                    map_cells.insert(point, CellState::Once);
                }
            }
        }
        println!("{}", doubles);
    }
}