use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Packet {
    version: usize,
    packet_type: PacketType,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn total_version(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(|p| p.total_version())
                .sum::<usize>()
    }

    fn value(&self) -> usize {
        match self.packet_type {
            PacketType::Literal(val) => val,
            PacketType::Sum => self.sub_packets.iter().map(|p| p.value()).sum::<usize>(),
            PacketType::Product => self
                .sub_packets
                .iter()
                .map(|p| p.value())
                .product::<usize>(),
            PacketType::Minimum => self.sub_packets.iter().map(|p| p.value()).min().unwrap(),
            PacketType::Maximum => self.sub_packets.iter().map(|p| p.value()).max().unwrap(),
            PacketType::Greater => {
                match self.sub_packets[0]
                    .value()
                    .cmp(&self.sub_packets[1].value())
                {
                    std::cmp::Ordering::Greater => 1,
                    _ => 0,
                }
            }
            PacketType::Less => {
                match self.sub_packets[0]
                    .value()
                    .cmp(&self.sub_packets[1].value())
                {
                    std::cmp::Ordering::Less => 1,
                    _ => 0,
                }
            }
            PacketType::Equal => {
                match self.sub_packets[0]
                    .value()
                    .cmp(&self.sub_packets[1].value())
                {
                    std::cmp::Ordering::Equal => 1,
                    _ => 0,
                }
            }
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

#[derive(Debug)]
enum PacketLength {
    Bits(usize),
    Packets(usize),
    None,
}

fn bits_to_int(bits: &[char]) -> usize {
    bits.iter().rev().enumerate().fold(
        0,
        |acc, (i, b)| if *b == '1' { acc + (1 << i) } else { acc },
    )
}

fn parse_literal_data(bits: &[char]) -> (usize, usize) {
    let mut it = bits.iter();
    let mut literal_bits: Vec<char> = vec![];
    let mut literal_bits_read: usize = 0;
    let mut c: &char = it.next().unwrap();
    loop {
        if *c == '1' {
            literal_bits.push(*it.next().unwrap());
            literal_bits.push(*it.next().unwrap());
            literal_bits.push(*it.next().unwrap());
            literal_bits.push(*it.next().unwrap());

            literal_bits_read += 5;
            c = it.next().unwrap();
        } else {
            literal_bits_read += 1;
            let last_bits_count = std::cmp::min(4, bits.len() - literal_bits_read);
            for _ in 0..last_bits_count {
                literal_bits.push(*it.next().unwrap());
                literal_bits_read += 1;
            }
            literal_bits.append(&mut vec!['0'; 4 - last_bits_count]);

            return (bits_to_int(&literal_bits), literal_bits_read);
        }
    }
}

fn parse_packet(bits: &[char]) -> (Packet, usize) {
    let version = bits_to_int(&bits[0..3]);
    let mut packet_type = match bits_to_int(&bits[3..6]) {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        4 => PacketType::Literal(0),
        5 => PacketType::Greater,
        6 => PacketType::Less,
        7 => PacketType::Equal,
        _ => unreachable!(),
    };
    let mut bits_read: usize = 6;
    let length = match packet_type {
        PacketType::Literal(_) => PacketLength::None,
        _ => {
            if bits[6] == '0' {
                bits_read += 16;
                PacketLength::Bits(bits_to_int(&bits[7..22]))
            } else {
                bits_read += 12;
                PacketLength::Packets(bits_to_int(&bits[7..18]))
            }
        }
    };

    let mut sub_packets: Vec<Packet> = vec![];

    match packet_type {
        PacketType::Literal(_) => {
            let (value, read) = parse_literal_data(&bits[6..]);
            packet_type = PacketType::Literal(value);
            bits_read += read;
        }
        _ => match length {
            PacketLength::Bits(num_bits) => {
                let mut next_start: usize = 22;

                while next_start < 22 + num_bits {
                    let (next_packet, next_read) = parse_packet(&bits[next_start..]);
                    next_start += next_read;
                    bits_read += next_read;
                    sub_packets.push(next_packet);
                }
            }
            PacketLength::Packets(num_packets) => {
                let mut next_start: usize = 18;
                for _ in 0..num_packets {
                    let (next_packet, next_read) = parse_packet(&bits[next_start..]);
                    next_start += next_read;
                    bits_read += next_read;
                    sub_packets.push(next_packet);
                }
            }
            PacketLength::None => unreachable!(),
        },
    }

    (
        Packet {
            version,
            packet_type,
            sub_packets,
        },
        bits_read,
    )
}

fn part1() -> usize {
    const INPUT_FILE: &str = "day16/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let line = io::BufReader::new(file).lines().flatten().next().unwrap();

    let mut bits_map: HashMap<char, [char; 4]> = HashMap::new();
    bits_map.insert('0', ['0', '0', '0', '0']);
    bits_map.insert('1', ['0', '0', '0', '1']);
    bits_map.insert('2', ['0', '0', '1', '0']);
    bits_map.insert('3', ['0', '0', '1', '1']);
    bits_map.insert('4', ['0', '1', '0', '0']);
    bits_map.insert('5', ['0', '1', '0', '1']);
    bits_map.insert('6', ['0', '1', '1', '0']);
    bits_map.insert('7', ['0', '1', '1', '1']);
    bits_map.insert('8', ['1', '0', '0', '0']);
    bits_map.insert('9', ['1', '0', '0', '1']);
    bits_map.insert('A', ['1', '0', '1', '0']);
    bits_map.insert('B', ['1', '0', '1', '1']);
    bits_map.insert('C', ['1', '1', '0', '0']);
    bits_map.insert('D', ['1', '1', '0', '1']);
    bits_map.insert('E', ['1', '1', '1', '0']);
    bits_map.insert('F', ['1', '1', '1', '1']);

    let bits: Vec<char> = line
        .chars()
        .flat_map(|c| bits_map.get(&c).unwrap())
        .cloned()
        .collect();

    let (first_packet, _) = parse_packet(&bits);

    first_packet.total_version()
}

fn part2() -> usize {
    const INPUT_FILE: &str = "day16/input.txt";
    let file = File::open(INPUT_FILE).unwrap();
    let line = io::BufReader::new(file).lines().flatten().next().unwrap();

    let mut bits_map: HashMap<char, [char; 4]> = HashMap::new();
    bits_map.insert('0', ['0', '0', '0', '0']);
    bits_map.insert('1', ['0', '0', '0', '1']);
    bits_map.insert('2', ['0', '0', '1', '0']);
    bits_map.insert('3', ['0', '0', '1', '1']);
    bits_map.insert('4', ['0', '1', '0', '0']);
    bits_map.insert('5', ['0', '1', '0', '1']);
    bits_map.insert('6', ['0', '1', '1', '0']);
    bits_map.insert('7', ['0', '1', '1', '1']);
    bits_map.insert('8', ['1', '0', '0', '0']);
    bits_map.insert('9', ['1', '0', '0', '1']);
    bits_map.insert('A', ['1', '0', '1', '0']);
    bits_map.insert('B', ['1', '0', '1', '1']);
    bits_map.insert('C', ['1', '1', '0', '0']);
    bits_map.insert('D', ['1', '1', '0', '1']);
    bits_map.insert('E', ['1', '1', '1', '0']);
    bits_map.insert('F', ['1', '1', '1', '1']);

    let bits: Vec<char> = line
        .chars()
        .flat_map(|c| bits_map.get(&c).unwrap())
        .cloned()
        .collect();

    let (first_packet, _) = parse_packet(&bits);

    first_packet.value()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
