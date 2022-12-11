use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult, Parser, error::ParseError,
};

use std::fs;
use std::collections::VecDeque;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn apply_to(&self, item: usize) -> usize {
        match self {
            Operation::Add(amount)      => item + amount,
            Operation::Multiply(amount) => item * amount,
            Operation::Square           => item * item,
        }
    }
}

#[derive(Clone)]
struct Test {
    modulus: usize,
    true_target: usize,
    false_target: usize,
}

impl Test {
    fn get_target(&self, item: usize) -> usize {
        if item % self.modulus == 0 { self.true_target } else { self.false_target }
    }
}

enum WorryManager {
    DivideBy(usize),
    ModBy(usize),
}

impl WorryManager {
    fn apply_to(&self, item: usize) -> usize {
        match self {
            WorryManager::DivideBy(amount) => item / amount,
            WorryManager::ModBy(amount)    => item % amount,
        }
    }
}

// BEGIN NOM PARSING CODE

fn on_line<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    delimited(space0, f, opt(line_ending))
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn heading(input: &str) -> IResult<&str, ()> {
    on_line(value((), tuple((tag("Monkey "), number, tag(":")))))(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    on_line(preceded(tag("Starting items: "), separated_list0(tag(", "), number)))(input)
}

fn add_operation(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("new = old + "), number), Operation::Add)(input)
}

fn multiply_operation(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("new = old * "), number), Operation::Multiply)(input)
}

fn square_operation(input: &str) -> IResult<&str, Operation> {
    value(Operation::Square, tag("new = old * old"))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    on_line(preceded(tag("Operation: "), alt((add_operation, multiply_operation, square_operation))))(input)
}

fn test_modulus(input: &str) -> IResult<&str, usize> {
    on_line(preceded(tag("Test: divisible by "), number))(input)
}

fn true_target(input: &str) -> IResult<&str, usize> {
    on_line(preceded(tag("If true: throw to monkey "), number))(input)
}

fn false_target(input: &str) -> IResult<&str, usize> {
    on_line(preceded(tag("If false: throw to monkey "), number))(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    map(preceded(heading, 
        tuple((starting_items, operation, test_modulus, true_target, false_target))
    ), |(items, operation, test_modulus, true_target, false_target)| Monkey { 
        items: VecDeque::from(items), 
        operation, 
        test: Test { modulus: test_modulus, true_target, false_target },
    })(input)
}

fn parse(input: &str) -> Vec<Monkey> {
    many0(on_line(monkey))(input).expect("failed to parse").1
}

// END NOM PARSING CODE

fn read_data() -> String {
    fs::read_to_string("inputs/11.txt").expect("unable to read input file")
}

fn take_turn(monkeys: &mut [Monkey], index: usize, worry_manager: &WorryManager) {
    while let Some(mut item) = monkeys[index].items.pop_front() {
        item = worry_manager.apply_to(monkeys[index].operation.apply_to(item));
        let target = monkeys[index].test.get_target(item);
        monkeys[target].items.push_back(item);
    }
}

fn modulus(monkeys: &[Monkey]) -> usize {
    monkeys.iter().map(|monkey| monkey.test.modulus).product()
}

fn inspection_counts(monkeys: &mut [Monkey], rounds: usize, worry_manager: &WorryManager) -> Vec<usize> {
    let mut inspection_counts = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            inspection_counts[index] += monkeys[index].items.len();
            take_turn(monkeys, index, worry_manager);
        }
    }
    inspection_counts
}

fn main() {
    let starting_monkeys = parse(&read_data());

    let mut monkeys = starting_monkeys.clone();
    let inspection_counts_part_1 = inspection_counts(&mut monkeys, 20, &WorryManager::DivideBy(3));
    let monkey_business: usize = inspection_counts_part_1.iter().sorted().rev().take(2).product();
    println!("Part 1: {}", monkey_business);

    let mut monkeys = starting_monkeys.clone();
    let modulus = modulus(&monkeys);
    let inspection_counts_part_2 = inspection_counts(&mut monkeys, 10_000, &WorryManager::ModBy(modulus));
    let monkey_business: usize = inspection_counts_part_2.iter().sorted().rev().take(2).product();
    println!("Part 2: {}", monkey_business);
}
