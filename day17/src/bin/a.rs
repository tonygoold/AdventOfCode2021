use regex::Regex;

#[derive(Debug, Clone)]
struct Rect {
    _xmin: isize,
    _xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl Rect {
    fn new(x1: isize, x2: isize, y1: isize, y2: isize) -> Self {
        let (xmin, xmax) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (ymin, ymax) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        Rect {
            _xmin: xmin,
            ymin,
            _xmax: xmax,
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
    fn pos_y(&self) -> isize {
        self.yvel * self.steps - self.steps * (self.steps - 1) / 2
    }

    fn max_height(&self) -> isize {
        if self.yvel <= 0 {
            return 0;
        }
        self.yvel * (self.yvel + 1) / 2
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

    let solution = YIter::new(&rect).next().expect("No solution found");
    println!("Maximum y = {}", solution.max_height());
}
