use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

use app;
use app::point::Point2D;

type Point = Point2D<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold<T> {
    X(T),
    Y(T),
}

#[derive(Debug, Clone, PartialEq)]
enum ParseFoldError {
    MatchError,
    ParseError(ParseIntError),
}

impl<T> FromStr for Fold<T>
where
    T: FromStr<Err = ParseIntError>,
{
    type Err = ParseFoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^fold along (x|y)=(\d+)$").expect("Failed to compile regex");
        let caps = re.captures(s).ok_or(Self::Err::MatchError)?;
        let val = caps[2].parse::<T>().map_err(|e| Self::Err::ParseError(e))?;
        Ok(if &caps[1] == "x" {
            Fold::X(val)
        } else {
            Fold::Y(val)
        })
    }
}

fn fold_point(fold: &Fold<usize>, point: &Point) -> Point {
    let p = *point;
    match fold {
        Fold::X(axis) => {
            if p.x > *axis {
                Point::new(2 * (*axis) - p.x, p.y)
            } else {
                p
            }
        }
        Fold::Y(axis) => {
            if p.y > *axis {
                Point::new(p.x, 2 * (*axis) - p.y)
            } else {
                p
            }
        }
    }
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg());
    let mut points: HashSet<Point> = HashSet::new();
    for line in &mut lines {
        if line == "" {
            break;
        }
        points.insert(line.parse::<Point>().unwrap());
    }
    println!("Read {} points", points.len());

    let mut folds: Vec<Fold<usize>> = Vec::new();
    for line in &mut lines {
        folds.push(line.parse::<Fold<usize>>().unwrap());
    }
    println!("Read {} folds", folds.len());

    for fold in folds.iter() {
        points = HashSet::from_iter(points.into_iter().map(|p| fold_point(fold, &p)));
        match fold {
            Fold::X(axis) => println!("After folding x={} are {} points left", axis, points.len()),
            Fold::Y(axis) => println!("After folding y={} are {} points left", axis, points.len()),
        };
    }
}
