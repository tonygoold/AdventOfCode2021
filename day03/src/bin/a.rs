use app;

// Returns (num_zeros, num_ones)
fn col_bits<S: AsRef<str>>(rows: impl Iterator<Item = S>, col: usize) -> (usize, usize) {
    rows.fold((0, 0), |acc @ (zeros, ones), row| {
        let c = row.as_ref().chars().nth(col).unwrap();
        match c {
            '0' => (zeros + 1, ones),
            '1' => (zeros, ones + 1),
            _ => acc,
        }
    })
}

fn main() {
    let rows: Vec<String> = app::read_lines(&app::input_arg()).collect();
    if rows.is_empty() {
        panic!("No input to process")
    }

    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for i in 0..rows[0].len() {
        gamma <<= 1;
        epsilon <<= 1;
        let counts = col_bits(rows.iter(), i);
        if counts.1 > counts.0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!(
        "The product of gamma {:?} and epsilon {:?} is {:?}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
