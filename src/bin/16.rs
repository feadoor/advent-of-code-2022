use lazy_static::lazy_static;
use regex::Regex;

use std::collections::{HashMap, VecDeque};
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

struct SearchState {
    actor_idx: usize,
    pressure_so_far: usize, 
    used_valves: Vec<String>, 
    steps_remaining: Vec<usize>, 
    current_valves: Vec<String> 
}

fn find_maximum_pressure_with_n_actors(n: usize, steps: usize, flow_rates: &HashMap<String, usize>, compressed_adjacencies: &HashMap<String, HashMap<String, usize>>) -> usize {

    let mut stack = vec![SearchState { actor_idx: 0, pressure_so_far: 0, used_valves: Vec::new(), steps_remaining: vec![steps; n], current_valves: vec![STARTING_VALVE.to_string(); n] }];
    let mut best_pressure = 0;
    
    while let Some(SearchState { actor_idx, pressure_so_far, used_valves, steps_remaining, current_valves }) = stack.pop() {
        if pressure_so_far > best_pressure { best_pressure = pressure_so_far; }
        let next_steps: Vec<_> = compressed_adjacencies[&current_valves[actor_idx]].iter().filter(|&(k, &v)| !used_valves.contains(k) && v < steps_remaining[actor_idx]).collect();
        if !next_steps.is_empty() {
            for (next_valve, steps_used) in next_steps {
                let mut next_steps_remaining = steps_remaining.clone(); next_steps_remaining[actor_idx] -= steps_used + 1;
                let mut next_pressure = pressure_so_far;                next_pressure += next_steps_remaining[actor_idx] * flow_rates[next_valve];
                let mut next_used_valves = used_valves.clone();         next_used_valves.push(next_valve.to_string());
                let mut next_current_valves = current_valves.clone();   next_current_valves[actor_idx] = next_valve.to_string();
                stack.push(SearchState { actor_idx, pressure_so_far: next_pressure, used_valves: next_used_valves, steps_remaining: next_steps_remaining, current_valves: next_current_valves });
            }
        } else if actor_idx < n - 1 {
            stack.push(SearchState { actor_idx: actor_idx + 1, pressure_so_far, used_valves, steps_remaining, current_valves });
        }
    }

    best_pressure
}

fn main() {
    let valves: Vec<Valve> = read_lines().map(|l| parse_line(&l)).collect();
    let (flow_rates, adjacencies) = collect_valves_as_flow_rates_and_adjacencies(&valves);
    let compressed_adjacencies = compress_adjacencies(&flow_rates, &adjacencies);

    println!("Part 1: {}", find_maximum_pressure_with_n_actors(1, 30, &flow_rates, &compressed_adjacencies));
    println!("Part 2: {}", find_maximum_pressure_with_n_actors(2, 26, &flow_rates, &compressed_adjacencies));
}
