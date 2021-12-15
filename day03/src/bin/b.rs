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

fn has_bit(col: usize, val: char) -> impl Fn(&str) -> bool {
    move |s| s.chars().nth(col).unwrap() == val
}

fn o2_rating<S: AsRef<str>>(rows: impl Iterator<Item = S> + Clone, start: usize) -> usize {
    if rows.clone().count() == 1 {
        let s = rows.last().unwrap();
        return usize::from_str_radix(s.as_ref(), 2).expect("Value is not binary");
    }
    let bits = col_bits(rows.clone(), start);
    let f = has_bit(start, if bits.1 >= bits.0 { '1' } else { '0' });
    let rem: Vec<String> = rows
        .filter(|s| f(s.as_ref()))
        .map(|s| s.as_ref().to_string())
        .collect();
    o2_rating(rem.iter(), start + 1)
}

fn co2_rating<S: AsRef<str>>(rows: impl Iterator<Item = S> + Clone, start: usize) -> usize {
    if rows.clone().count() == 1 {
        let s = rows.last().unwrap();
        return usize::from_str_radix(s.as_ref(), 2).expect("Value is not binary");
    }
    let bits = col_bits(rows.clone(), start);
    // This just reverses the bit filter from o2_rating, since:
    // (bits.0 <= bits.1) iff (bits1 >= bits.0)
    let f = has_bit(start, if bits.0 <= bits.1 { '0' } else { '1' });
    let rem: Vec<String> = rows
        .filter(|s| f(s.as_ref()))
        .map(|s| s.as_ref().to_string())
        .collect();
    co2_rating(rem.iter(), start + 1)
}

fn main() {
    let rows: Vec<String> = app::read_lines(&app::input_arg()).collect();
    if rows.is_empty() {
        panic!("No input to process")
    }

    let o2 = o2_rating(rows.iter(), 0);
    let co2 = co2_rating(rows.iter(), 0);

    println!(
        "The product of O2 {:?} and Co2 {:?} is {:?}",
        o2,
        co2,
        o2 * co2
    );
}
