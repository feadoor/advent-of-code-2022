use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0},
    combinator::{map, map_res, recognize},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser, error::ParseError,
};

use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Blueprint {
    id: usize,
    costs: HashMap<String, Vec<(usize, String)>>,
}

// BEGIN PARSING CODE

fn ws<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    delimited(space0, f, space0)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn single_material(input: &str) -> IResult<&str, (usize, String)> {
    tuple((
        number,
        map(ws(alpha1), str::to_string)
    ))(input)
}

fn material_cost(input: &str) -> IResult<&str, Vec<(usize, String)>> {
    separated_list1(tag("and"), ws(single_material))(input)
}

fn robot(input: &str) -> IResult<&str, (String, Vec<(usize, String)>)> {
    terminated(tuple((
        ws(delimited(tag("Each"), map(ws(alpha1), str::to_string), tag("robot"))),
        ws(preceded(tag("costs"), material_cost)),
    )), tag("."))(input)
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(tuple((
        delimited(tag("Blueprint"), ws(number), ws(tag(":"))),
        map(many0(ws(robot)), |robots| robots.into_iter().collect()),
    )), |(id, costs)| Blueprint { id, costs })(input)
}

// END PARSING CODE

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/19.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_line(line: &str) -> Blueprint {
    blueprint(line).expect("failed to parse blueprint").1
}

fn maximise_geodes(blueprint: &Blueprint, time: usize) -> usize {

    struct SearchState { time_remaining: usize, built_robots: HashMap<String, usize>, held_materials: HashMap<String, usize> }
    let mut stack = vec![SearchState { time_remaining: time, built_robots: HashMap::from([("ore".to_string(), 1)]), held_materials: HashMap::new() }];
    let mut best_geodes = 0;

    let maximum_spending_power = blueprint.costs.values().flatten().fold(HashMap::new(), |mut acc, (n, m)| {
        if n > acc.get(m).unwrap_or(&0) { acc.insert(m.to_string(), *n); }
        acc
    });

    while let Some(SearchState { time_remaining, built_robots, held_materials }) = stack.pop() {
        
        let geodes_at_end = built_robots.get("geode").unwrap_or(&0) * time_remaining + held_materials.get("geode").unwrap_or(&0);
        best_geodes = max(best_geodes, geodes_at_end);

        let geodes_upper_bound = geodes_at_end + time_remaining * (time_remaining - 1) / 2;
        if geodes_upper_bound <= best_geodes { continue; }

        for (material, cost) in blueprint.costs.iter() {
            if built_robots.get(material).unwrap_or(&0) >= maximum_spending_power.get(material).unwrap_or(&usize::max_value()) { continue; }
            let time_required_to_build = cost.iter().map(|(n, m)| built_robots.get(m).map(|r| (n.saturating_sub(*held_materials.get(m).unwrap_or(&0)) + r - 1) / r).unwrap_or(time)).max().unwrap() + 1;
            if time_required_to_build < time_remaining {
                let mut next_materials = held_materials.clone(); 
                let mut next_robots = built_robots.clone(); 
                for (material, count) in built_robots.iter() { *next_materials.entry(material.to_string()).or_insert(0) += count * time_required_to_build; }
                for (count, material) in cost.iter()         { *next_materials.get_mut(material).unwrap() -= count; }
                *next_robots.entry(material.to_string()).or_insert(0) += 1;
                stack.push(SearchState { time_remaining: time_remaining - time_required_to_build, built_robots: next_robots, held_materials: next_materials });
            }
        }
    }

    best_geodes
}

fn main() {
    let blueprints = read_lines().map(|l| parse_line(&l)).collect_vec();

    let quality_levels = blueprints.iter().map(|b| b.id * maximise_geodes(&b, 24));
    println!("Part 1: {}", quality_levels.sum::<usize>());

    let geodes = blueprints[..3].iter().map(|b| maximise_geodes(&b, 32));
    println!("Part 2: {}", geodes.product::<usize>());
}
