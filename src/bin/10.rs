use std::fs::File;
use std::io::{BufRead, BufReader};

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
}

#[derive(Clone, Copy)]
struct ProgramState {
    register: isize,
}

impl ProgramState {

    fn new() -> Self {
        Self { register: 1 }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Addx(amount) => self.register += amount,
            Instruction::Noop         => {},
        }
    }
}

struct Executor<'a> {
    state: ProgramState,
    program_counter: usize,
    cycles_in_current_instruction: usize,
    program: &'a [Instruction],
}

impl<'a> Executor<'a> {

    fn for_program(program: &'a [Instruction]) -> Self {
        Self { 
            state: ProgramState::new(),
            program_counter: 0,
            cycles_in_current_instruction: 0,
            program,
        }
    }

    fn is_finished(&self) -> bool {
        self.program_counter >= self.program.len()
    }

    fn step(&mut self) {
        if let Some(instruction) = self.program.get(self.program_counter) {
            self.cycles_in_current_instruction += 1;
            if self.cycles_in_current_instruction == instruction.cycles() {
                self.state.apply_instruction(instruction);
                self.cycles_in_current_instruction = 0;
                self.program_counter += 1;
            }
        }
    }

    fn into_iter(self) -> ExecutorIter<'a> {
        ExecutorIter { started: false, executor: self }
    }
}

struct ExecutorIter<'a> {
    started: bool,
    executor: Executor<'a>,
}

impl<'a> Iterator for ExecutorIter<'a> {
    type Item = ProgramState;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started { 
            self.started = true;
            Some(self.executor.state)
        } else {
            if !self.executor.is_finished() {
                self.executor.step();
                Some(self.executor.state)
            } else {
                None
            }
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

fn crt_line<I: Iterator<Item = isize>>(sprite_positions: &mut I) -> Option<String> {
    let output: String = sprite_positions.take(40).enumerate().map(|(idx, sprite_position)| {
        if (sprite_position - (idx as isize)).abs() <= 1 { '#' } else { ' ' }
    }).collect();
    if output.is_empty() { None } else { Some(output) }
}

fn main() {
    let program: Vec<_> = read_lines().map(|l| parse_line(&l)).collect();

    let executor = Executor::for_program(&program);
    let signal_strengths = executor.into_iter().enumerate().map(|(idx, state)| ((idx + 1) as isize) * state.register);
    println!("Part 1: {}", signal_strengths.skip(19).step_by(40).take(6).sum::<isize>());

    let mut sprite_positions = Executor::for_program(&program).into_iter().map(|state| state.register);
    println!("Part 2:");
    while let Some(line) = crt_line(&mut sprite_positions) {
        println!("{}", line);
    }
}
