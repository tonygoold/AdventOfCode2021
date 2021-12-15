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

fn main() {
    let panels: Vec<Panel> = app::read_lines(&app::input_arg())
        .map(|l| l.parse::<Panel>().unwrap())
        .collect();

    let mut counts = [0usize; 10];
    for segment in panels.iter().flat_map(|p| p.displays.iter()) {
        match segment.count() {
            2 => counts[1] += 1,
            3 => counts[7] += 1,
            4 => counts[4] += 1,
            7 => counts[8] += 1,
            _ => {}
        };
    }

    println!(
        "There are {} 1s, {} 4s, {} 7s, and {} 8s for a total of {}",
        counts[1],
        counts[4],
        counts[7],
        counts[8],
        counts[1] + counts[4] + counts[7] + counts[8],
    )
}
