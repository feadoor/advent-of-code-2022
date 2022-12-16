use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use std::cmp::max;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const STARTING_VALVE: &str = "AA";

struct Valve {
    name: String,
    flow_rate: usize,
    neighbours: Vec<String>,
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/16.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_line(line: &str) -> Valve {
    lazy_static! {
        static ref VALVE_REGEX: Regex = Regex::new(r"Valve (?P<name>[^\s]+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<neighbours>.+)").unwrap();
    }
    VALVE_REGEX.captures(line).map(|captures| Valve {
        name: captures["name"].to_string(),
        flow_rate: captures["rate"].parse().unwrap(),
        neighbours: captures["neighbours"].split(", ").map(|s| s.to_string()).collect()
    }).expect(&format!("failed to parse valve {}", line))
}

fn collect_valves_as_flow_rates_and_adjacencies(valves: &[Valve]) -> (HashMap<String, usize>, HashMap<String, Vec<String>>) {
    let (mut flow_rates, mut adjacencies) = (HashMap::new(), HashMap::new());
    for valve in valves {
        flow_rates.insert(valve.name.clone(), valve.flow_rate);
        adjacencies.insert(valve.name.clone(), valve.neighbours.clone());
    }
    (flow_rates, adjacencies)
}

fn distances_from_valve(starting_valve: &str, adjacencies: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    let mut distances = HashMap::new(); distances.insert(starting_valve.to_string(), 0);
    let mut queue = VecDeque::new(); queue.push_back((starting_valve.to_string(), 0));
    while let Some((current_node, current_distance)) = queue.pop_front() {
        for neighbour in &adjacencies[&current_node] {
            if !distances.contains_key(neighbour) {
                distances.insert(neighbour.to_string(), current_distance + 1);
                queue.push_back((neighbour.to_string(), current_distance + 1));
            }
        }
    }
    distances
}

// Compress the adjacencies list to only care about the distance between valves with positive flow rate
fn compress_adjacencies(flow_rates: &HashMap<String, usize>, adjacencies: &HashMap<String, Vec<String>>) -> HashMap<String, HashMap<String, usize>> {
    let positive_flows: Vec<String> = flow_rates.iter().filter(|&(_k, &v)| v > 0).map(|(k, _v)| k.to_string()).collect();
    let mut compressed_adjacencies = HashMap::new();
    
    for valve in &positive_flows {
        let distances = distances_from_valve(valve, adjacencies).into_iter().filter(|(k, _v)| positive_flows.contains(k)).collect();
        compressed_adjacencies.insert(valve.to_string(), distances);
    }

    // Also add distances from the starting valve, or we'll get stuck
    let distances_from_start = distances_from_valve(STARTING_VALVE, adjacencies).into_iter().filter(|(k, _v)| positive_flows.contains(k)).collect();
    compressed_adjacencies.insert(STARTING_VALVE.to_string(), distances_from_start);

    compressed_adjacencies
}

fn find_possible_total_pressures_for_steps(steps: usize, flow_rates: &HashMap<String, usize>, compressed_adjacencies: &HashMap<String, HashMap<String, usize>>) -> HashMap<BTreeSet<String>, usize> {

    struct SearchState { pressure_so_far: usize, valves_opened: BTreeSet<String>, steps_remaining: usize, current_valve: String }

    let mut possible_pressures = HashMap::new();
    let mut stack = vec![SearchState { pressure_so_far: 0, valves_opened: BTreeSet::new(), steps_remaining: steps, current_valve: STARTING_VALVE.to_string()} ];
    while let Some(SearchState { pressure_so_far, valves_opened, steps_remaining, current_valve }) = stack.pop() {

        // Update the best known pressure for this set of open valves
        let current_best_pressure = *possible_pressures.get(&valves_opened).unwrap_or(&0);
        possible_pressures.insert(valves_opened.clone(), max(pressure_so_far, current_best_pressure));

        // Add all possible next states to the stack
        for (next_valve, steps_used) in compressed_adjacencies[&current_valve].iter().filter(|&(k, &v)| !valves_opened.contains(k) && v < steps_remaining) {
            let next_steps_remaining   = steps_remaining - steps_used - 1;
            let next_pressure          = pressure_so_far + next_steps_remaining * flow_rates[next_valve];
            let mut next_valves_opened = valves_opened.clone(); next_valves_opened.insert(next_valve.to_string());
            stack.push(SearchState { pressure_so_far: next_pressure, valves_opened: next_valves_opened, steps_remaining: next_steps_remaining, current_valve: next_valve.to_string() });
        }
    }

    possible_pressures
}

fn best_pressure_from_two_disjoint_subsets(valves: &BTreeSet<String>, possible_pressures: &HashMap<BTreeSet<String>, usize>) -> usize {
    let mut best_combined_pressure = 0;
    for (first_subset, first_pressure) in possible_pressures.iter() {
        let allowable_valves = valves.iter().filter(|&v| !first_subset.contains(v));
        for second_subset in allowable_valves.powerset().map(|s| BTreeSet::from_iter(s.into_iter().map(|v| v.to_string()))) {
            if let Some(second_pressure) = possible_pressures.get(&second_subset) {
                best_combined_pressure = max(best_combined_pressure, first_pressure + second_pressure);
            }
        }
    } 
    best_combined_pressure
}

fn main() {
    let valves: Vec<Valve> = read_lines().map(|l| parse_line(&l)).collect();
    let (flow_rates, adjacencies) = collect_valves_as_flow_rates_and_adjacencies(&valves);
    let compressed_adjacencies = compress_adjacencies(&flow_rates, &adjacencies);

    let possible_pressures = find_possible_total_pressures_for_steps(30, &flow_rates, &compressed_adjacencies);
    println!("Part 1: {}", possible_pressures.values().max().unwrap());

    let possible_pressures = find_possible_total_pressures_for_steps(26, &flow_rates, &compressed_adjacencies);
    let interesting_valves: BTreeSet<String> = valves.iter().filter(|&v| v.flow_rate > 0).map(|v| v.name.to_string()).collect();
    println!("Part 2: {}", best_pressure_from_two_disjoint_subsets(&interesting_valves, &possible_pressures));
}
