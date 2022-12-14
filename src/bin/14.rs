use itertools::{iterate, Itertools};

use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (usize, usize);

#[derive(Copy, Clone)]
enum Material {
    Rock,
    Sand,
    Air,
}

struct Sandbox {
    entry_point: Point,
    has_floor: bool,
    contents: Vec<Vec<Material>>,
}

impl Sandbox {

    fn with_walls(walls: &[Point], has_floor: bool) -> Self {
        let entry_point = (500, 0);
        let floor_height = walls.iter().map(|&(_, y)| y).max().unwrap_or(0) + 2;
        let maximum_width  = entry_point.0 + floor_height;

        let mut contents = vec![vec![Material::Air; entry_point.0 + maximum_width]; floor_height];
        for &(x, y) in walls {
            contents[y][x] = Material::Rock;
        }

        Self { entry_point, has_floor, contents }
    }

    fn contents_at(&self, point: Point) -> Option<Material> {
        self.contents.get(point.1).and_then(|row| row.get(point.0).copied())
    }

    fn point_to_fall_to(&self, (x, y): Point) -> Option<Point> {
        [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)].into_iter().find(|&p| matches!(self.contents_at(p), Some(Material::Air)))
    }

    fn drop_sand(&mut self) -> bool {
        if let Some(Material::Air) = self.contents_at(self.entry_point) {
            let final_destination = iterate(Some(self.entry_point), |x| x.and_then(|x| self.point_to_fall_to(x)))
                .take_while(|p| p.is_some())
                .last().unwrap().unwrap();
            if self.has_floor || final_destination.1 < self.contents.len() - 1 {
                self.contents[final_destination.1][final_destination.0] = Material::Sand;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/14.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn fill_path(points: &[Point]) -> Vec<Point> {
    let mut path = vec![points[0]];
    for &c in &points[1..] { extend_path(&mut path, c); }
    path
}

fn extend_path(path: &mut Vec<Point>, point: Point) {
    let (old_x, old_y) = *path.last().unwrap();
    match point {
        (new_x, new_y) if new_x == old_x && new_y >= old_y => path.extend((old_y + 1..=new_y).map(|y| (old_x, y))),
        (new_x, new_y) if new_x == old_x && new_y < old_y  => path.extend((new_y..old_y).rev().map(|y| (old_x, y))),
        (new_x, new_y) if new_y == old_y && new_x >= old_x => path.extend((old_x + 1..=new_x).map(|x| (x, old_y))),
        (new_x, new_y) if new_y == old_y && new_x < old_x  => path.extend((new_x..old_x).rev().map(|x| (x, old_y))),
        _                                                  => panic!("Coordinates were not on the same line"),
    };
}

fn parse_coordinate(s: &str) -> Point {
    s.split(",").map(|v| str::parse(v).expect("could not parse as integer")).collect_tuple().expect("wrong number of items in a coordinate")
}

fn parse_path(line: &str) -> Vec<Point> {
    fill_path(&line.split(" -> ").map(parse_coordinate).collect::<Vec<_>>())
}

fn main() {
    let walls: Vec<_> = read_lines().map(|l| parse_path(&l)).flatten().collect();

    let (mut sandbox, mut grains) = (Sandbox::with_walls(&walls, false), 0);
    while let true = sandbox.drop_sand() { grains += 1; }
    println!("Part 1: {}", grains);

    let (mut sandbox, mut grains) = (Sandbox::with_walls(&walls, true), 0);
    while let true = sandbox.drop_sand() { grains += 1; }
    println!("Part 2: {}", grains);
}
