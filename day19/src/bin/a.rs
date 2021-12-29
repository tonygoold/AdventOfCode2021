use std::collections::{HashSet, VecDeque};
use std::num::ParseIntError;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

/*
This is a point set registration problem (https://en.wikipedia.org/wiki/Point_set_registration).
Specifically, it is a rigid, simultaneous pose and correspondence registration, with the additional
constraint that rotations must be multiples of 90 degrees about the X, Y, and Z axes.

A general solution is Iterative Closest Point: https://en.wikipedia.org/wiki/Iterative_closest_point.

This isn't Iterative Closest Point, though it is conceptually similar. For each of the 24 possible
rotations, it looks for a translation that produces the most exact matches. Based on the problem
statement, a transformation is accepted if and only if it produces 12 exact matches.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Transform {
    cells: [isize; 16],
}

impl Transform {
    fn identity() -> Self {
        let cells = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
        Transform { cells }
    }

    fn rotate_x() -> Self {
        let cells = [1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1];
        Transform { cells }
    }

    fn rotate_y() -> Self {
        let cells = [0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 1];
        Transform { cells }
    }

    fn rotate_z() -> Self {
        let cells = [0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
        Transform { cells }
    }

    fn translate(x: isize, y: isize, z: isize) -> Self {
        let cells = [1, 0, 0, x, 0, 1, 0, y, 0, 0, 1, z, 0, 0, 0, 1];
        Transform { cells }
    }
}

impl Mul for Transform {
    type Output = Self;

    #[allow(clippy::identity_op, clippy::erasing_op)]
    fn mul(self, other: Self) -> Self {
        let mut cells = [0; 16];
        for j in 0..4 {
            for i in 0..4 {
                cells[j * 4 + i] = self.cells[j * 4 + 0] * other.cells[0 * 4 + i]
                    + self.cells[j * 4 + 1] * other.cells[1 * 4 + i]
                    + self.cells[j * 4 + 2] * other.cells[2 * 4 + i]
                    + self.cells[j * 4 + 3] * other.cells[3 * 4 + i];
            }
        }
        Transform { cells }
    }
}

impl Mul<Position> for Transform {
    type Output = Position;

    fn mul(self, p: Position) -> Position {
        Position {
            x: self.cells[0] * p.x + self.cells[1] * p.y + self.cells[2] * p.z + self.cells[3],
            y: self.cells[4] * p.x + self.cells[5] * p.y + self.cells[6] * p.z + self.cells[7],
            z: self.cells[8] * p.x + self.cells[9] * p.y + self.cells[10] * p.z + self.cells[11],
        }
    }
}

/*
If only 90-degree rotations about the X, Y, and Z axes are permitted, then the set of rotations
corresponds to the symmetric group S4: https://en.wikiversity.org/wiki/Symmetric_group_S4

There are different ways of deriving the 24 unique orientations. I've taken a less elegant
approach and assumed every rotation can be expressed as (X^a Y^b Z^c), where X, Y, and Z
represent a 90 degree rotation about that axis, and a, b, and c are in {0, 1, 2, 3}. This
indeed generates 24 unique transformation matrices. Here, rot(a, b, c) = (X^a Y^b Z^c):

    rot(0, 0, 0)
    rot(1, 0, 0)
    rot(2, 0, 0)
    rot(3, 0, 0)
    rot(0, 1, 0)
    rot(1, 1, 0)
    rot(2, 1, 0)
    rot(3, 1, 0)
    rot(0, 2, 0)
    rot(1, 2, 0)
    rot(2, 2, 0)
    rot(3, 2, 0)
    rot(0, 3, 0)
    rot(1, 3, 0)
    rot(2, 3, 0)
    rot(3, 3, 0)
    rot(0, 0, 1)
    rot(1, 0, 1)
    rot(2, 0, 1)
    rot(3, 0, 1)
    rot(0, 2, 1)
    rot(1, 2, 1)
    rot(2, 2, 1)
    rot(3, 2, 1)
*/
#[derive(Clone, PartialEq, Eq)]
struct Alignment {
    rx: u8,
    ry: u8,
    rz: u8,
    tx: isize,
    ty: isize,
    tz: isize,
}

impl Alignment {
    fn new() -> Self {
        Alignment {
            rx: 0,
            ry: 0,
            rz: 0,
            tx: 0,
            ty: 0,
            tz: 0,
        }
    }

