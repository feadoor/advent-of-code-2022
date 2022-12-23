use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (isize, isize);

#[derive(Clone)]
struct Movement {
    delta: (isize, isize),
    checks: Vec<(isize, isize)>,
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/23.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_input() -> HashSet<Point> {
    read_lines().enumerate().flat_map(|(x, row)| 
        row.chars().enumerate()
            .filter(|&(_, c)| c == '#')
            .map(move |(y, _)| (x as isize, y as isize))
            .collect_vec().into_iter()
    ).collect()
}

fn all_neighbours((x, y): Point) -> Vec<Point> {
    [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1), (x + 1, y), (x + 1, y - 1), (x, y - 1)].to_vec()
}

fn movement_required(elves: &HashSet<Point>) -> bool {
    elves.iter().any(|&elf| elf_should_move(elf, elves))
}

fn elf_should_move(elf: Point, elves: &HashSet<Point>) -> bool {
    all_neighbours(elf).iter().any(|p| elves.contains(p))
}

fn perform_movement_round(elves: &HashSet<Point>, movement_order: &VecDeque<Movement>) -> HashSet<Point> {
    let mut proposals = HashMap::new(); let mut counts = HashMap::new();

    for &(x, y) in elves.iter() {
        let proposal = if !elf_should_move((x, y), elves) { (x, y) } else { 
            movement_order.iter()
                .find(|m| !m.checks.iter().any(|&(dx, dy)| elves.contains(&(x + dx, y + dy))))
                .map(|m| (x + m.delta.0, y + m.delta.1))
                .unwrap_or((x, y))
        };
        proposals.insert((x, y), proposal); *counts.entry(proposal).or_insert(0) += 1;
    }

    elves.iter().map(|&(x, y)| {
        let proposal = proposals[&(x, y)]; if counts[&proposal] == 1 { proposal } else { (x, y) }
    }).collect()
}

fn find_bounding_box_area(elves: &HashSet<Point>) -> usize {
    let min_x = elves.iter().map(|p| p.0).min().unwrap();
    let max_x = elves.iter().map(|p| p.0).max().unwrap();
    let min_y = elves.iter().map(|p| p.1).min().unwrap();
    let max_y = elves.iter().map(|p| p.1).max().unwrap();
    ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize
}

fn main() {
    let elves = parse_input();
    let movements = VecDeque::from([
        Movement { delta: (-1, 0), checks: [(-1, -1), (-1, 0), (-1, 1)].to_vec() },
        Movement { delta: (1, 0),  checks: [ (1, -1),  (1, 0),  (1, 1)].to_vec() },
        Movement { delta: (0, -1), checks: [(-1, -1), (0, -1), (1, -1)].to_vec() },
        Movement { delta: (0, 1),  checks: [ (-1, 1),  (0, 1),  (1, 1)].to_vec() },
    ]);

    let mut positions = elves.clone(); let mut movement_order = movements.clone();
    for _ in 0..10 {
        positions = perform_movement_round(&positions, &movement_order);
        let first_movement = movement_order.pop_front().unwrap(); 
        movement_order.push_back(first_movement);
    }
    let area = find_bounding_box_area(&positions);
    println!("Part 1: {}", area - positions.len());

    let mut positions = elves.clone(); let mut movement_order = movements.clone(); let mut round = 1;
    while movement_required(&positions) {
        positions = perform_movement_round(&positions, &movement_order);
        let first_movement = movement_order.pop_front().unwrap(); 
        movement_order.push_back(first_movement);
        round += 1;
    }
    println!("Part 2: {}", round);
}