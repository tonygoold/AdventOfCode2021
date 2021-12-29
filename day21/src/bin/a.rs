use regex::Regex;

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
}

struct Die {
    rolls: usize,
}

impl Die {
    fn new() -> Self {
        Die { rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        ((self.rolls - 1) % 100) + 1
    }

    fn num_rolls(&self) -> usize {
        self.rolls
    }
}

struct Game {
    die: Die,
    p1_turn: bool,
    player1: Player,
    player2: Player,
}

impl Game {
    fn new(p1: usize, p2: usize) -> Self {
        Game {
            die: Die::new(),
            p1_turn: true,
            player1: Player::new(p1),
            player2: Player::new(p2),
        }
    }

    fn take_turn(&mut self) {
        let player = if self.p1_turn {
            &mut self.player1
        } else {
            &mut self.player2
        };
        let delta = self.die.roll() + self.die.roll() + self.die.roll();
        player.advance(delta);
        self.p1_turn = !self.p1_turn;
    }

    fn finished(&self) -> bool {
        self.player1.score >= 1000 || self.player2.score >= 1000
    }

    fn scores(&self) -> (usize, usize) {
        (self.player1.score, self.player2.score)
    }

    fn num_rolls(&self) -> usize {
        self.die.num_rolls()
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
    let loser = if scores.0 >= 1000 { scores.1 } else { scores.0 };
    let result = loser * game.num_rolls();
    println!("Final result: {}", result);
}
