use std::collections::HashSet;

use regex::Regex;

use app::point::Point3D;

const MIN_COORD: isize = -50;
const MAX_COORD: isize = 50;

struct Command {
    on: bool,
    min: Point3D<isize>,
    max: Point3D<isize>,
}

impl Command {
    fn iter(&self) -> BoxIter {
        BoxIter::new(self.min, self.max)
    }
}

struct BoxIter {
    min: Point3D<isize>,
    max: Point3D<isize>,
    cur: Point3D<isize>,
}

impl BoxIter {
    fn new(min: Point3D<isize>, max: Point3D<isize>) -> Self {
        BoxIter { min, max, cur: min }
    }
}

impl Iterator for BoxIter {
    type Item = Point3D<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.z > self.max.z {
            None
        } else {
            let val = self.cur;
            self.cur.x += 1;
            if self.cur.x > self.max.x {
                self.cur.x = self.min.x;
                self.cur.y += 1;
            }
            if self.cur.y > self.max.y {
                self.cur.y = self.min.y;
                self.cur.z += 1;
            }
            Some(val)
        }
    }
}

fn parse_command(s: &str) -> Command {
    let num_range = r"(-?\d+)..(-?\d+)";
    let mut pattern = String::from("(on|off) x=");
    pattern += num_range;
    pattern += ",y=";
    pattern += num_range;
    pattern += ",z=";
    pattern += num_range;
    let re = Regex::new(&pattern).expect("Failed to compile regex");
    let caps = re.captures(s).expect("Failed to match input");
    let on = &caps[1] == "on";
    let (x1, x2, y1, y2, z1, z2) = (
        caps[2].parse().expect("Invalid number"),
        caps[3].parse().expect("Invalid number"),
        caps[4].parse().expect("Invalid number"),
        caps[5].parse().expect("Invalid number"),
        caps[6].parse().expect("Invalid number"),
        caps[7].parse().expect("Invalid number"),
    );
    let min = Point3D::new(x1, y1, z1);
    let max = Point3D::new(x2, y2, z2);
    Command { on, min, max }
}

fn main() {
    let commands = app::read_lines(&app::input_arg())
        .map(|s| parse_command(&s))
        .filter(|c| {
            c.min.x >= MIN_COORD
                && c.min.y >= MIN_COORD
                && c.min.z >= MIN_COORD
                && c.max.x <= MAX_COORD
                && c.max.y <= MAX_COORD
                && c.max.z <= MAX_COORD
        });
    let mut cubes = HashSet::new();

    for command in commands {
        for point in command.iter() {
            if command.on {
                cubes.insert(point);
            } else {
                cubes.remove(&point);
            }
        }
    }
    println!("There are {} cubes on", cubes.len());
}
