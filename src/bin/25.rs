use itertools::{iterate, Itertools};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/25.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_char(c: char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _   => panic!("unexpected char {c}"),
    }
}

fn parse_line(l: &str) -> Vec<isize> {
    l.chars().map(parse_char).collect()
}

fn snafu_to_decimal(snafu: &[isize]) -> isize {
    iterate(1, |x| x * 5).zip(snafu.iter().rev()).map(|(pv, d)| pv * d).sum()
}

fn decimal_to_quinary(mut decimal: isize) -> Vec<isize> {
    let mut digits = Vec::new();
    while decimal > 0 { digits.push(decimal % 5); decimal /= 5; }
    digits
}

fn quinary_to_snafu(mut quinary: Vec<isize>) -> Vec<isize> {
    for idx in 0..quinary.len() {
        if idx == quinary.len() - 1 && quinary[idx] > 2 { quinary.push(0); }
        if quinary[idx] == 3 { quinary[idx] = -2; quinary[idx + 1] += 1; }
        if quinary[idx] == 4 { quinary[idx] = -1; quinary[idx + 1] += 1; }
        if quinary[idx] == 5 { quinary[idx] = 0;  quinary[idx + 1] += 1; }
    }
    quinary.reverse(); quinary
}

fn display_snafu(snafu: &[isize]) -> String {
    let chars = ['=', '-', '0', '1', '2'];
    snafu.iter().map(|d| chars[(d + 2) as usize]).join("")
}

fn main() {
    let input_digits = read_lines().map(|l| parse_line(&l));

    let total_input: isize = input_digits.map(|s| snafu_to_decimal(&s)).sum();
    let converted = quinary_to_snafu(decimal_to_quinary(total_input));
    println!("Part 1: {}", display_snafu(&converted));

    println!("Part 2: Merry Christmas!");
}
