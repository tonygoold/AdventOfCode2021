use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Rect {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl Rect {
    fn new(x1: isize, x2: isize, y1: isize, y2: isize) -> Self {
        let (xmin, xmax) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (ymin, ymax) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        Rect {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Solution {
    xvel: isize,
    yvel: isize,
    steps: isize,
}

impl Solution {
    fn pos_x(&self) -> isize {
        let (xvel, steps) = (self.xvel, self.steps);
        if xvel < steps {
            xvel * (xvel + 1) / 2
        } else {
            xvel * steps - (steps - 1) * steps / 2
        }
    }

    fn pos_y(&self) -> isize {
        self.yvel * self.steps - self.steps * (self.steps - 1) / 2
    }
}

struct YIter {
    rect: Rect,
    current: Solution,
}

impl YIter {
    fn new(r: &Rect) -> Self {
        YIter {
            rect: r.clone(),
            current: Solution {
                xvel: 0,
                yvel: -r.ymin,
                steps: 0,
            },
        }
    }
}

impl Iterator for YIter {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        let (top, bottom) = (self.rect.ymax, self.rect.ymin);
        while self.current.yvel >= bottom {
            self.current.steps += 1;
            let y = self.current.pos_y();
            if y < bottom {
                self.current.yvel -= 1;
                self.current.steps = 0;
            } else if y <= top {
                return Some(self.current.clone());
            }
        }
        None
    }
}

struct XIter {
    rect: Rect,
    current: Solution,
    xmax: isize,
}

impl XIter {
    fn new(r: &Rect, base: &Solution) -> Self {
        let xmin = if r.xmin < 0 { r.xmin } else { 0 };
        let xmax = if r.xmax > 0 { r.xmax } else { 0 };
        let mut base = base.clone();
        base.xvel = xmin - 1;
        XIter {
            rect: r.clone(),
            current: base,
            xmax,
        }
    }
}

impl Iterator for XIter {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        let (left, right) = (self.rect.xmin, self.rect.xmax);
        while self.current.xvel <= self.xmax {
            self.current.xvel += 1;
            let x = self.current.pos_x();
            if x >= left && x <= right {
                return Some(self.current.clone());
            }
        }
        None
    }
}

fn main() {
    let input = app::read_line(&app::input_arg());
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let caps = re
        .captures(&input)
        .expect("Input does not match expectation");
    let rect = Rect::new(
        caps[1].parse().unwrap(),
        caps[2].parse().unwrap(),
        caps[3].parse().unwrap(),
        caps[4].parse().unwrap(),
    );

    let solutions: Vec<Solution> = YIter::new(&rect)
        .flat_map(|base| XIter::new(&rect, &base))
        .collect();

    // Some solutions hit the target area more than once. Unify solutions that
    // differ only by number of steps.
    let mut uniques = HashSet::new();
    for solution in solutions.iter() {
        uniques.insert(Solution {
            xvel: solution.xvel,
            yvel: solution.yvel,
            steps: 0,
        });
    }
    println!("Found {} solutions", uniques.len());
}
