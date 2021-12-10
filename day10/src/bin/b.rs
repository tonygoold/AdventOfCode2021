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

    fn suffix(&self) -> String {
        self.stack
            .iter()
            .rev()
            .map(|c| match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => panic!("Unexpected character {} in stack", c),
            })
            .collect()
    }
}

fn score_char(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Cannot score character {}", c),
    }
}

fn main() {
    let lines: Vec<_> = app::read_lines(&app::input_arg()).collect();
    let mut scores: Vec<usize> = Vec::new();
    for line in lines {
        let mut nav = Navigator::new();
        if nav.parse_str(&line).is_ok() {
            let score = nav
                .suffix()
                .chars()
                .fold(0, |acc, c| acc * 5 + score_char(c));
            scores.push(score);
        }
    }
    scores.sort();
    println!("The median score is {}", &scores[scores.len() / 2]);
}
