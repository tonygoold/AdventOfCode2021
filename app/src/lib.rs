use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod grid;
pub mod point;

pub fn input_arg() -> String {
    env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string())
}

pub fn read_line(path: &str) -> String {
    read_lines(path).next().expect("No lines of input")
}

pub fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).expect("Unable to read input file");
    let reader = BufReader::new(f);
    reader
        .lines()
        .into_iter()
        .map(|x| x.expect("Unable to read input line"))
}

pub fn read_uints(path: &str) -> impl Iterator<Item = usize> {
    read_lines(path).map(|x| {
        x.parse::<usize>()
            .expect("Line was not an unsigned integer")
    })
}
