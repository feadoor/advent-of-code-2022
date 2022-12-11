use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending, space1},
    combinator::{map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult, Parser, error::ParseError,
};

use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum Node {
    File(usize),
    Directory(usize, HashMap<String, Node>),
}

struct NodeIterator<'a> {
    node_stack: Vec<&'a Node>,
}

impl Node {

    fn size(&self) -> usize {
        match self {
            Node::File(size) => *size,
            Node::Directory(size, _) => *size,
        }
    }

    fn is_directory(&self) -> bool {
        matches!(self, Node::Directory(_, _))
    }

    fn iter(&self) -> NodeIterator {
        NodeIterator::new(self)
    }
}

impl<'a> NodeIterator<'a> {

    fn new(node: &'a Node) -> Self {
        Self { node_stack: vec![node] }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node_stack.pop() {
            if let Node::Directory(_, contents) = node {
                self.node_stack.extend(contents.values());
            }
            Some(node)
        } else {
            None
        }
    }
}

// BEGIN NOM PARSING CODE

fn identifier(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn on_line<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    terminated(f, opt(line_ending))
}

fn dummy_directory(input: &str) -> IResult<&str, Option<(&str, Node)>> {
    on_line(value(None, tuple((tag("dir"), space1, identifier))))(input)
}

fn file(input: &str) -> IResult<&str, Option<(&str, Node)>> {
    on_line(map(tuple((number, space1, identifier)), |(size, _, name)| Some((name, Node::File(size)))))(input)
}

fn cd_down(input: &str) -> IResult<&str, &str> {
    on_line(preceded(tuple((tag("$ cd"), space1)), identifier))(input)
}

fn ls(input: &str) -> IResult<&str, ()> {
    on_line(value((), tag("$ ls")))(input)
}

fn cd_up(input: &str) -> IResult<&str, ()> {
    on_line(value((), tuple((tag("$ cd"), space1, tag("..")))))(input)
}

fn directory(input: &str) -> IResult<&str, Option<(&str, Node)>> {
    map(
        terminated(
            tuple((
                terminated(cd_down, ls),
                map(
                    many0(alt((dummy_directory, file, directory))),
                    |items| HashMap::from_iter(items.into_iter().filter_map(|item| item.map(|(name, node)| (name.to_string(), node))))
                )
            )),
            opt(cd_up)
        ), 
        |(name, contents)| Some((name, Node::Directory(contents.values().map(|n| n.size()).sum(), contents)))
    )(input)
}

fn parse(input: &str) -> Node {
    directory(input).expect("failed to parse input").1.unwrap().1
}

// END NOM PARSING CODE

fn read_data() -> String {
    fs::read_to_string("inputs/07.txt").expect("unable to read input file")
}

fn main() {
    let terminal_output = read_data();
    let root_node = parse(&terminal_output);
    
    let small_directories = root_node.iter().filter(|n| n.is_directory() && n.size() <= 100_000);
    println!("Part 1: {}", small_directories.map(|n| n.size()).sum::<usize>());

    let space_required = root_node.size() - 40_000_000;
    let large_directories = root_node.iter().filter(|n| n.is_directory() && n.size() >= space_required);
    let directory_to_delete = large_directories.min_by_key(|n| n.size()).expect("no directories large enough");
    println!("Part 2: {}", directory_to_delete.size());
}
