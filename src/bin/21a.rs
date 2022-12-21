use itertools::Itertools;

use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub, Mul};

enum Op {
    Add,
    Subtract,
    Multiply,
}

enum Monkey {
    Value(Polynomial),
    Expr(String, String, Op),
}

#[derive(Clone)]
struct Polynomial {
    coeffs: Vec<isize>,
}

impl Polynomial {

    fn constant(c: isize) -> Self {
        Self { coeffs: vec![c] }
    }

    fn identity() -> Self {
        Self { coeffs: vec![0, 1] }
    }

    fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    fn coeff_at(&self, idx: usize) -> isize {
        if idx <= self.degree() { self.coeffs[idx] } else { 0 }
    }

    fn normalise(&mut self) {
        while let Some(&0) = self.coeffs.last() {
            if self.coeffs.len() == 1 { return; }
            self.coeffs.pop();
        }
    }

    fn sum_of_roots(&self) -> (isize, isize) {
        if self.degree() < 1 { (0, 1) } else {
            (-self.coeffs[self.degree() - 1], self.coeffs[self.degree()])
        }
    }
}

impl Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, other: &Polynomial) -> Polynomial {
        let out_degree = max(self.degree(), other.degree());
        let mut result = Polynomial { coeffs: (0..=out_degree).map(|idx| self.coeff_at(idx).checked_add(other.coeff_at(idx)).unwrap()).collect() };
        result.normalise(); result
    }
}

impl Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, other: &Polynomial) -> Polynomial {
        let out_degree = max(self.degree(), other.degree());
        let mut result = Polynomial { coeffs: (0..=out_degree).map(|idx| self.coeff_at(idx).checked_sub(other.coeff_at(idx)).unwrap()).collect() };
        result.normalise(); result
    }
}

impl Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, other: &Polynomial) -> Polynomial {
        let out_degree = self.degree() + other.degree();
        let coeffs = (0..=out_degree).map(|deg| (0..=deg).map(|idx| self.coeff_at(idx).checked_mul(other.coeff_at(deg - idx)).unwrap()).sum()).collect();
        let mut result = Polynomial { coeffs };
        result.normalise(); result
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/21a.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_line(line: &str) -> (String, Monkey) {
    let (name, value) = line.split(": ").collect_tuple().expect("wrong number of items on line");
    if let Ok(v) = value.parse() {
        (name.to_string(), Monkey::Value(Polynomial::constant(v)))
    } else {
        let (left, op, right) = value.split(" ").collect_tuple().expect("wrong number of items in RHS");
        let op = match op {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            _   => panic!("unexpected operation {op}"),
        };
        (name.to_string(), Monkey::Expr(left.to_string(), right.to_string(), op))
    }
}

fn evaluate_monkey(name: &str, monkeys: &HashMap<String, Monkey>) -> Polynomial {
    if !monkeys.contains_key(name) { println!("Missing {}", name); }
    match &monkeys[name] {
        Monkey::Value(v) => v.clone(),
        Monkey::Expr(left, right, op) => match op {
            Op::Add      => &evaluate_monkey(left, monkeys) + &evaluate_monkey(right, monkeys),
            Op::Subtract => &evaluate_monkey(left, monkeys) - &evaluate_monkey(right, monkeys),
            Op::Multiply => &evaluate_monkey(left, monkeys) * &evaluate_monkey(right, monkeys),
        }
    }
}

fn main() {
    let mut monkeys: HashMap<_, _> = read_lines().map(|l| parse_line(&l)).collect();
    monkeys.insert("humn".to_string(), Monkey::Value(Polynomial::identity()));
    let Monkey::Expr(left, right, _) = &monkeys["root"] else { panic!() };
    let root_poly = &evaluate_monkey(&left.clone(), &monkeys) - &evaluate_monkey(&right.clone(), &monkeys);
    let (numer, denom) = root_poly.sum_of_roots();
    if numer % denom == 0 { println!("{}", numer / denom); } else { println!("{} / {}", numer, denom); }
}