    fn transform(&self) -> Transform {
        let mut trans = Transform::identity();
        (0..self.rx).for_each(|_| trans = Transform::rotate_x() * trans);
        (0..self.ry).for_each(|_| trans = Transform::rotate_y() * trans);
        (0..self.rz).for_each(|_| trans = Transform::rotate_z() * trans);
        if self.tx != 0 || self.ty != 0 || self.tz != 0 {
            trans = Transform::translate(self.tx, self.ty, self.tz) * trans;
        }
        trans
    }

    fn orientations() -> Vec<Alignment> {
        let mut result = Vec::new();
        result.reserve(24);
        for rx in 0..4 {
            for ry in 0..4 {
                result.push(Alignment {
                    rx,
                    ry,
                    rz: 0,
                    tx: 0,
                    ty: 0,
                    tz: 0,
                });
                if ry == 0 || ry == 2 {
                    result.push(Alignment {
                        rx,
                        ry,
                        rz: 1,
                        tx: 0,
                        ty: 0,
                        tz: 0,
                    });
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
enum ParsePositionError {
    WrongDimensions,
    BadCoord(ParseIntError),
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ns = s
            .split(',')
            .map(|n| n.parse::<isize>().map_err(Self::Err::BadCoord));
        let x = ns.next().unwrap_or(Err(Self::Err::WrongDimensions))?;
        let y = ns.next().unwrap_or(Err(Self::Err::WrongDimensions))?;
        let z = ns.next().unwrap_or(Err(Self::Err::WrongDimensions))?;
        if ns.next().is_some() {
            Err(Self::Err::WrongDimensions)
        } else {
            Ok(Position { x, y, z })
        }
    }
}

#[derive(Clone)]
struct Scanner {
    beacons: HashSet<Position>,
}

impl Scanner {
    fn parse(lines: &mut impl Iterator<Item = String>) -> Option<Self> {
        let header = lines.next()?;
        if !header.starts_with("--- scanner ") {
            panic!("Unexpected input instead of scanner header: {}", &header);
        }
        let mut beacons = HashSet::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            beacons.insert(line.parse::<Position>().unwrap());
        }
        Some(Scanner { beacons })
    }

    fn align(&self, scanner: &Self) -> Option<Alignment> {
        for orientation in Alignment::orientations() {
            let alignment = self.align_with_orientation(scanner, &orientation);
            if alignment.is_some() {
                return alignment;
            }
        }
        None
    }

    fn align_with_orientation(&self, scanner: &Self, orientation: &Alignment) -> Option<Alignment> {
        /*
        This rotates all the other scanner's beacons using the given orientation.
        Pair each rotated beacon with every existing (self) beacon.
        For each pair, create the translation that overlays the rotated beacon on the existing beacon.
        Apply that translation to all the other rotated beacons.
        If at least 12 translated beacons match up, then accept this as a solution.
        */
        let tx = orientation.transform();
        let ps: Vec<Position> = scanner.beacons.iter().map(|p| tx * (*p)).collect();
        for p1 in ps.iter() {
            for p2 in self.beacons.iter() {
                let delta = *p2 - *p1;
                let candidates = ps
                    .iter()
                    .map(|&p| p + delta)
                    .filter(|p| self.beacons.contains(p));
                if candidates.count() >= 12 {
                    let mut alignment = orientation.clone();
                    alignment.tx = delta.x;
                    alignment.ty = delta.y;
                    alignment.tz = delta.z;
                    return Some(alignment);
                }
            }
        }
        None
    }
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg());
    // let mut scanners = Vec::new();
    let mut scanners = VecDeque::new();
    while let Some(scanner) = Scanner::parse(&mut lines) {
        scanners.push_back(scanner);
    }
    println!("Read {} scanners", scanners.len());

    let mut base = scanners.pop_front().expect("Did not read any input");
    let mut alignments = vec![(base.clone(), Alignment::new())];
    while let Some(scanner) = scanners.pop_front() {
        println!("Aligning scanner...");
        if let Some(alignment) = base.align(&scanner) {
            println!("Aligned! Adding transformed to base");
            let tx = alignment.transform();
            let ps = scanner.beacons.iter().map(|p| tx * (*p));
            base.beacons.extend(ps);
            alignments.push((scanner, alignment));
        } else if !scanners.is_empty() {
            println!("Cannot align, returing to back of queue");
            scanners.push_back(scanner);
        } else {
            panic!("Unable to find alignment");
        }
    }
    println!("All scanners aligned!");
    println!("Found {} beacons", base.beacons.len());
}
