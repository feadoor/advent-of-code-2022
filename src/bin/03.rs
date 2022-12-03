use itertools::Itertools;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/03.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn split_in_half<'a>(s: &'a str) -> (&'a str, &'a str) {
    let halfway_point = s.len() / 2;
    (&s[0..halfway_point], &s[halfway_point..])
}

fn find_common_bytes<'a>(s1: &'a str, s2: &'a str) -> impl Iterator<Item = u8> + 'a {
    let byte_set1: HashSet<_> = s1.bytes().collect();
    let byte_set2: HashSet<_> = s2.bytes().collect();
    (&byte_set1 & &byte_set2).into_iter()
}

fn find_common_bytes_iter<'a, I: Iterator<Item = &'a String>>(items: I) -> impl Iterator<Item = u8> {
    items.map(|item| item.bytes().collect::<HashSet<_>>())
        .reduce(|acc, set| &acc & &set)
        .expect("input iterator empty")
        .into_iter()
}

fn priority(b: u8) -> usize {
    match b {
        b'a'..=b'z' => (b - b'a' + 1) as usize,
        b'A'..=b'Z' => (b - b'A' + 27) as usize,
        _ => panic!("Unexpected byte value: {}", b),
    }
}

fn main() {
    let rucksacks: Vec<_> = read_lines().collect();

    let total_priority_part_1: usize = rucksacks.iter()
        .map(|r| split_in_half(&r))
        .flat_map(|(s1, s2)| find_common_bytes(s1, s2))
        .map(priority)
        .sum();
    println!("Part 1: {}", total_priority_part_1);

    let total_priority_part_2: usize = rucksacks.iter().chunks(3).into_iter()
        .flat_map(|chunk| find_common_bytes_iter(chunk.into_iter()))
        .map(priority)
        .sum();
    println!("Part 2: {}", total_priority_part_2);
}