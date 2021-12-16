#![feature(test)]
mod bench;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use utils::AocSolution;

pub struct Solution {
    input_path: String,
}

impl AocSolution<usize, usize> for Solution {
    fn part1(&self) -> usize {
        part1(&self.input_path)
    }
    fn part2(&self) -> usize {
        part2(&self.input_path)
    }
    fn with_input_path(input_path: &str) -> Self {
        Solution {
            input_path: input_path.to_owned(),
        }
    }
}

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

    fn cmp_packets(&self) -> std::cmp::Ordering {
        self.sub_packets[0]
            .value()
            .cmp(&self.sub_packets[1].value())
    }

    fn value(&self) -> usize {
        match self.packet_type {
            PacketType::Literal(val) => val,
            PacketType::Sum => self.sub_packets.iter().map(Packet::value).sum(),
            PacketType::Product => self.sub_packets.iter().map(Packet::value).product(),
            PacketType::Minimum => self.sub_packets.iter().map(Packet::value).min().unwrap(),
            PacketType::Maximum => self.sub_packets.iter().map(Packet::value).max().unwrap(),
            PacketType::Greater => match self.cmp_packets() {
                std::cmp::Ordering::Greater => 1,
                _ => 0,
            },
            PacketType::Less => match self.cmp_packets() {
                std::cmp::Ordering::Less => 1,
                _ => 0,
            },
            PacketType::Equal => match self.cmp_packets() {
                std::cmp::Ordering::Equal => 1,
                _ => 0,
            },
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

    while let Some(c) = it.next() {
        literal_bits.push(*it.next().unwrap());
        literal_bits.push(*it.next().unwrap());
        literal_bits.push(*it.next().unwrap());
        literal_bits.push(*it.next().unwrap());

        literal_bits_read += 5;

        if *c == '0' {
            return (bits_to_int(&literal_bits), literal_bits_read);
        }
    }

    unreachable!();
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

fn build_bits_map() -> HashMap<char, [char; 4]> {
    HashMap::from_iter([
        ('0', ['0', '0', '0', '0']),
        ('1', ['0', '0', '0', '1']),
        ('2', ['0', '0', '1', '0']),
        ('3', ['0', '0', '1', '1']),
        ('4', ['0', '1', '0', '0']),
        ('5', ['0', '1', '0', '1']),
        ('6', ['0', '1', '1', '0']),
        ('7', ['0', '1', '1', '1']),
        ('8', ['1', '0', '0', '0']),
        ('9', ['1', '0', '0', '1']),
        ('A', ['1', '0', '1', '0']),
        ('B', ['1', '0', '1', '1']),
        ('C', ['1', '1', '0', '0']),
        ('D', ['1', '1', '0', '1']),
        ('E', ['1', '1', '1', '0']),
        ('F', ['1', '1', '1', '1']),
    ])
}

fn hex_to_binary(hex: &str) -> Vec<char> {
    let bits_map = build_bits_map();
    hex.chars()
        .flat_map(|c| bits_map.get(&c).unwrap())
        .cloned()
        .collect()
}

fn part1(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let line = io::BufReader::new(file).lines().flatten().next().unwrap();

    let bits = hex_to_binary(&line);

    let (first_packet, _) = parse_packet(&bits);

    first_packet.total_version()
}

fn part2(input_path: &str) -> usize {
    let file = File::open(input_path).unwrap();
    let line = io::BufReader::new(file).lines().flatten().next().unwrap();

    let bits = hex_to_binary(&line);

    let (first_packet, _) = parse_packet(&bits);

    first_packet.value()
}
