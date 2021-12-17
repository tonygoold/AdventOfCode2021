use std::mem::size_of;
use std::str::FromStr;

struct Operator {
    _kind: usize,
    packets: Vec<Packet>,
}

enum Payload {
    Literal(usize),
    Operator(Operator),
}

impl Packet {
    fn sum_versions(&self) -> usize {
        self.version
            + match &self.payload {
                Payload::Literal(_) => 0,
                Payload::Operator(operator) => {
                    operator.packets.iter().map(|p| p.sum_versions()).sum()
                }
            }
    }
}

#[derive(Debug, Clone)]
enum ParsePacketError {
    MissingVersion,
    MissingType,
    MissingLiteral,
    MissingLengthType,
    MissingLength,
    InconsistentSubpacketLength,
}

struct Packet {
    version: usize,
    payload: Payload,
}

impl Packet {
    fn from_stream(stream: &mut BitStream) -> Result<Self, ParsePacketError> {
        let version = stream.read_n(3).ok_or(ParsePacketError::MissingVersion)?;
        let packet_type = stream.read_n(3).ok_or(ParsePacketError::MissingType)?;
        let payload = match packet_type {
            4 => {
                let value = stream
                    .read_literal()
                    .ok_or(ParsePacketError::MissingLiteral)?;
                Payload::Literal(value)
            }
            kind => {
                let packets = Packet::from_substream(stream)?;
                Payload::Operator(Operator {
                    _kind: kind,
                    packets,
                })
            }
        };
        Ok(Packet { version, payload })
    }

    fn from_substream(stream: &mut BitStream) -> Result<Vec<Self>, ParsePacketError> {
        let len_type = stream
            .read_n(1)
            .ok_or(ParsePacketError::MissingLengthType)?;
        let mut packets = Vec::new();
        if len_type == 0 {
            let num_bits = stream.read_n(15).ok_or(ParsePacketError::MissingLength)?;
            let expected_len = stream.len() - num_bits;
            while stream.len() > expected_len {
                packets.push(Packet::from_stream(stream)?);
            }
            if stream.len() != expected_len {
                return Err(ParsePacketError::InconsistentSubpacketLength);
            }
        } else {
            let num_packets = stream.read_n(11).ok_or(ParsePacketError::MissingLength)?;
            for _ in 0..num_packets {
                packets.push(Packet::from_stream(stream)?);
            }
        }
        Ok(packets)
    }
}

#[derive(Debug, Clone)]
enum ParseBitsError {
    BadDigit(char),
}

struct BitStream {
    bits: Vec<u8>,
    byte_offset: usize,
    bit_offset: usize,
}

impl BitStream {
    fn new(bits: Vec<u8>) -> Self {
        BitStream {
            bits,
            byte_offset: 0,
            bit_offset: 0,
        }
    }

    fn len(&self) -> usize {
        8 * (self.bits.len() - self.byte_offset) - self.bit_offset
    }

    fn read_n(&mut self, n: usize) -> Option<usize> {
        if self.byte_offset >= self.bits.len() {
            return None;
        } else if n > size_of::<usize>() * 8 {
            panic!("Cannot handle {} bits in a single read", n);
        }
        let mut bits_left = n;

        // Handle bit offset at head
        let head = self.bits[self.byte_offset] as usize;
        let head_bits = 8 - self.bit_offset;
        let mut val = head & ((1 << head_bits) - 1);
        if head_bits > bits_left {
            self.bit_offset += bits_left;
            val >>= head_bits - bits_left;
            return Some(val);
        }
        bits_left -= head_bits;
        self.bit_offset = 0;
        self.byte_offset += 1;

        // Read whole bytes
        while bits_left >= 8 && self.byte_offset < self.bits.len() {
            val = (val << 8) | (self.bits[self.byte_offset] as usize);
            self.byte_offset += 1;
            bits_left -= 8;
        }

        // Read partial-byte tail
        if bits_left > 0 {
            if self.byte_offset >= self.bits.len() {
                return None;
            }
            let tail = self.bits[self.byte_offset] as usize;
            val = (val << bits_left) | (tail >> (8 - bits_left));
            self.bit_offset += bits_left;
        }
        Some(val)
    }

    fn read_literal(&mut self) -> Option<usize> {
        let mut val = 0;
        while let Some(n) = self.read_n(5) {
            val = (val << 4) | (n & 0b1111);
            if (n & 0b10000) == 0 {
                return Some(val);
            }
        }
        None
    }
}

impl FromStr for BitStream {
    type Err = ParseBitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ds = s
            .chars()
            .map(|c| c.to_digit(16).ok_or_else(|| Self::Err::BadDigit(c)))
            .collect::<Result<Vec<u32>, Self::Err>>()?;
        let bits = ds.chunks(2).map(|d| {
            let x = if d.len() == 1 { d[0] } else { d[0] << 4 | d[1] };
            x as u8
        });
        Ok(BitStream::new(bits.collect()))
    }
}

fn main() {
    let input = app::read_line(&app::input_arg());
    let mut stream: BitStream = input.parse().unwrap();
    let packet = Packet::from_stream(&mut stream).unwrap();
    let vs = packet.sum_versions();
    println!("Sum of versions: {}", vs);
}
