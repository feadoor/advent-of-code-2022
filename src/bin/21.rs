use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Monkey {
    Value(usize),
    Expr(String, String, Op),
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/21.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_line(line: &str) -> (String, Monkey) {
    let (name, value) = line.split(": ").collect_tuple().expect("wrong number of items on line");
    if let Ok(v) = value.parse() {
        (name.to_string(), Monkey::Value(v))
    } else {
        let (left, op, right) = value.split(" ").collect_tuple().expect("wrong number of items in RHS");
        let op = match op {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            _   => panic!("unexpected operation {op}"),
        };
        (name.to_string(), Monkey::Expr(left.to_string(), right.to_string(), op))
    }
}

fn evaluate_monkey(name: &str, monkeys: &HashMap<String, Monkey>) -> usize {
    match &monkeys[name] {
        Monkey::Value(v) => *v,
        Monkey::Expr(left, right, op) => match op {
            Op::Add      => evaluate_monkey(left, monkeys) + evaluate_monkey(right, monkeys),
            Op::Subtract => evaluate_monkey(left, monkeys) - evaluate_monkey(right, monkeys),
            Op::Multiply => evaluate_monkey(left, monkeys) * evaluate_monkey(right, monkeys),
            Op::Divide   => evaluate_monkey(left, monkeys) / evaluate_monkey(right, monkeys),
        }
    }
}

fn comparison_result_with_initial_number(humn: usize, left: &str, right: &str, monkeys: &mut HashMap<String, Monkey>) -> Ordering {
    monkeys.insert("humn".to_string(), Monkey::Value(humn));
    evaluate_monkey(left, monkeys).cmp(&evaluate_monkey(right, monkeys))
}

fn binary_search_for_equality(left: &str, right: &str, monkeys: &mut HashMap<String, Monkey>) -> usize {
    let mut humn = 1; let initial_result = comparison_result_with_initial_number(humn, left, right, monkeys);
    while comparison_result_with_initial_number(humn, left, right, monkeys) == initial_result { humn *= 2; }
    let (mut lo, mut hi) = (humn / 2, humn); loop {
        let mid = (lo + hi) / 2;
        match comparison_result_with_initial_number(mid, left, right, monkeys) {
            Ordering::Equal          => return mid,
            x if x == initial_result => (lo, hi) = (mid + 1, hi),
            _                        => (lo, hi) = (lo, mid),
        }
    }
}

fn main() {
    let mut monkeys: HashMap<_, _> = read_lines().map(|l| parse_line(&l)).collect();
    println!("Part 1: {}", evaluate_monkey("root", &monkeys));

    let Monkey::Expr(left, right, _) = &monkeys["root"] else { panic!() };
    println!("Part 2: {}", binary_search_for_equality(&left.clone(), &right.clone(), &mut monkeys));
}
