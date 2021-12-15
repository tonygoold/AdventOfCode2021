use std::collections::HashMap;

const NUM_STEPS: usize = 40;

#[derive(Debug, Clone)]
enum ParseRuleError {
    InvalidSyntax,
    WrongSizeKey,
    WrongSizeValue,
}

type CharCount = HashMap<char, usize>;

trait Mergeable {
    fn merge(&mut self, rhs: &Self);
}

impl Mergeable for CharCount {
    fn merge(&mut self, rhs: &Self) {
        for (k, v) in rhs.iter() {
            let entry = self.entry(*k).or_default();
            *entry += v;
        }
    }
}

struct Expansion {
    memory: HashMap<(char, char, usize), CharCount>,
    rules: HashMap<(char, char), char>,
}

impl Expansion {
    fn new() -> Self {
        Expansion { memory: HashMap::new(), rules: HashMap::new() }
    }

    fn add_rule(&mut self, k1: char, k2: char, v: char) {
        self.rules.insert((k1, k2), v);
    }

    fn expand_str(&mut self, s: &str, depth: usize) -> CharCount {
        let mut counts = CharCount::new();
        for c in s.chars() {
            let entry = counts.entry(c).or_default();
            *entry += 1;
        }
        let pairs = s.chars().zip(s.chars().skip(1));
        for (a, b) in pairs {
            counts.merge(&self.expand(a, b, depth));
        }
        counts
    }

    fn expand(&mut self, k1: char, k2: char, depth: usize) -> CharCount {
        if depth == 0 {
            return CharCount::new();
        }
        if let Some(count) = self.memory.get(&(k1, k2, depth)) {
            return count.clone();
        }
        let mut counts = CharCount::new();
        let c = *self.rules.get(&(k1, k2)).expect("Missing rule for char pair");
        counts.merge(&self.expand(k1, c, depth - 1));
        counts.merge(&self.expand(c, k2, depth - 1));
        let entry = counts.entry(c).or_default();
        *entry += 1;
        self.memory.insert((k1, k2, depth), counts.clone());
        counts
    }
}

fn parse_rule(s: &str) -> Result<(char, char, char), ParseRuleError> {
    let mut parts = s.split(" -> ");
    let k: Vec<char> = parts.next().map(|s| s.chars().collect()).ok_or(ParseRuleError::InvalidSyntax)?;
    let v: Vec<char> = parts.next().map(|s| s.chars().collect()).ok_or(ParseRuleError::InvalidSyntax)?;
    match (k.len(), v.len()) {
        (2, 1) => Ok((k[0], k[1], v[0])),
        (2, _) => Err(ParseRuleError::WrongSizeValue),
        _ => Err(ParseRuleError::WrongSizeKey),
    }
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg());
    let input = lines.next().expect("No input");
    lines.next(); // Consume blank separator

    let mut expansion = Expansion::new();
    lines.map(|l| parse_rule(&l).unwrap()).for_each(|(k1, k2, v)| expansion.add_rule(k1, k2, v));

    let map = expansion.expand_str(&input, NUM_STEPS);
    let minmax = map.iter().fold((' ', usize::MAX, ' ', usize::MIN), |acc, (k, v)| {
        let min = if *v < acc.1 {
            (*k, *v)
        } else {
            (acc.0, acc.1)
        };
        let max = if *v > acc.3 {
            (*k, *v)
        } else {
            (acc.2, acc.3)
        };
        (min.0, min.1, max.0, max.1)
    });
    println!("The most frequent is '{}' at {}", minmax.2, minmax.3);
    println!("The least frequent is '{}' at {}", minmax.0, minmax.1);
    println!("The difference between most and least frequent is {}", minmax.3 - minmax.1);
}
