use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum PacketValue {
    Literal(usize),
    Sub(Packet),
}

impl PacketValue {
    fn get_value(&self) -> usize {
        match self {
            PacketValue::Literal(v) => *v,
            PacketValue::Sub(sub) => sub.get_value(),
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    id: u8,
    fields: Vec<PacketValue>,
}

impl Packet {
    fn get_version_number(&self) -> usize {
        self.version as usize
            + self.fields.iter().fold(0, |sum, f| {
                sum + match f {
                    PacketValue::Sub(sub) => sub.get_version_number(),
                    _ => 0,
                }
            })
    }

    fn get_value(&self) -> usize {
        match self.id {
            0 | 4 => {
                // sum all fields
                self.fields.iter().fold(0, |sum, f| sum + f.get_value())
            }

            1 => {
                // multiply all fields
                self.fields.iter().fold(1, |sum, f| sum * f.get_value())
            }

            2 => {
                // minimum field
                self.fields.iter().map(|f| f.get_value()).min().unwrap()
            }

            3 => {
                // maximum field
                self.fields.iter().map(|f| f.get_value()).max().unwrap()
            }

            5 => {
                // greater than
                if self.fields[0].get_value() > self.fields[1].get_value() {
                    1
                } else {
                    0
                }
            }

            6 => {
                // less than
                if self.fields[0].get_value() < self.fields[1].get_value() {
                    1
                } else {
                    0
                }
            }

            7 => {
                // equal to
                if self.fields[0].get_value() == self.fields[1].get_value() {
                    1
                } else {
                    0
                }
            }

            _ => panic!("unhandled id"),
        }
    }
}

struct ParseState<'a> {
    chars: &'a Vec<u8>,
    index: usize,
    bit: usize,
}

impl ParseState<'_> {
    fn total_bits_read(&self) -> usize {
        self.index * 4 + self.bit
    }

    fn read_bits(&mut self, num_bits: usize) -> usize {
        let mut result: usize = 0;
        let mut num_bits = num_bits;
        while num_bits > 0 && self.index < self.chars.len() {
            let length = if num_bits > (4 - self.bit) {
                4 - self.bit
            } else {
                num_bits
            };
            num_bits = num_bits.saturating_sub(length);

            result = result << length;

            let offset = 4 - (self.bit + length);
            let c = (self.chars[self.index] >> offset) as usize;
            let mask = (1 << length) - 1;
            result |= c & mask;

            self.bit += length;
            if self.bit == 4 {
                self.bit = 0;
                self.index += 1;
            }
        }
        result
    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.read_bits(3) as u8;
        let id = self.read_bits(3) as u8;
        let mut fields: Vec<PacketValue> = Vec::new();
        match id {
            4 => {
                // literal value
                let mut result: usize = 0;
                loop {
                    let is_last = self.read_bits(1);
                    result = (result << 4) | self.read_bits(4);
                    if is_last == 0 {
                        break;
                    }
                }
                fields.push(PacketValue::Literal(result));
            }
            _ => {
                // operator packet
                let package_type = self.read_bits(1);
                match package_type {
                    0 => {
                        // x bits
                        let num_bits = self.read_bits(15);
                        let current_bit = self.total_bits_read();
                        while self.total_bits_read() < current_bit + num_bits {
                            fields.push(PacketValue::Sub(self.parse_packet()));
                        }
                        assert_eq!(self.total_bits_read(), current_bit + num_bits);
                    }
                    1 => {
                        // num sub packets
                        let num_sub_packets = self.read_bits(11);
                        for _ in 0..num_sub_packets {
                            fields.push(PacketValue::Sub(self.parse_packet()));
                        }
                    }
                    _ => panic!("unhandled package_type {}", package_type),
                }
            }
        }

        Packet {
            version: version,
            id: id,
            fields: fields,
        }
    }

    fn is_finished(&self) -> bool {
        self.index >= self.chars.len()
    }
}

#[aoc_generator(day16)]
fn day16_input(s: &str) -> Vec<Packet> {
    let mut parse_state = ParseState {
        chars: &s
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect(),
        index: 0,
        bit: 0,
    };

    let mut packets: Vec<Packet> = Vec::new();
    while !parse_state.is_finished() {
        packets.push(parse_state.parse_packet());
    }
    packets
}

#[aoc(day16, part1)]
fn day16_part1(input: &[Packet]) -> usize {
    input.iter().fold(0, |sum, p| sum + p.get_version_number())
}

#[aoc(day16, part2)]
fn day16_part2(input: &[Packet]) -> usize {
    input.iter().fold(0, |sum, p| sum + p.get_value())
}
