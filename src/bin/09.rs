use itertools::Itertools;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;

type Coordinate = (isize, isize);
type Movement = (Coordinate, usize);

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/09.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_movement(movement: &str) -> Movement {
    let (direction, amount) = movement.split(" ").collect_tuple().expect("too many items on line");
    let parsed_amount = amount.parse().expect("could not parse amount as integer");
    match direction {
        "R" => ((0, 1), parsed_amount),
        "L" => ((0, -1), parsed_amount),
        "D" => ((1, 0), parsed_amount),
        "U" => ((-1, 0), parsed_amount),
        _   => panic!("unexpected direction")
    }
}

fn drag_position(leader: Coordinate, follower: Coordinate) -> Coordinate {
    let delta = (leader.0 - follower.0, leader.1 - follower.1);
    match delta {
        (-1..=1, -1..=1) => follower,
        (x, y)           => (follower.0 + x.signum(), follower.1 + y.signum()),
    }
}

fn apply_movement(knots: &mut [Coordinate], step: Coordinate) {
    knots[0] = (knots[0].0 + step.0, knots[0].1 + step.1);
    for idx in 1..knots.len() {
        knots[idx] = drag_position(knots[idx - 1], knots[idx]);
    }
}

fn tail_positions<'a>(knots: &'a mut [Coordinate], movements: &'a [Movement]) -> impl Iterator<Item = Coordinate> + 'a {
    let steps = movements.iter().flat_map(|&(step, amount)| repeat(step).take(amount));
    steps.scan(knots, |positions, step| {
        apply_movement(positions, step);
        positions.last().copied()
    })
}

fn main() {
    let movements: Vec<_> = read_lines().map(|s| parse_movement(&s)).collect();

    let mut knot_positions_part_1 = vec![(0, 0); 2];
    let tail_positions_part_1 = tail_positions(&mut knot_positions_part_1, &movements);
    println!("Part 1: {}", tail_positions_part_1.unique().count());

    let mut knot_positions_part_2 = vec![(0, 0); 10];
    let tail_positions_part_2 = tail_positions(&mut knot_positions_part_2, &movements);
    println!("Part 2: {}", tail_positions_part_2.unique().count());
}
