use std::fs::File;
use std::io::{BufRead, BufReader};

struct Node {
    next: usize,
    prev: usize,
}

struct LinkedList {
    nodes: Vec<Node>,
}

impl LinkedList {

    fn with_length(len: usize) -> Self {
        Self {
            nodes: (0..len).map(|idx| Node {
                next: (idx + 1).rem_euclid(len), 
                prev: (idx + len - 1).rem_euclid(len) 
            }).collect(),
        }
    }

    fn step_forward(&self, mut idx: usize, amount: usize) -> usize {
        let steps = amount.rem_euclid(self.nodes.len());
        for _ in 0..steps { idx = self.nodes[idx].next; }
        idx
    }

    fn shift_node(&mut self, idx: usize, shift: isize) {
        let shift_amount = shift.rem_euclid((self.nodes.len() - 1) as isize) as usize;
        if shift_amount <= self.nodes.len() - 1 - shift_amount { 
            self.shift_right(idx, shift_amount); 
        } else {
            self.shift_left(idx, self.nodes.len() - shift_amount - 1);
        }
    }

    fn shift_right(&mut self, idx: usize, amount: usize) {
        for _ in 0..amount {
            let (prev, curr, next, next_next) = (self.nodes[idx].prev, idx, self.nodes[idx].next, self.nodes[self.nodes[idx].next].next);
            self.nodes[prev].next = next;
            self.nodes[curr].prev = next; self.nodes[curr].next = next_next;
            self.nodes[next].prev = prev; self.nodes[next].next = curr;
            self.nodes[next_next].prev = curr;
        }
    }

    fn shift_left(&mut self, idx: usize, amount: usize) {
        for _ in 0..amount {
            let (prev_prev, prev, curr, next) = (self.nodes[self.nodes[idx].prev].prev, self.nodes[idx].prev, idx, self.nodes[idx].next);
            self.nodes[prev_prev].next = curr;
            self.nodes[prev].prev = curr; self.nodes[prev].next = next;
            self.nodes[curr].prev = prev_prev; self.nodes[curr].next = prev;
            self.nodes[next].prev = prev;
        }
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/20.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn main() {
    let numbers: Vec<isize> = read_lines().map(|l| l.parse().expect("could not parse as integer")).collect();
    let zero_index = numbers.iter().position(|&n| n == 0).unwrap();
    
    let mut linked_list = LinkedList::with_length(numbers.len());
    for (idx, number) in numbers.iter().copied().enumerate() {
        linked_list.shift_node(idx, number);
    }
    let answer: isize = [1000, 2000, 3000].into_iter().map(|n| numbers[linked_list.step_forward(zero_index, n)]).sum();
    println!("Part 1: {}", answer);

    let numbers: Vec<isize> = numbers.into_iter().map(|n| n * 811_589_153).collect();
    let mut linked_list = LinkedList::with_length(numbers.len());
    for _ in 0..10 {
        for (idx, number) in numbers.iter().copied().enumerate() {
            linked_list.shift_node(idx, number);
        }
    }
    let answer: isize = [1000, 2000, 3000].into_iter().map(|n| numbers[linked_list.step_forward(zero_index, n)]).sum();
    println!("Part 2: {}", answer);
}
