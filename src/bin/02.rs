use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {

    fn intrinsic_score(self) -> usize {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl TryFrom<&str> for GameResult {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Shape {

    fn intrinsic_score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn result_against(self, other: Shape) -> GameResult {
        match self {
            Self::Rock => match other {
                Self::Rock => GameResult::Draw,
                Self::Paper => GameResult::Lose,
                Self::Scissors => GameResult::Win,
            },
            Self::Paper => match other {
                Self::Rock => GameResult::Win,
                Self::Paper => GameResult::Draw,
                Self::Scissors => GameResult::Lose,
            },
            Self::Scissors => match other {
                Self::Rock => GameResult::Lose,
                Self::Paper => GameResult::Win,
                Self::Scissors => GameResult::Draw,
            }
        }
    }

    fn score_against(self, other: Shape) -> usize {
        self.intrinsic_score() + self.result_against(other).intrinsic_score()
    }

    fn from_result_and_opponent(result: GameResult, opponent: Shape) -> Self {
        match opponent {
            Shape::Rock => match result {
                GameResult::Win => Self::Paper,
                GameResult::Lose => Self::Scissors,
                GameResult::Draw => Self::Rock,
            },
            Shape::Paper => match result {
                GameResult::Win => Self::Scissors,
                GameResult::Lose => Self::Rock,
                GameResult::Draw => Self::Paper,
            },
            Shape::Scissors => match result {
                GameResult::Win => Self::Rock,
                GameResult::Lose => Self::Paper,
                GameResult::Draw => Self::Scissors,
            }
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/02.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_to_round_data(line: String) -> (String, String) {
    (line[0..1].to_string(), line[2..3].to_string())
}

fn parse_round_as_two_shapes(round: &(String, String)) -> (Shape, Shape) {
    (
        Shape::try_from(round.0.as_str()).expect("unknown character"),
        Shape::try_from(round.1.as_str()).expect("unknown character"),
    )
}

fn parse_round_as_shape_and_result(round: &(String, String)) -> (Shape, GameResult) {
    (
        Shape::try_from(round.0.as_str()).expect("unknown character"),
        GameResult::try_from(round.1.as_str()).expect("unknown character"),
    )
}

fn main() {
    let rounds: Vec<_> = read_lines().map(parse_to_round_data).collect();

    let total_score_part_1: usize = rounds.iter()
        .map(parse_round_as_two_shapes)
        .map(|(yours, mine)| mine.score_against(yours))
        .sum();
    println!("Part 1: {}", total_score_part_1);

    let total_score_part_2: usize = rounds.iter()
        .map(parse_round_as_shape_and_result)
        .map(|(yours, result)| (yours, Shape::from_result_and_opponent(result, yours)))
        .map(|(yours, mine)| mine.score_against(yours))
        .sum();
    println!("Part 2: {}", total_score_part_2);
}
