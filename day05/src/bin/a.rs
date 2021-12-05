use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use app;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsePointError {
    WrongDimensions(usize),
    BadCoord(ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseSegmentError {
    WrongNumPoints(usize),
    BadPoint(ParsePointError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Segment {
    p1: Point,
    p2: Point,
}

struct Grid {
    lines: Vec<Segment>,
}

fn ord_to_delta(o: Ordering) -> isize {
    match o {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ns = s.split(',')
            .map(|n| n.parse::<isize>().map_err(|e| Self::Err::BadCoord(e)));
        let x = ns.next().unwrap_or(Err(Self::Err::WrongDimensions(0)))?;
        let y = ns.next().unwrap_or(Err(Self::Err::WrongDimensions(1)))?;
        match ns.count() {
            0 => Ok(Point{x, y}),
            n => Err(Self::Err::WrongDimensions(n+2)),
        }
    }
}

impl Segment {
    fn slope(&self) -> (isize, isize) {
        (
            ord_to_delta(self.p1.x.cmp(&self.p2.x)),
            ord_to_delta(self.p1.y.cmp(&self.p2.y)),
        )
    }

    fn is_axis_aligned(&self) -> bool {
        match self.slope() {
            (0, 0) => false,
            (0, _) | (_, 0) => true,
            _ => false
        }
    }

    fn points(&self) -> Vec<Point> {
        let (dx, dy) = self.slope();
        if dx == 0 && dy == 0 {
            panic!("Segment is a point");
        } else if dx != 0 && dy != 0 {
            panic!("Should only consider horizontal and vertical lines");
        }

        let mut p = self.p1;
        let mut ps = vec![p];
        let mut sanity_check = 0;
        while p != self.p2 {
            p.x += dx;
            p.y += dy;
            ps.push(p);
            sanity_check += 1;
            if sanity_check > 1_000_000 {
                panic!("Failed sanity check");
            }
        }
        ps
    }
}

impl FromStr for Segment {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ps = s
            .split(" -> ")
            .map(|p| p.parse::<Point>().map_err(|e| Self::Err::BadPoint(e)));
        let p1 = ps.next().unwrap_or(Err(Self::Err::WrongNumPoints(0)))?;
        let p2 = ps.next().unwrap_or(Err(Self::Err::WrongNumPoints(1)))?;
        match ps.count() {
            0 => Ok(Segment{p1, p2}),
            n => Err(Self::Err::WrongNumPoints(n+2)),
        }
    }
}

impl Grid {
    fn new(lines: Vec<Segment>) -> Grid {
        Grid{lines}
    }

    fn num_lines(&self) -> usize {
        self.lines.len()
    }

    fn coverage(&self) -> HashMap<Point, usize> {
        let mut map = HashMap::new();
        let ps = self.lines.iter()
            .filter(|l| l.is_axis_aligned())
            .map(|l| l.points().into_iter())
            .flatten();
        for p in ps {
            let count = map.entry(p).or_default();
            *count += 1;
        }
        map
    }
}

fn main() {
    let lines: Vec<Segment> = app::read_lines(&app::input_arg())
        .map(|line| line.parse::<Segment>().unwrap())
        .collect();

    let grid = Grid::new(lines);
    println!("Read {:?} lines of input", grid.num_lines());

    let collisions = grid.coverage().into_iter().filter(|(_, c)| *c > 1);
    println!("There were {:?} collisions", collisions.count());
}
