use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct MoveInstruction {
    from: usize,
    to: usize,
    amount: usize,
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/05.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_crate(s: &str) -> Option<char> {
    match &s[0..1] {
        "[" => s[1..2].chars().next(),
        _ => None
    }
}

fn parse_crates<I: Iterator<Item = String>>(crate_lines: I) -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();

    for line in crate_lines {
        for idx in 0..=line.len() / 4 {
            if idx >= stacks.len() { stacks.push(VecDeque::new()); }
            if let Some(c) = parse_crate(&line[4 * idx .. 4 * idx + 3]) { stacks[idx].push_front(c); }
        }
    }

    stacks
}

fn parse_move_instruction(instruction: &str) -> MoveInstruction {
    lazy_static! {
        static ref INSTRUCTION_REGEX: Regex = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    }
    INSTRUCTION_REGEX.captures(instruction).map(|captures| MoveInstruction {
        from: captures["from"].parse().unwrap(),
        to: captures["to"].parse().unwrap(),
        amount: captures["amount"].parse().unwrap(),
    }).expect("failed to parse instruction")
}

fn parse_input<I: Iterator<Item = String>>(lines: I) -> (Vec<VecDeque<char>>, Vec<MoveInstruction>) {
    let line_groups = lines.group_by(|line| line.is_empty());
    let (crate_lines, instruction_lines) = line_groups.into_iter()
        .filter_map(|(k, g)| if k { None } else { Some(g)})
        .collect_tuple()
        .expect("too many empty lines in input");

    (parse_crates(crate_lines), instruction_lines.map(|s| parse_move_instruction(&s)).collect())
}

fn apply_instruction_part_1(crates: &mut[VecDeque<char>], instruction: &MoveInstruction) {
    for _ in 0..instruction.amount {
        let crate_to_move = crates[instruction.from - 1].pop_back().expect("tried to pop from empty stack");
        crates[instruction.to - 1].push_back(crate_to_move);
    }
}

fn apply_instruction_part_2(crates: &mut[VecDeque<char>], instruction: &MoveInstruction) {
    let mut moved_crates = crates[instruction.from - 1].split_off(crates[instruction.from - 1].len() - instruction.amount);
    crates[instruction.to - 1].append(&mut moved_crates);
}

fn main() {
    let (crates, instructions) = parse_input(read_lines());
    
    let mut crates_part_1 = crates.clone();
    for instruction in &instructions { apply_instruction_part_1(&mut crates_part_1, instruction); }
    println!("Part 1: {}", crates_part_1.iter().map(|stack| stack.back().expect("empty stack")).join(""));

    let mut crates_part_2 = crates.clone();
    for instruction in &instructions { apply_instruction_part_2(&mut crates_part_2, instruction); }
    println!("Part 2: {}", crates_part_2.iter().map(|stack| stack.back().expect("empty stack")).join(""));
}
