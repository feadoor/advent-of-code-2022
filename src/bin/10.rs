use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;

enum Instruction {
    Addx(isize),
    Noop
}

impl Instruction {

    fn cycles(&self) -> usize {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop    => 1,
        }
    }

    fn apply_to(&self, state: &mut isize) {
        match self {
            Instruction::Addx(amount) => *state += amount,
            Instruction::Noop         => {},
        }
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/10.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_line(line: &str) -> Instruction {
    match &line[0..4] {
        "addx" => Instruction::Addx(str::parse(&line[5..]).expect("could not parse integer")),
        "noop" => Instruction::Noop,
        _ => panic!("unexpected instruction"),
    }
}

fn cycle_values(program: &[Instruction]) -> impl Iterator<Item = isize> + '_ {
    program.iter().scan(1, |state, instr| {
        let values = repeat(*state).take(instr.cycles());
        instr.apply_to(state);
        Some(values)
    }).flatten()
}

fn crt_line<I: Iterator<Item = isize>>(sprite_positions: &mut I) -> Option<String> {
    let output: String = sprite_positions.take(40).enumerate().map(|(idx, sprite_position)| {
        if (sprite_position - (idx as isize)).abs() <= 1 { '#' } else { ' ' }
    }).collect();
    if output.is_empty() { None } else { Some(output) }
}

fn main() {
    let program: Vec<_> = read_lines().map(|l| parse_line(&l)).collect();

    let signal_strengths = cycle_values(&program).enumerate().map(|(idx, value)| ((idx + 1) as isize) * value);
    println!("Part 1: {}", signal_strengths.skip(19).step_by(40).take(6).sum::<isize>());

    let mut sprite_positions = cycle_values(&program);
    println!("Part 2:");
    while let Some(line) = crt_line(&mut sprite_positions) {
        println!("{}", line);
    }
}
