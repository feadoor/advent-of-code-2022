use itertools::Itertools;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Position = (isize, isize, isize);

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/18.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_position(s: &str) -> Position {
    s.split(",").map(|n| n.parse().expect("could not parse as integer")).collect_tuple().expect("wrong number of items to parse")
}

fn neighbours((x, y, z): Position) -> Vec<Position> {
    vec![(x - 1, y, z), (x + 1, y, z), (x, y - 1, z), (x, y + 1, z), (x, y, z - 1), (x, y, z + 1)]
}

fn surface_area(positions: &HashSet<Position>) -> usize {
    positions.iter().map(|&p| neighbours(p).iter().filter(|n| !positions.contains(n)).count()).sum()
}

fn bounding_box(positions: &HashSet<Position>) -> ((isize, isize), (isize, isize), (isize, isize)) {
    (
        (positions.iter().map(|p| p.0).min().unwrap() - 1, positions.iter().map(|p| p.0).max().unwrap() + 1),
        (positions.iter().map(|p| p.1).min().unwrap() - 1, positions.iter().map(|p| p.1).max().unwrap() + 1),
        (positions.iter().map(|p| p.2).min().unwrap() - 1, positions.iter().map(|p| p.2).max().unwrap() + 1),
    )
}

fn exterior_surface_area(positions: &HashSet<Position>) -> usize {
    let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = bounding_box(&positions);
    let mut stack = vec![(min_x, min_y, min_z)]; let mut visited = HashSet::new(); let mut surface_area = 0;
    while let Some((x, y, z)) = stack.pop() {
        let next_positions = neighbours((x, y, z)).into_iter()
            .filter(|n| min_x <= n.0 && n.0 <= max_x && min_y <= n.1 && n.1 <= max_y && min_z <= n.2 && n.2 <= max_z)
            .filter(|n| !visited.contains(n))
            .collect_vec();
        for next in next_positions {
            if positions.contains(&next) { surface_area += 1; }
            else { visited.insert(next); stack.push(next); }
        }
    }
    surface_area
}

fn main() {
    let positions: HashSet<_> = read_lines().map(|l| parse_position(&l)).collect();
    println!("Part 1: {}", surface_area(&positions));
    println!("Part 2: {}", exterior_surface_area(&positions));
}
