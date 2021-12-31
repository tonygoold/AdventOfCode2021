use regex::Regex;

use app::point::Point3D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    min: Point3D<isize>,
    max: Point3D<isize>,
}

impl Cuboid {
    fn new(min: Point3D<isize>, max: Point3D<isize>) -> Self {
        Self { min, max }
    }

    fn volume(&self) -> isize {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        dx * dy * dz
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let min = Point3D::new(
            self.min.x.max(other.min.x),
            self.min.y.max(other.min.y),
            self.min.z.max(other.min.z),
        );
        let max = Point3D::new(
            self.max.x.min(other.max.x),
            self.max.y.min(other.max.y),
            self.max.z.min(other.max.z),
        );
        if min.x >= max.x || min.y >= max.y || min.z >= max.z {
            None
        } else {
            Some(Cuboid::new(min, max))
        }
    }
}

struct NegationTree {
    region: Cuboid,
    negations: Vec<NegationTree>,
}

impl NegationTree {
    fn new(region: Cuboid) -> Self {
        Self {
            region,
            negations: Vec::new(),
        }
    }

    fn negate(&mut self, region: &Cuboid) {
        let intersection = match self.region.intersection(region) {
            Some(cuboid) => cuboid,
            None => return,
        };
        for negation in self.negations.iter_mut() {
            negation.negate(&intersection);
        }
        self.negations.push(Self::new(intersection));
    }

    fn volume(&self) -> isize {
        let neg_volume: isize = self
            .negations
            .iter()
            .map(|negation| negation.volume())
            .sum();
        if neg_volume > self.region.volume() {
            panic!("Encountered negative volume space");
        }
        self.region.volume() - neg_volume
    }
}

struct NegationSpace {
    trees: Vec<NegationTree>,
}

impl NegationSpace {
    fn new() -> Self {
        Self { trees: Vec::new() }
    }

    fn add(&mut self, region: &Cuboid) {
        self.subtract(region);
        self.trees.push(NegationTree::new(*region));
    }

    fn subtract(&mut self, region: &Cuboid) {
        for tree in self.trees.iter_mut() {
            tree.negate(region);
        }
    }

    fn volume(&self) -> isize {
        self.trees.iter().map(|tree| tree.volume()).sum()
    }
}

struct Command {
    on: bool,
    cuboid: Cuboid,
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
        caps[2].parse::<isize>().expect("Invalid number"),
        caps[3].parse::<isize>().expect("Invalid number"),
        caps[4].parse::<isize>().expect("Invalid number"),
        caps[5].parse::<isize>().expect("Invalid number"),
        caps[6].parse::<isize>().expect("Invalid number"),
        caps[7].parse::<isize>().expect("Invalid number"),
    );
    let min = Point3D::new(x1, y1, z1);
    let max = Point3D::new(x2 + 1, y2 + 1, z2 + 1);
    Command {
        on,
        cuboid: Cuboid::new(min, max),
    }
}

fn main() {
    let commands = app::read_lines(&app::input_arg()).map(|s| parse_command(&s));

    let mut space = NegationSpace::new();
    for command in commands {
        if command.on {
            println!("Adding {:?}", &command.cuboid);
            space.add(&command.cuboid);
        } else {
            println!("Subtracting {:?}", &command.cuboid);
            space.subtract(&command.cuboid);
        }
    }
    println!("There are {} cubes on", space.volume());
}
