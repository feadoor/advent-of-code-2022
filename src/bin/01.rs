use itertools::Itertools;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/01.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn totals_grouped_by_empty_lines<I: Iterator<Item = String>>(lines: I) -> Vec<usize> {
    lines.group_by(|line| line.is_empty()).into_iter().filter_map(|(k, g)| if k { None } else { Some(g) })
        .map(|group| group.map(|line| line.parse::<usize>().expect("could not parse line as usize")))
        .map(|group| group.sum())
        .collect()
}

fn sum_of_top_k_values(values: &[usize], k: usize) -> usize {
    values.iter().sorted().rev().take(k).sum()
}

fn main() {
    let calorie_totals = totals_grouped_by_empty_lines(read_lines());
    println!("Part 1: {}", sum_of_top_k_values(&calorie_totals, 1));
    println!("Part 2: {}", sum_of_top_k_values(&calorie_totals, 3));
}
