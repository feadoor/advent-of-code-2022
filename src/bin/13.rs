use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        use Packet::*;
        match (self, other) {
            (Number(a), Number(b)) => a.partial_cmp(b),
            (Number(a), List(_b))  => List(vec![Number(*a)]).partial_cmp(other),
            (List(_a), Number(b))  => self.partial_cmp(&List(vec![Number(*b)])),
            (List(a), List(b))     => a.partial_cmp(b),
        }
    }
}

// BEGIN NOM PARSING CODE

fn number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn list(input: &str) -> IResult<&str, Vec<Packet>> {
    delimited(tag("["), separated_list0(tag(","), packet), tag("]"))(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(number, Packet::Number),
        map(list,   Packet::List),
    ))(input)
}

// END NOM PARSING CODE

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/13.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_packet(line: &str) -> Packet {
    packet(line).expect("failed to parse packet").1
}

fn main() {
    let packets: Vec<_> = read_lines().filter(|l| !l.is_empty()).map(|l| parse_packet(&l)).collect();

    let ordered_indices = packets.iter().tuples().enumerate()
        .filter(|(_, (p1, p2))| p1.le(p2))
        .map(|(idx, _)| idx + 1);
    println!("Part 1: {}", ordered_indices.sum::<usize>());

    let dividers = (parse_packet("[[2]]"), parse_packet("[[6]]"));
    let before_first = packets.iter().filter(|&p| p.lt(&dividers.0)).count();
    let before_second = packets.iter().filter(|&p| p.lt(&dividers.1)).count();
    println!("Part 2: {}", (before_first + 1) * (before_second + 2));
}
