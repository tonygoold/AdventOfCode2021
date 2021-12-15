use regex::Regex;

enum Move {
    Forward(isize),
    Down(isize),
    Up(isize),
}

use Move::{Down, Forward, Up};

fn parse_move(dir: &str, val: &str) -> Option<Move> {
    match val.parse::<isize>() {
        Err(_) => None,
        Ok(xy) => match dir {
            "forward" => Some(Forward(xy)),
            "down" => Some(Down(xy)),
            "up" => Some(Up(xy)),
            _ => None,
        },
    }
}

struct Position {
    x: isize,
    y: isize,
    aim: isize,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0, aim: 0 }
    }
    fn apply(&mut self, m: Move) {
        match m {
            Forward(x) => {
                self.x += x;
                self.y += self.aim * x;
            }
            Down(y) => self.aim += y,
            Up(y) => self.aim -= y,
        }
    }
}

fn main() {
    let re = Regex::new(r"^(forward|up|down) (\d+)$").expect("Failed to compile regex");
    let mut pos = Position::new();
    let moves = app::read_lines(&app::input_arg()).map(|line| {
        let caps = re.captures(&line).expect("Did not match line");
        match parse_move(&caps[1], &caps[2]) {
            Some(mv) => mv,
            None => panic!("Invalid move"),
        }
    });
    for mv in moves {
        pos.apply(mv);
    }

    println!(
        "The product of length {:?} and depth {:?} is {:?}",
        pos.x,
        pos.y,
        pos.x * pos.y
    );
}
