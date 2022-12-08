use itertools::iproduct;
use take_until::TakeUntilExt;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/08.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_input<I: Iterator<Item = String>>(lines: I) -> Vec<Vec<usize>> {
    lines.map(|line| line.chars().map(|c| c.to_digit(10).expect("not a digit") as usize).collect()).collect()
}

fn is_visible(row: usize, col: usize, trees: &[Vec<usize>]) -> bool {
    let visible_from_left = (0..col).map(|c| trees[row][c]).all(|t| t < trees[row][col]);
    let visible_from_right = (col + 1..trees[row].len()).map(|c| trees[row][c]).all(|t| t < trees[row][col]);
    let visible_from_above = (0..row).map(|r| trees[r][col]).all(|t| t < trees[row][col]);
    let visible_from_below = (row + 1..trees.len()).map(|r| trees[r][col]).all(|t| t < trees[row][col]);

    visible_from_left || visible_from_right || visible_from_above || visible_from_below
}

fn visible_count(trees: &[Vec<usize>]) -> usize {
    iproduct!(0..trees.len(), 0..trees[0].len()).filter(|&(r, c)| is_visible(r, c, trees)).count()
}

fn scenic_score(row: usize, col: usize, trees: &[Vec<usize>]) -> usize {
    let score_left = (0..col).rev().map(|c| trees[row][c]).take_until(|&t| t >= trees[row][col]).count();
    let score_right = (col + 1..trees[row].len()).map(|c| trees[row][c]).take_until(|&t| t >= trees[row][col]).count();
    let score_above = (0..row).rev().map(|r| trees[r][col]).take_until(|&t| t >= trees[row][col]).count();
    let score_below = (row + 1..trees.len()).map(|r| trees[r][col]).take_until(|&t| t >= trees[row][col]).count();

    score_left * score_right * score_above * score_below
}

fn maximum_scenic_score(trees: &[Vec<usize>]) -> usize {
    iproduct!(0..trees.len(), 0..trees[0].len()).map(|(r, c)| scenic_score(r, c, trees)).max().unwrap()
}

fn main() {
    let trees = parse_input(read_lines());

    println!("Part 1: {}", visible_count(&trees));
    println!("Part 2: {}", maximum_scenic_score(&trees));
}
