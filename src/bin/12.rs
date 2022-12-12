use itertools::iproduct;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coordinate = (usize, usize);

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/12.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn elevation(b: u8) -> u8 {
    match b {
        b'a'..=b'z' => b - b'a',
        b'S'        => 0,
        b'E'        => 25,
        _           => panic!("unexpected character"),
    }
}

fn parse_map<I: Iterator<Item = String>>(input: I) -> (Vec<Vec<u8>>, Coordinate, Coordinate) {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut elevations = Vec::new();
    
    for (row, line) in input.enumerate() {
        elevations.push(Vec::new());
        for (col, b) in line.bytes().enumerate() {
            if b == b'S' { start = (row, col); }
            if b == b'E' { end = (row, col); }
            elevations[row].push(elevation(b));
        }
    }

    (elevations, start, end)
}

fn neighbours(elevations: &[Vec<u8>], coord: Coordinate) -> Vec<Coordinate> {
    let (row, col) = coord;
    [(row.checked_sub(1), Some(col)), (row.checked_add(1), Some(col)), (Some(row), col.checked_sub(1)), (Some(row), col.checked_add(1))]
        .iter()
        .filter(|&(r, c)| r.is_some() && c.is_some() && r.unwrap() < elevations.len() && c.unwrap() < elevations[r.unwrap()].len())
        .filter(|&(r, c)| elevations[row][col] <= elevations[r.unwrap()][c.unwrap()] + 1)
        .map(|(r, c)| (r.unwrap(), c.unwrap()))
        .collect()

}

fn calculate_minimum_distances(elevations: &[Vec<u8>], start: Coordinate) -> Vec<Vec<Option<usize>>> {
    let mut queue = VecDeque::new();
    let mut distances: Vec<Vec<Option<usize>>> = elevations.iter().map(|row| row.iter().map(|_| None).collect()).collect();

    queue.push_back(start);
    distances[start.0][start.1] = Some(0);

    while let Some(current_coordinate) = queue.pop_front() {
        let current_distance = distances[current_coordinate.0][current_coordinate.1].unwrap();
        for next_coordinate in neighbours(elevations, current_coordinate) {
            if let None = distances[next_coordinate.0][next_coordinate.1] {
                distances[next_coordinate.0][next_coordinate.1] = Some(current_distance + 1);
                queue.push_back(next_coordinate);
            }
        }
    }

    distances
}

fn main() {
    let (elevations, start, end) = parse_map(read_lines());
    let minimum_distances = calculate_minimum_distances(&elevations, end);

    let distance_from_start = minimum_distances[start.0][start.1].expect("goal is not reachable");
    println!("Part 1: {}", distance_from_start);

    let distance_from_low_elevation = iproduct!(0..elevations.len(), 0..elevations[0].len())
        .filter(|&(r, c)| elevations[r][c] == 0)
        .filter_map(|(r, c)| minimum_distances[r][c])
        .min().expect("minimum elevation is not reachable");
    println!("Part 2: {}", distance_from_low_elevation);
}
