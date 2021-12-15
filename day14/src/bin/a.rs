use std::collections::HashMap;

const NUM_STEPS: usize = 10;

fn expand(s: &str, rules: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut key = String::new();
    let pairs = s.chars().zip(s.chars().skip(1));
    for (a, b) in pairs {
        key.clear();
        key.push(a);
        key.push(b);

        match rules.get(&key) {
            Some(val) => {
                result.push(a);
                result.push_str(val);
            }
            None => panic!("Failed to look up {}", &key),
        }
    }
    result.push(s.chars().last().unwrap());
    result
}

fn char_map(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let entry = map.entry(c).or_default();
        *entry += 1;
    }
    map
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg());
    let mut rules: HashMap<String, String> = HashMap::new();
    let mut chain = lines.next().expect("No input");
    lines.next(); // Consume blank separator
    for line in lines {
        let mut parts = line.split(" -> ");
        let k = parts.next().expect("Missing left hand side");
        let v = parts.next().expect("Missing right hand side");
        rules.insert(k.into(), v.into());
    }

    for _ in 0..NUM_STEPS {
        chain = expand(&chain, &rules);
    }

    let map = char_map(&chain);
    let minmax = map
        .iter()
        .fold((' ', usize::MAX, ' ', usize::MIN), |acc, (k, v)| {
            let min = if *v < acc.1 { (*k, *v) } else { (acc.0, acc.1) };
            let max = if *v > acc.3 { (*k, *v) } else { (acc.2, acc.3) };
            (min.0, min.1, max.0, max.1)
        });
    println!("The most frequent is '{}' at {}", minmax.2, minmax.3);
    println!("The least frequent is '{}' at {}", minmax.0, minmax.1);
    println!(
        "The difference between most and least frequent is {}",
        minmax.3 - minmax.1
    );
}
