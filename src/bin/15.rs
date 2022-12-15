use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (isize, isize);
type Range = (isize, isize);

#[derive(Clone, Debug)]
struct Constraint {
    sensor: Point,
    nearest_beacon: Point,
}

impl Constraint {

    fn radius(&self) -> isize {
        (self.sensor.0 - self.nearest_beacon.0).abs() + (self.sensor.1 - self.nearest_beacon.1).abs()
    }

    fn covered_x_for_fixed_y(&self, y: isize) -> Option<Range> {
        // Equation of this diamond is |x - sensor_x| + |y - sensor_y| ≤ self.radius()
        let rhs = self.radius() - (y - self.sensor.1).abs();
        if rhs < 0 { None } else { Some((self.sensor.0 - rhs, self.sensor.0 + rhs)) }
    }

    fn top_left(&self) -> isize {
        self.sensor.0 + self.sensor.1 - self.radius()
    }

    fn bottom_right(&self) -> isize {
        self.sensor.0 + self.sensor.1 + self.radius()
    }

    fn top_right(&self) -> isize {
        self.sensor.1 - self.sensor.0 - self.radius()
    }

    fn bottom_left(&self) -> isize {
        self.sensor.1 - self.sensor.0 + self.radius()
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/15.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_constraint(line: &str) -> Constraint {
    lazy_static! {
        static ref CONSTRAINT_REGEX: Regex = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();
    }
    CONSTRAINT_REGEX.captures(line).map(|captures| Constraint {
        sensor: (captures["sensor_x"].parse().unwrap(), captures["sensor_y"].parse().unwrap()),
        nearest_beacon: (captures["beacon_x"].parse().unwrap(), captures["beacon_y"].parse().unwrap()),
    }).expect(&format!("failed to parse constraint {}", line))
}

fn union_of_ranges(input_ranges: &[Range]) -> Vec<Range> {
    let mut output_ranges = Vec::new();
    for &(start, end) in input_ranges.iter().sorted() {
        if let Some(&(left, right)) = output_ranges.last() {
            if right >= start { *output_ranges.last_mut().unwrap() = (left, max(right, end)); }
            else              { output_ranges.push((start, end)); }
        } else {
            output_ranges.push((start, end));
        }
    }
    output_ranges
}

fn value_contained_in_ranges(value: isize, ranges: &[Range]) -> bool {
    ranges.iter().any(|&(start, end)| start <= value && value <= end)
}

fn values_not_covered_by_sorted_ranges(bounds: Range, ranges: Vec<Range>) -> impl Iterator<Item = isize> {
    let uncovered_at_start  = bounds.0 .. ranges[0].0;
    let uncovered_at_end    = ranges.last().unwrap().1 + 1 ..= bounds.1;
    let uncovered_in_middle = ranges.into_iter().tuple_windows()
        .filter_map(move |(r1, r2)| if bounds.0 <= r1.1 + 1 && r1.1 + 1 <= bounds.1 { Some(r1.1 + 1 ..= min(r2.0 - 1, bounds.1)) } else { None });

    uncovered_at_start.chain(uncovered_in_middle.flatten()).chain(uncovered_at_end)
}

fn find_a_point_not_covered_by_constraints(bounds: (isize, isize, isize, isize), constraints: &[Constraint]) -> Option<(isize, isize)> {

    let (min_x, max_x, min_y, max_y) = bounds;

    // Use a sweep-line algorithm with a diagonal sweep from top-left to bottom-right. Start by
    // obtaining a list of all of the sorted (x + y)-positions of top-left and bottom-right edges of the
    // diamonds.
    let transfer_points: Vec<_> = constraints.iter()
        .map(|c| vec![c.top_left(), c.bottom_right() + 1])
        .flatten()
        .filter(|&t| t > min_x + min_y)
        .sorted()
        .dedup()
        .collect();

    // Start sweeping!
    let mut current_sweep_position = min_x + min_y;
    let mut currently_included_constraints: Vec<_> = constraints.iter()
        .filter(|c| c.top_left() <= current_sweep_position && current_sweep_position <= c.bottom_right())
        .collect();

    for &transfer in &transfer_points {

        // Check for any uncovered points in (y - x) space at the current (x + y)-position of the sweep line
        let covered_ranges: Vec<_> = currently_included_constraints.iter().map(|c| (c.top_right(), c.bottom_left())).collect();
        let combined_coverage = union_of_ranges(&covered_ranges);
        let bounds_range = (current_sweep_position - 2 * min(current_sweep_position - min_y, max_x), current_sweep_position - 2 * max(current_sweep_position - max_y, min_x));
        if let Some(v) = values_not_covered_by_sorted_ranges(bounds_range, combined_coverage).next() {

            // v is equal to (y - x) and current_sweep_position is equal to (x + y)
            return Some(((current_sweep_position - v) / 2, (current_sweep_position + v) / 2));
        }

        // Move the sweep line to the next transfer point, adding and removing any diamonds that have
        // come into or gone out of scope
        currently_included_constraints = currently_included_constraints.into_iter().filter(|c| c.bottom_right() >= transfer).collect();
        currently_included_constraints.extend(constraints.iter().filter(|c| c.top_left() == transfer));
        current_sweep_position = transfer;
    }

    None
}

fn main() {
    let constraints: Vec<_> = read_lines().map(|l| parse_constraint(&l)).collect();

    let fixed_y_coordinate = 2_000_000;
    let ranges_for_fixed_y: Vec<_> = constraints.iter().filter_map(|c| c.covered_x_for_fixed_y(fixed_y_coordinate)).collect();
    let combined_ranges = union_of_ranges(&ranges_for_fixed_y);
    let size_of_covered_area = combined_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<isize>();
    let known_beacons_in_covered_area = constraints.iter()
        .map(|c| c.nearest_beacon)
        .filter(|&b| b.1 == fixed_y_coordinate && value_contained_in_ranges(b.0, &combined_ranges))
        .unique()
        .count() as isize;
    println!("Part 1: {}", size_of_covered_area - known_beacons_in_covered_area);

    let bounds = (0, 4_000_000, 0, 4_000_000);
    if let Some((x, y)) = find_a_point_not_covered_by_constraints(bounds, &constraints) {
        println!("Part 2: {}", 4_000_000 * x + y);
    }
}
