use itertools::{iproduct, Itertools};

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(Copy, Clone)]
enum Material {
    Empty,
    Blizzard(Direction),
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/24.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_char(c: char) -> Material {
    use Material::*;
    match c {
        '>' => Blizzard(Direction::Right),
        '<' => Blizzard(Direction::Left),
        'v' => Blizzard(Direction::Down),
        '^' => Blizzard(Direction::Up),
        _   => Empty,
    }
}

fn parse_map() -> Vec<Vec<Material>> {
    let mut rows = Vec::new();
    for line in read_lines() {
        let row = line.chars().map(parse_char).collect_vec();
        rows.push(row[1 .. row.len() - 1].to_vec());
    }
    rows[1 .. rows.len() - 1].to_vec()
}

fn move_in_direction((x, y): (usize, usize), amount: usize, direction: Direction, height: usize, width: usize) -> (usize, usize) {
    match direction {
        Direction::Up    => ((x + height - (amount % height)) % height, y),
        Direction::Left  => (x, (y + width - (amount % width)) % width),
        Direction::Down  => ((x + (amount % height)) % height, y),
        Direction::Right => (x, (y + (amount % width)) % width),
    }
}

fn add_time_dimension(map: &[Vec<Material>]) -> Vec<Vec<Vec<bool>>> {
    let mut timed_map = Vec::new();
    let height = map.len(); let width = map[0].len(); let time_required = height * width;

    for t in 0..time_required {
        let mut layer = vec![vec![true; width]; height];
        for (px, py) in iproduct!(0..height, 0..width).filter_map(|(x, y)| {
            if let Material::Blizzard(direction) = map[x][y] {
                Some(move_in_direction((x, y), t, direction, height, width))
            } else {
                None
            }
        }) {
            layer[px][py] = false;
        }
        timed_map.push(layer);
    }
    timed_map
}

fn neighbours((x, y, t): (usize, usize, usize), height: usize, width: usize, timespan: usize) -> Vec<(usize, usize, usize)> {
    let mut neighbours = vec![(x, y, (t + 1) % timespan)];
    if x > 0          { neighbours.push((x - 1, y, (t + 1) % timespan)); }
    if x < height - 1 { neighbours.push((x + 1, y, (t + 1) % timespan)); }
    if y > 0          { neighbours.push((x, y - 1, (t + 1) % timespan)); }
    if y < width - 1  { neighbours.push((x, y + 1, (t + 1) % timespan)); }
    neighbours
}

fn shortest_route(starting_square: (usize, usize), ending_square: (usize, usize), starting_time: usize, map: &[Vec<Material>]) -> usize {

    let timed_map = add_time_dimension(map);
    let timespan = timed_map.len(); let height = timed_map[0].len(); let width = timed_map[0][0].len();
    let mut seen = vec![vec![vec![false; timespan]; width]; height]; 
    let mut entry_times = (starting_time .. timespan + starting_time).rev().filter(|&t| timed_map[t % timespan][0][0]).collect_vec();
    let mut queue = VecDeque::new();

    while let Some(entry_time) = entry_times.pop() {

        seen[starting_square.0][starting_square.1][entry_time % timespan] = true;
        queue.push_back((starting_square.0, starting_square.1, entry_time % timespan, entry_time));

        while let Some((x, y, t, d)) = queue.pop_front() {

            if (x, y) == ending_square { 
                return d; 
            }

            if let Some(&entry_time) = entry_times.last() {
                if entry_time == d + 1 {
                    seen[starting_square.0][starting_square.1][entry_time % timespan] = true;
                    queue.push_back((starting_square.0, starting_square.1, entry_time % timespan, entry_time));
                    entry_times.pop();
                }
            }

            for (nx, ny, nt) in neighbours((x, y, t), height, width, timespan) {
                if timed_map[nt][nx][ny] && !seen[nx][ny][nt] {
                    seen[nx][ny][nt] = true;
                    queue.push_back((nx, ny, nt, d + 1));
                }
            }
        }
    }

    panic!("No path found to end")
}

fn main() {
    let map = parse_map();
    let (height, width) = (map.len(), map[0].len());

    let shortest_path_to_goal = shortest_route((0, 0), (height - 1, width - 1), 0, &map);
    println!("Part 1: {}", shortest_path_to_goal + 1);

    let shortest_path_back = shortest_route((height - 1, width - 1), (0, 0), shortest_path_to_goal + 1, &map);
    let shortest_path_there_again = shortest_route((0, 0), (height - 1, width - 1), shortest_path_back + 1, &map);
    println!("Part 2: {}", shortest_path_there_again + 1);
}