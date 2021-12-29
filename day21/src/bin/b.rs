use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Player { score: 0, position }
    }

    fn advance(&mut self, n: usize) {
        self.position = ((self.position + n - 1) % 10) + 1;
        self.score += self.position;
    }

    fn split(&self) -> [Self; 27] {
        let mut states = [*self; 27];
        let mut i = 0;
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                for d3 in 1..=3 {
                    states[i].advance(d1 + d2 + d3);
                    i += 1;
                }
            }
        }
        states
    }

    fn won(&self) -> bool {
        self.score >= 21
    }
}

struct Game {
    p1_turn: bool,
    states: HashMap<(Player, Player), usize>,
}

impl Game {
    fn new(p1: usize, p2: usize) -> Self {
        let p1 = Player::new(p1);
        let p2 = Player::new(p2);
        Game {
            p1_turn: true,
            states: HashMap::from([((p1, p2), 1)]),
        }
    }

    fn take_turn(&mut self) {
        let mut state_new = HashMap::new();
        self.states.iter().for_each(|(&(p1, p2), &count)| {
            if p1.won() || p2.won() {
                *state_new.entry((p1, p2)).or_default() += count;
            } else if self.p1_turn {
                for state in p1.split() {
                    *state_new.entry((state, p2)).or_default() += count;
                }
            } else {
                for state in p2.split() {
                    *state_new.entry((p1, state)).or_default() += count;
                }
            }
        });
        self.states = state_new;
        self.p1_turn = !self.p1_turn;
    }

    fn finished(&self) -> bool {
        self.states.keys().all(|(p1, p2)| p1.won() || p2.won())
    }

    fn scores(&self) -> (usize, usize) {
        self.states.iter().fold((0, 0), |acc, ((p1, p2), &count)| {
            if p1.won() {
                (acc.0 + count, acc.1)
            } else if p2.won() {
                (acc.0, acc.1 + count)
            } else {
                acc
            }
        })
    }
}

fn read_position(s: &str) -> usize {
    let re = Regex::new(r"^Player \d starting position: (\d+)").expect("Failed to compile regex");
    if let Some(caps) = re.captures(s) {
        caps[1].parse().expect("Non-numeric position")
    } else {
        panic!("Invalid input")
    }
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg());
    let p1 = read_position(&lines.next().expect("Missing player 1"));
    let p2 = read_position(&lines.next().expect("Missing player 2"));
    let mut game = Game::new(p1, p2);
    while !game.finished() {
        game.take_turn();
    }
    let scores = game.scores();
    if scores.0 > scores.1 {
        println!("Player 1 wins: {}", scores.0);
    } else {
        println!("Player 2 wins: {}", scores.1);
    }
}
