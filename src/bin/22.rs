use itertools::{iproduct, iterate, Itertools};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Movement {
    Forward(usize), TurnRight, TurnLeft,
}

#[derive(Copy, Clone)]
enum Material {
    Open, Wall, Void,
}

// BEGIN FLAT MAP

#[derive(Copy, Clone)]
enum FlatDirection {
    Right = 0, Down = 1, Left = 2, Up = 3,
}

impl FlatDirection {

    fn rotate_right(&self) -> FlatDirection {
        match self {
            FlatDirection::Up    => FlatDirection::Right,
            FlatDirection::Down  => FlatDirection::Left,
            FlatDirection::Left  => FlatDirection::Up,
            FlatDirection::Right => FlatDirection::Down,
        }
    }

    fn rotate_left(&self) -> FlatDirection {
        match self {
            FlatDirection::Up    => FlatDirection::Left,
            FlatDirection::Down  => FlatDirection::Right,
            FlatDirection::Left  => FlatDirection::Down,
            FlatDirection::Right => FlatDirection::Up,
        }
    }

    fn reverse(&self) -> FlatDirection {
        match self {
            FlatDirection::Up    => FlatDirection::Down,
            FlatDirection::Down  => FlatDirection::Up,
            FlatDirection::Left  => FlatDirection::Right,
            FlatDirection::Right => FlatDirection::Left,
        }
    }
}

struct FlatPosition {
    coords: (usize, usize),
    direction: FlatDirection,
}

struct FlatMap {
    map: Vec<Vec<Material>>,
    position: FlatPosition,
}

impl FlatMap {

    fn new(map: Vec<Vec<Material>>) -> Self {
        let starting_coords = (0, map[0].iter().position(|mat| !matches!(mat, Material::Void)).unwrap());
        Self { map, position: FlatPosition { coords: starting_coords, direction: FlatDirection::Right }}
    }

    fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(amount) => self.move_forward(amount),
            Movement::TurnRight       => self.position.direction = self.position.direction.rotate_right(),
            Movement::TurnLeft        => self.position.direction = self.position.direction.rotate_left(),
        }
    }

    fn move_forward(&mut self, amount: usize) {
        for _ in 0..amount {
            if !self.try_step_forward() { break; }
        }
    }

    fn get_material_at(&self, (x, y): (usize, usize)) -> Material {
        *self.map[x].get(y).unwrap_or(&Material::Void)
    }

    fn try_step_forward(&mut self) -> bool {
        let (target_x, target_y) = self.next_space_wrapping();
        if let Material::Open = self.get_material_at((target_x, target_y)) {
            self.position.coords = (target_x, target_y);
            true
        } else {
            false
        }
    }

    fn next_space_wrapping(&self) -> (usize, usize) {
        iterate(self.coordinates_in_front(self.position.coords), |x| self.coordinates_in_front(*x))
            .find(|&(x, y)| !matches!(self.get_material_at((x, y)), Material::Void))
            .unwrap()
    }

    fn coordinates_in_front(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self.position.direction {
            FlatDirection::Up    => ((x + self.map.len() - 1) % self.map.len(), y),
            FlatDirection::Down  => ((x + 1) % self.map.len(), y),
            FlatDirection::Left  => (x, (y + self.map[x].len() - 1) % self.map[x].len()),
            FlatDirection::Right => (x, (y + 1) % self.map[x].len()),
        }
    }
}

// END FLAT MAP

// BEGIN CUBE MAP

#[derive(Copy, Clone)]
enum Face {
    U = 0, F = 1, R = 2, L = 3, B = 4, D = 5,
}

#[derive(Copy, Clone)]
enum Orientation {
    UF =  0, UR =  1, UB =  2, UL =  3, FD =  4, FR =  5, FU =  6, FL =  7, 
    RD =  8, RB =  9, RU = 10, RF = 11, LD = 12, LF = 13, LU = 14, LB = 15, 
    BD = 16, BL = 17, BU = 18, BR = 19, DB = 20, DR = 21, DF = 22, DL = 23,
}

