use itertools::Itertools;

use std::fs::File;
use std::io::{BufRead, BufReader};

struct Range {
    lo: usize,
    hi: usize,
}

impl Range {

    fn fully_contains(&self, other: &Range) -> bool {
        other.lo >= self.lo && other.hi <= self.hi
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/04.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_range(range: &str) -> Range {
    let (lo, hi) = range.split("-")
        .map(|s| s.parse().expect("could not parse as usize"))
        .collect_tuple().expect("wrong number of elements in a range");
    Range { lo, hi }
}

fn parse_line(line: &str) -> (Range, Range) {
    line.split(",").map(parse_range).collect_tuple().expect("wrong number of ranges in a line")
}

fn main() {
    let range_pairs: Vec<_> = read_lines().map(|line| parse_line(&line)).collect();

    let number_of_redundancies = range_pairs.iter().filter(|(r1, r2)| r1.fully_contains(r2) || r2.fully_contains(r1)).count();
    println!("Part 1: {}", number_of_redundancies);

    let number_of_overlaps = range_pairs.iter().filter(|(r1, r2)| r1.overlaps(r2)).count();
    println!("Part 2: {}", number_of_overlaps);
}
