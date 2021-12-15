struct BingoCard {
    cells: [u8; 25],
    called: [bool; 25],
}

impl BingoCard {
    fn new() -> BingoCard {
        BingoCard {
            cells: [0; 25],
            called: [false; 25],
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.cells[5 * y + x]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.cells[5 * y + x] = val;
    }

    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.called[5 * y + x]
    }

    fn mark(&mut self, x: usize, y: usize) {
        self.called[5 * y + x] = true;
    }

    fn mark_value(&mut self, val: u8) -> bool {
        for y in 0..5 {
            for x in 0..5 {
                if self.get(x, y) == val {
                    self.mark(x, y);
                    return true;
                }
            }
        }
        false
    }

    fn score(&self, val: u8) -> usize {
        let mut score = 0usize;
        for y in 0..5 {
            for x in 0..5 {
                if !self.is_marked(x, y) {
                    score += self.get(x, y) as usize;
                }
            }
        }
        score * val as usize
    }

    fn wins(&self) -> bool {
        for i in 0..5 {
            // Row i or column i
            if (0..5).all(|j| self.is_marked(i, j)) || (0..5).all(|j| self.is_marked(j, i)) {
                return true;
            }
        }
        // "Diagonals don't count"
        // (0..5).all(|i| self.is_marked(i, i)) || (0..5).all(|i| self.is_marked(i, 4 - i))
        false
    }

    fn from_strings(lines: &[String]) -> Option<BingoCard> {
        if lines.len() != 5 {
            return None;
        }
        let mut card = BingoCard::new();
        for (j, line) in lines.iter().enumerate() {
            let ns = line
                .split(' ')
                .filter(|l| l.len() > 0)
                .map(|l| l.parse::<u8>().expect("Invalid card value"));
            for (i, n) in ns.enumerate() {
                card.set(i, j, n);
            }
        }
        Some(card)
    }
}

fn main() {
    let rows: Vec<String> = app::read_lines(&app::input_arg()).collect();

    // A blank line precedes each card
    if rows.len() < 7 || (rows.len() - 1) % 6 != 0 {
        panic!("Input has invalid number of lines");
    }

    let input: Vec<u8> = rows[0]
        .split(',')
        .map(|l| l.parse::<u8>().expect("Invalid input value"))
        .collect();

    let mut cards: Vec<BingoCard> = rows[1..]
        .chunks_exact(6)
        .map(|chunk| BingoCard::from_strings(&chunk[1..]).expect("Invalid card"))
        .collect();

    for &x in input.iter() {
        cards.retain(|card| !card.wins());
        for card in cards.iter_mut() {
            card.mark_value(x);
        }
        if cards.len() == 1 && cards[0].wins() {
            println!(
                "Final winning card on input {:?} has score {:?}",
                x,
                cards[0].score(x)
            );
            return;
        }
    }
}
