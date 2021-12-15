use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseSegmentError {
    InvalidChar(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsePanelError {
    MissingSeparator,
    WrongNumDisplays,
    InvalidSegment(ParseSegmentError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SegmentSet {
    value: u8,
}

struct Panel {
    inputs: Vec<SegmentSet>,
    displays: [SegmentSet; 4],
}

impl SegmentSet {
    fn count(&self) -> usize {
        self.value.count_ones() as usize
    }

    fn intersect(&self, rhs: &Self) -> Self {
        SegmentSet {
            value: self.value & rhs.value,
        }
    }
}

impl FromStr for SegmentSet {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0u8;
        for c in s.chars() {
            match c {
                'a' => value |= 0b0000001,
                'b' => value |= 0b0000010,
                'c' => value |= 0b0000100,
                'd' => value |= 0b0001000,
                'e' => value |= 0b0010000,
                'f' => value |= 0b0100000,
                'g' => value |= 0b1000000,
                _ => return Err(Self::Err::InvalidChar(c)),
            }
        }
        Ok(SegmentSet { value })
    }
}

impl FromStr for Panel {
    type Err = ParsePanelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(" | ");
        let left = halves.next().ok_or(Self::Err::MissingSeparator)?;
        let right = halves.next().ok_or(Self::Err::MissingSeparator)?;
        let inputs: Vec<SegmentSet> =
            left.split(' ')
                .map(|s| s.parse::<SegmentSet>())
                .fold(Ok(Vec::new()), |r, x| {
                    if let Ok(mut v) = r {
                        match x {
                            Ok(segment) => {
                                v.push(segment);
                                Ok(v)
                            }
                            Err(e) => Err(Self::Err::InvalidSegment(e)),
                        }
                    } else {
                        r
                    }
                })?;
        let outputs: Vec<SegmentSet> =
            right
                .split(' ')
                .map(|s| s.parse::<SegmentSet>())
                .fold(Ok(Vec::new()), |r, x| {
                    if let Ok(mut v) = r {
                        match x {
                            Ok(segment) => {
                                v.push(segment);
                                Ok(v)
                            }
                            Err(e) => Err(Self::Err::InvalidSegment(e)),
                        }
                    } else {
                        r
                    }
                })?;
        if outputs.len() != 4 {
            return Err(Self::Err::WrongNumDisplays);
        }
        Ok(Panel {
            inputs,
            displays: [outputs[0], outputs[1], outputs[2], outputs[3]],
        })
    }
}

impl Panel {
    fn solve_digits(&self) -> Option<[SegmentSet; 10]> {
        if self.inputs.len() != 10 {
            return None;
        }
        let one = self.inputs.iter().find(|s| s.count() == 2)?;
        let four = self.inputs.iter().find(|s| s.count() == 4)?;
        let seven = self.inputs.iter().find(|s| s.count() == 3)?;
        let eight = self.inputs.iter().find(|s| s.count() == 7)?;
        let three = self
            .inputs
            .iter()
            .find(|s| s.count() == 5 && s.intersect(one).count() == 2)?;
        let six = self
            .inputs
            .iter()
            .find(|s| s.count() == 6 && s.intersect(one).count() == 1)?;
        let nine = self
            .inputs
            .iter()
            .find(|s| s.count() == 6 && s.intersect(three).count() == 5)?;
        let zero = self
            .inputs
            .iter()
            .find(|s| s.count() == 6 && *s != six && *s != nine)?;
        let five = self
            .inputs
            .iter()
            .find(|s| s.count() == 5 && s.intersect(six).count() == 5)?;
        let two = self
            .inputs
            .iter()
            .find(|s| s.count() == 5 && *s != three && *s != five)?;
        Some([
            *zero, *one, *two, *three, *four, *five, *six, *seven, *eight, *nine,
        ])
    }

    fn solve_value(&self) -> Option<usize> {
        let values = self.solve_digits()?;
        let mut sum = 0;
        for display in self.displays.iter() {
            let value = values.iter().position(|d| d == display)?;
            sum = sum * 10 + value;
        }
        Some(sum)
    }
}

fn main() {
    let panels: Vec<Panel> = app::read_lines(&app::input_arg())
        .map(|l| l.parse::<Panel>().unwrap())
        .collect();

    let sum: usize = panels
        .iter()
        .map(|p| p.solve_value().expect("Unsolved"))
        .sum();

    println!("The sum of all displays is {}", sum);
}