impl Orientation {
    fn top_face(&self) -> Face {
        use Orientation::*;
        match self {
            UF | UR | UB | UL => Face::U,
            FD | FR | FU | FL => Face::F,
            RD | RB | RU | RF => Face::R,
            LD | LF | LU | LB => Face::L,
            BD | BL | BU | BR => Face::B,
            DB | DR | DF | DL => Face::D,
        }
    }
}

#[derive(Copy, Clone)]
enum FoldDirection {
    Up, Down, Left, Right,
}

#[derive(Copy, Clone)]
enum CubeDirection {
    Up = 0, Down = 1, Left = 2, Right = 3, In = 4, Out = 5,
}

impl CubeDirection {

    fn rotate_right(&self, face: Face) -> CubeDirection {
        use CubeDirection::*;
        match face {
            Face::U => vec![Up, Down, In, Out, Right, Left][*self as usize],
            Face::F => vec![Right, Left, Up, Down, In, Out][*self as usize],
            Face::R => vec![In, Out, Left, Right, Down, Up][*self as usize],
            Face::L => vec![Out, In, Left, Right, Up, Down][*self as usize],
            Face::B => vec![Left, Right, Down, Up, In, Out][*self as usize],
            Face::D => vec![Up, Down, Out, In, Left, Right][*self as usize],
        }
    }

    fn rotate_left(&self, face: Face) -> CubeDirection {
        use CubeDirection::*;
        match face {
            Face::U => vec![Up, Down, Out, In, Left, Right][*self as usize],
            Face::F => vec![Left, Right, Down, Up, In, Out][*self as usize],
            Face::R => vec![Out, In, Left, Right, Up, Down][*self as usize],
            Face::L => vec![In, Out, Left, Right, Down, Up][*self as usize],
            Face::B => vec![Right, Left, Up, Down, In, Out][*self as usize],
            Face::D => vec![Up, Down, In, Out, Right, Left][*self as usize],
        }
    }
}

struct CubePosition {
    face: Face,
    coords: (usize, usize),
    direction: CubeDirection,
}

struct CubeMap {
    side_length: usize,
    faces: Vec<Vec<Vec<Material>>>,
    position_translator: Vec<Vec<Vec<(usize, usize)>>>,
    direction_translator: Vec<Vec<Option<FlatDirection>>>,
    position: CubePosition,
}

impl CubeMap {

    fn new(map: Vec<Vec<Material>>) -> Self {

        // First calculate the side length of the cube. This will be helpful.
        let total_surface: usize = map.iter().map(|row| row.iter().filter(|&mat| !matches!(mat, Material::Void)).count()).sum();
        let side_length = ((total_surface / 6) as f64).sqrt().floor() as usize;

        // Some space where we will store the faces as we discover them and the translation
        // from directions on the cube to directions on the original map
        let mut faces = vec![vec![vec![Material::Void; side_length]; side_length]; 6]; 
        let mut position_translator = vec![vec![vec![(0, 0); side_length]; side_length]; 6];
        let mut direction_translator = vec![vec![None; 6]; 6];

        // Make the arbitrary decision that the top-left-most square in the input
        // corresponds to a top-down view of the top face of the cube.
        let starting_coords = (0, map[0].iter().position(|mat| !matches!(mat, Material::Void)).unwrap());
        let mut seen_faces = vec![false; 6];
        let mut stack = vec![(Orientation::UF, starting_coords)];

        // Traverse the net of the cube and store the contents of the faces as we find them
        while let Some((orientation, (start_x, start_y))) = stack.pop() {
            
            // Copy data about the top face into the cubical map
            let top_face = orientation.top_face(); seen_faces[top_face as usize] = true;
            for (x, y) in iproduct!(start_x .. start_x + side_length, start_y .. start_y + side_length) {
                let (fx, fy) = Self::canonical_position_on_face((x - start_x, y - start_y), side_length, orientation);
                position_translator[top_face as usize][fx][fy] = (x, y);
                faces[top_face as usize][fx][fy] = map[x][y];
            }

            // Store how to translate directions on this face to directions on the original map
            direction_translator[top_face as usize] = Self::actual_direction_translation(orientation);

            // Look for the faces directly reachable from this face on the net
            for (fold_direction, (next_x, next_y)) in Self::search_positions((start_x, start_y), side_length) {
                if next_x < map.len() && next_y < map[next_x].len() && !matches!(map[next_x][next_y], Material::Void) {
                    let next_orientation = Self::next_orientation(orientation, fold_direction);
                    if !seen_faces[next_orientation.top_face() as usize] {
                        stack.push((next_orientation, (next_x, next_y))); 
                    }
                }
            }
        }

        Self { side_length, faces, position_translator, direction_translator, position: CubePosition { face: Face::U, coords: (0, 0), direction: CubeDirection::Right }}
    }

    fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(amount) => self.move_forward(amount),
            Movement::TurnRight       => self.position.direction = self.position.direction.rotate_right(self.position.face),
            Movement::TurnLeft        => self.position.direction = self.position.direction.rotate_left(self.position.face),
        }
    }

    fn move_forward(&mut self, amount: usize) {
        for _ in 0..amount {
            if !self.try_step_forward() { break; }
        }
    }

    fn try_step_forward(&mut self) -> bool {
        let (face, (x, y), direction) = self.next_position();
        if let Material::Open = self.faces[face as usize][x][y] {
            self.position = CubePosition { face, coords: (x, y), direction };
            true
        } else {
            false
        }
    }

    fn next_position(&self) -> (Face, (usize, usize), CubeDirection) {
        match self.position.face {
            Face::U => self.next_position_u(),
            Face::F => self.next_position_f(),
            Face::R => self.next_position_r(),
            Face::L => self.next_position_l(),
            Face::B => self.next_position_b(),
            Face::D => self.next_position_d(),
        }
    }

    fn next_position_u(&self) -> (Face, (usize, usize), CubeDirection) {
        use CubeDirection::*;
        let ((x, y), side_length, direction) = (self.position.coords, self.side_length, self.position.direction);
        match direction {
            Left  => if y > 0               { (Face::U, (x, y - 1), direction) } else { (Face::L, (0, x),                   Down) },
            Right => if y < side_length - 1 { (Face::U, (x, y + 1), direction) } else { (Face::R, (0, side_length - x - 1), Down) },
            In    => if x > 0               { (Face::U, (x - 1, y), direction) } else { (Face::B, (0, side_length - y - 1), Down) },
            Out   => if x < side_length - 1 { (Face::U, (x + 1, y), direction) } else { (Face::F, (0, y),                   Down) },
            _     => panic!("illegal direction for face U"),
        }
    }

    fn next_position_f(&self) -> (Face, (usize, usize), CubeDirection) {
        use CubeDirection::*;
        let ((x, y), side_length, direction) = (self.position.coords, self.side_length, self.position.direction);
        match direction {
            Left  => if y > 0               { (Face::F, (x, y - 1), direction) } else { (Face::L, (x, side_length - 1), In) },
            Right => if y < side_length - 1 { (Face::F, (x, y + 1), direction) } else { (Face::R, (x, 0),               In) },
            Up    => if x > 0               { (Face::F, (x - 1, y), direction) } else { (Face::U, (side_length - 1, y), In) },
            Down  => if x < side_length - 1 { (Face::F, (x + 1, y), direction) } else { (Face::D, (0, y),               In) },
            _     => panic!("illegal direction for face F"),
        }
    }

    fn next_position_r(&self) -> (Face, (usize, usize), CubeDirection) {
        use CubeDirection::*;
        let ((x, y), side_length, direction) = (self.position.coords, self.side_length, self.position.direction);
        match direction {
            Out  => if y > 0               { (Face::R, (x, y - 1), direction) } else { (Face::F, (x, side_length - 1),                   Left) },
            In   => if y < side_length - 1 { (Face::R, (x, y + 1), direction) } else { (Face::B, (x, 0),                                 Left) },
            Up   => if x > 0               { (Face::R, (x - 1, y), direction) } else { (Face::U, (side_length - y - 1, side_length - 1), Left) },
            Down => if x < side_length - 1 { (Face::R, (x + 1, y), direction) } else { (Face::D, (y, side_length - 1),                   Left) },
            _    => panic!("illegal direction for face R"),
        }
    }

    fn next_position_l(&self) -> (Face, (usize, usize), CubeDirection) {
        use CubeDirection::*;
        let ((x, y), side_length, direction) = (self.position.coords, self.side_length, self.position.direction);
        match direction {
            In   => if y > 0               { (Face::L, (x, y - 1), direction) } else { (Face::B, (x, side_length - 1),     Right) },
            Out  => if y < side_length - 1 { (Face::L, (x, y + 1), direction) } else { (Face::F, (x, 0),                   Right) },
            Up   => if x > 0               { (Face::L, (x - 1, y), direction) } else { (Face::U, (y, 0),                   Right) },
            Down => if x < side_length - 1 { (Face::L, (x + 1, y), direction) } else { (Face::D, (side_length - y - 1, 0), Right) },
            _    => panic!("illegal direction for face L"),
        }
    }

    fn next_position_b(&self) -> (Face, (usize, usize), CubeDirection) {
        use CubeDirection::*;
        let ((x, y), side_length, direction) = (self.position.coords, self.side_length, self.position.direction);
        match direction {
            Right => if y > 0               { (Face::B, (x, y - 1), direction) } else { (Face::R, (x, side_length - 1),                   Out) },
            Left  => if y < side_length - 1 { (Face::B, (x, y + 1), direction) } else { (Face::L, (x, 0),                                 Out) },
            Up    => if x > 0               { (Face::B, (x - 1, y), direction) } else { (Face::U, (0, side_length - y - 1),               Out) },
            Down  => if x < side_length - 1 { (Face::B, (x + 1, y), direction) } else { (Face::D, (side_length - 1, side_length - y - 1), Out) },
            _     => panic!("illegal direction for face B"),
        }
    }

    fn next_position_d(&self) -> (Face, (usize, usize), CubeDirection) {
        use CubeDirection::*;
        let ((x, y), side_length, direction) = (self.position.coords, self.side_length, self.position.direction);
        match direction {
            Left  => if y > 0               { (Face::D, (x, y - 1), direction) } else { (Face::L, (side_length - 1, side_length - x - 1), Up) },
            Right => if y < side_length - 1 { (Face::D, (x, y + 1), direction) } else { (Face::R, (side_length - 1, x),                   Up) },
            Out   => if x > 0               { (Face::D, (x - 1, y), direction) } else { (Face::F, (side_length - 1, y),                   Up) },
            In    => if x < side_length - 1 { (Face::D, (x + 1, y), direction) } else { (Face::B, (side_length - 1, side_length - y - 1), Up) },
            _     => panic!("illegal direction for face D"),
        }
    }

    fn search_positions((x, y): (usize, usize), side_length: usize) -> Vec<(FoldDirection, (usize, usize))> {
        let mut search_positions = vec![(FoldDirection::Down, (x + side_length, y)), (FoldDirection::Right, (x, y + side_length))];
        if y >= side_length { search_positions.push((FoldDirection::Left, (x, y - side_length))); }
        if x >= side_length { search_positions.push((FoldDirection::Up, (x - side_length, y))); }
        search_positions
    }

    fn canonical_position_on_face((x, y): (usize, usize), side_length: usize, orientation: Orientation) -> (usize, usize) {
        match (orientation as usize) % 4 {
            0 => (x, y),
            1 => (side_length - y - 1, x),
            2 => (side_length - x - 1, side_length - y - 1),
            3 => (y, side_length - x - 1),
            _ => unreachable!(),
        }
    }

    fn canonical_direction_translation(face: Face) -> Vec<Option<FlatDirection>> {
        use FlatDirection::*;
        match face {
            Face::U => vec![None, None, Some(Left), Some(Right), Some(Up), Some(Down)],
            Face::F => vec![Some(Up), Some(Down), Some(Left), Some(Right), None, None],
            Face::R => vec![Some(Up), Some(Down), None, None, Some(Right), Some(Left)],
            Face::L => vec![Some(Up), Some(Down), None, None, Some(Left), Some(Right)],
            Face::B => vec![Some(Up), Some(Down), Some(Right), Some(Left), None, None],
            Face::D => vec![None, None, Some(Left), Some(Right), Some(Down), Some(Up)],
        }
    }

    fn actual_direction_translation(orientation: Orientation) -> Vec<Option<FlatDirection>> {
        let rotation_func = match (orientation as usize) % 4 {
            0 => |d: FlatDirection| d,
            1 => |d: FlatDirection| d.rotate_right(),
            2 => |d: FlatDirection| d.reverse(),
            3 => |d: FlatDirection| d.rotate_left(),
            _ => unreachable!(),
        };
        Self::canonical_direction_translation(orientation.top_face()).into_iter().map(|d| d.map(rotation_func)).collect()
    }

    fn next_orientation(orientation: Orientation, fold: FoldDirection) -> Orientation {
        use Orientation::*;
        match fold {
            FoldDirection::Up    => vec![BU, LU, FU, RU, UF, LF, DF, RF, UR, FR, DR, BR, UL, BL, DL, FL, UB, RB, DB, LB, FD, LD, BD, RD][orientation as usize],
            FoldDirection::Down  => vec![FD, RD, BD, LD, DB, RB, UB, LB, DL, BL, UL, FL, DR, FR, UR, BR, DF, LF, UF, RF, BU, RU, FU, LU][orientation as usize],
            FoldDirection::Left  => vec![LF, FR, RB, BL, LD, DR, RU, UL, FD, DB, BU, UF, BD, DF, FU, UB, RD, DL, LU, UR, LB, BR, RF, FL][orientation as usize],
            FoldDirection::Right => vec![RF, BR, LB, FL, RD, UR, LU, DL, BD, UB, FU, DF, FD, UF, BU, DB, LD, UL, RU, DR, RB, FR, LF, BL][orientation as usize],
        }
    }
}

