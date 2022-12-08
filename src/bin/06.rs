use itertools::Itertools;

use std::fs;

fn read_data() -> String {
    fs::read_to_string("inputs/06.txt").expect("unable to read input file")
}

fn characters_until_n_distinct(data: &str, n: usize) -> usize {
    data.chars().collect_vec().windows(n).position(|s| s.iter().all_unique()).expect("not found") + n
}

fn main() {
    let data = read_data();
    println!("Part 1: {}", characters_until_n_distinct(&data, 4));
    println!("Part 2: {}", characters_until_n_distinct(&data, 14));
}
