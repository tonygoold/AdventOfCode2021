use app;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ChunkError {
    // The stack was empty
    EmptyClose(char),
    IncorrectClose { expected: char, actual: char },
    UnexpectedInput(char),
}

struct Navigator {
    stack: Vec<char>,
}

impl Navigator {
    fn new() -> Navigator {
        Navigator { stack: Vec::new() }
    }

    fn parse_char(&mut self, c: char) -> Result<(), ChunkError> {
        match c {
            '(' | '[' | '{' | '<' => {
                self.stack.push(c);
                Ok(())
            }
            ')' => self.pop_char('('),
            ']' => self.pop_char('['),
            '}' => self.pop_char('{'),
            '>' => self.pop_char('<'),
            _ => Err(ChunkError::UnexpectedInput(c)),
        }
    }

    fn pop_char(&mut self, c: char) -> Result<(), ChunkError> {
        let top = self.stack.last().ok_or(ChunkError::EmptyClose(c))?;
        if *top == c {
            self.stack.pop();
            Ok(())
        } else {
            Err(ChunkError::IncorrectClose {
                expected: *top,
                actual: c,
            })
        }
    }

    fn parse_str(&mut self, s: &str) -> Result<(), ChunkError> {
        s.chars().try_for_each(|c| self.parse_char(c))
    }
}

fn score_char(c: char) -> usize {
    match c {
        '(' => 3,
        '[' => 57,
        '{' => 1197,
        '<' => 25137,
        _ => panic!("Cannot score character {}", c),
    }
}

fn main() {
    let lines: Vec<_> = app::read_lines(&app::input_arg()).collect();
    let mut score = 0;
    for line in lines {
        let mut nav = Navigator::new();
        if let Err(err) = nav.parse_str(&line) {
            match err {
                ChunkError::EmptyClose(c) => score += score_char(c),
                ChunkError::IncorrectClose {
                    expected: _,
                    actual: c,
                } => score += score_char(c),
                ChunkError::UnexpectedInput(c) => panic!("Unexpected input: {}", c),
            };
        }
    }
    println!("Sum of error scores is {}", score);
}
