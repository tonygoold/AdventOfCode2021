use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsePointError {
    WrongDimensions(usize),
    BadCoord(ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T> FromStr for Point2D<T>
    where T: FromStr<Err = ParseIntError>,
{
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ns = s
            .split(',')
            .map(|n| n.parse::<T>().map_err(|e| Self::Err::BadCoord(e)));
        let x = ns.next().unwrap_or(Err(Self::Err::WrongDimensions(0)))?;
        let y = ns.next().unwrap_or(Err(Self::Err::WrongDimensions(1)))?;
        match ns.count() {
            0 => Ok(Self { x, y }),
            n => Err(Self::Err::WrongDimensions(n + 2)),
        }
    }
}