// END CUBE MAP

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/22.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_map_row(s: &str) -> Vec<Material> {
    s.chars().map(|c| match c {
        '.' => Material::Open,
        '#' => Material::Wall,
        _   => Material::Void,
    }).collect()
}

fn parse_movements(s: &str) -> Vec<Movement> {
    s.chars().group_by(|c| c.is_digit(10)).into_iter().map(|(is_digit, mut group)| {
        if is_digit { 
            Movement::Forward(group.collect::<String>().parse().unwrap())
        } else { match group.next().unwrap() {
            'R' => Movement::TurnRight,
            'L' => Movement::TurnLeft,
            _   => panic!("unexpected char in movement"),
        }}
    }).collect()
}

fn parse_input() -> (Vec<Vec<Material>>, Vec<Movement>) {
    let mut map = Vec::new(); let mut lines = read_lines();
    
    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        map.push(parse_map_row(&line));
    }

    (map, parse_movements(&lines.next().unwrap()))
}

fn main() {
    let (map, movements) = parse_input();

    let mut flat_map = FlatMap::new(map.clone());
    for &movement in movements.iter() { flat_map.apply_movement(movement); }
    let (x, y, direction) = (flat_map.position.coords.0, flat_map.position.coords.1, flat_map.position.direction as usize);
    println!("Part 1: {}", 1_000 * (x + 1) + 4 * (y + 1) + direction);

    let mut cube_map = CubeMap::new(map.clone());
    for &movement in movements.iter() { cube_map.apply_movement(movement); }
    let face = cube_map.position.face as usize;
    let ((x, y), direction) = (
        cube_map.position_translator[face][cube_map.position.coords.0][cube_map.position.coords.1], 
        cube_map.direction_translator[face][cube_map.position.direction as usize].unwrap() as usize
    );
    println!("Part 2: {}", 1_000 * (x + 1) + 4 * (y + 1) + direction);
}
