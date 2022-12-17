use itertools::Itertools;

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::iter::repeat;

#[derive(Copy, Clone)]
enum Movement {
    Left,
    Right,
    Down
}

impl Movement {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Movement::Left,
            '>' => Movement::Right,
            _   => panic!("unexpected character {c}"),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum RockShape {
    Horizontal,
    Plus,
    Vee,
    Vertical,
    Square,
}

impl RockShape {
    fn next_shape(&self) -> Self {
        use RockShape::*;
        match self {
            Horizontal => Plus,
            Plus       => Vee,
            Vee        => Vertical,
            Vertical   => Square,
            Square     => Horizontal,
        }
    }
}

struct Rock {
    shape: RockShape,
    positions: Vec<(usize, usize)>,
}

impl Rock {
    fn new(shape: RockShape, bottom_left: (usize, usize)) -> Self {
        use RockShape::*;
        let (x, y) = bottom_left;
        let positions = match shape {
            Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Plus       => vec![(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)],
            Vee        => vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)],
            Vertical   => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Square     => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        };
        Self { shape, positions }
    }
}

struct Rocktris {
    width: usize,
    placed_rocks: usize,
    movements_completed: usize,
    occupied_spaces: Vec<BTreeSet<usize>>,
    falling_rock: Rock,
}

impl Rocktris {

    fn new(width: usize) -> Self {
        Self {
            width,
            placed_rocks: 0,
            movements_completed: 0,
            occupied_spaces: vec![BTreeSet::from([0]); width + 1],
            falling_rock: Rock::new(RockShape::Horizontal, (3, 4)),
        }
    }

    fn simulate_drop<I: Iterator<Item = Movement>>(&mut self, movements: &mut I) {
        let current_rocks = self.placed_rocks; while self.placed_rocks == current_rocks {
            self.apply_movement(movements.next().unwrap());
        }
    }

    fn state_key(&self) -> (RockShape, Vec<usize>) {
        (
            self.falling_rock.shape,
            self.occupied_spaces[1..].iter()
                .map(|col| { let offset = col.first().unwrap(); col.iter().map(move |x| x - offset) })
                .flatten()
                .collect_vec()
        )
    }

    fn apply_movement(&mut self, movement: Movement) {
        use Movement::*;
        match movement {
            Left  => self.move_left(),
            Right => self.move_right(),
            Down  => self.move_down(),
        }
        self.movements_completed += 1;
    }

    fn move_right(&mut self) {
        if !self.falling_rock.positions.iter().any(|&(x, y)| self.is_blocked((x + 1, y))) {
            self.falling_rock.positions.iter_mut().for_each(|pos| pos.0 += 1);
        }
    }

    fn move_left(&mut self) {
        if !self.falling_rock.positions.iter().any(|&(x, y)| self.is_blocked((x - 1, y))) {
            self.falling_rock.positions.iter_mut().for_each(|pos| pos.0 -= 1);
        }
    }

    fn move_down(&mut self) {
        if !self.falling_rock.positions.iter().any(|&(x, y)| self.is_blocked((x, y - 1))) {
            self.falling_rock.positions.iter_mut().for_each(|pos| pos.1 -= 1);
        } else {
            self.lock_falling_rock();
        }
    }

    fn is_blocked(&self, space: (usize, usize)) -> bool {
        space.0 == 0 || space.0 == self.width + 1 || self.occupied_spaces[space.0].contains(&space.1)
    }

    fn lock_falling_rock(&mut self) {
        let positions_to_lock = self.falling_rock.positions.clone();
        positions_to_lock.iter().for_each(|&(x, y)| { self.occupied_spaces[x].insert(y); });
        self.check_for_complete_lines(&positions_to_lock.iter().map(|&(_x, y)| y).sorted().rev().dedup().collect_vec());
        self.falling_rock = Rock::new(self.falling_rock.shape.next_shape(), self.spawn_position());
        self.placed_rocks += 1;
    }

    fn height(&self) -> usize {
        *self.occupied_spaces[1..].iter().map(|col| col.last().unwrap()).max().unwrap()
    }

    fn spawn_position(&self) -> (usize, usize) {
        (3, self.height() + 4)
    }

    fn check_for_complete_lines(&mut self, heights: &[usize]) {
        for height in heights {
            if self.occupied_spaces[1..].iter().all(|col| col.contains(height)) {
                self.occupied_spaces[1..].iter_mut().for_each(|col| { *col = col.split_off(height); });
                break;
            }
        }
    }
}

fn read_data() -> String {
    fs::read_to_string("inputs/17.txt").expect("unable to read input file")
}

fn collect_movements(input: &str) -> Vec<Movement> {
    input.chars().map(Movement::from_char).interleave_shortest(repeat(Movement::Down)).collect_vec()
}

fn height_after_n_drops_naive(target_drops: usize, movements: Vec<Movement>) -> usize {
    let mut rocktris = Rocktris::new(7);
    let mut movements = movements.into_iter().cycle();
    for _ in 0..target_drops { rocktris.simulate_drop(&mut movements); }
    rocktris.height()
}

fn height_after_n_drops_short_circuit(target_drops: usize, movements: Vec<Movement>) -> usize {
    let mut rocktris = Rocktris::new(7);
    let (movements_length, mut movements) = (movements.len(), movements.into_iter().cycle());
    let mut state_cache: HashMap<_, usize> = HashMap::new();
    let mut heights: Vec<usize> = Vec::new();

    for completed_drops in 0..target_drops {
        let state_key = (rocktris.movements_completed % movements_length, rocktris.state_key());
        if let Some(&drops) = state_cache.get(&state_key) {
            let (period_start, period_length, period_height) = (drops, completed_drops - drops, rocktris.height() - heights[drops]);
            let required_offset = (target_drops - period_start) % period_length;
            let full_periods    = (target_drops - period_start) / period_length;
            return heights[period_start + required_offset] + full_periods * period_height;
        } else {
            state_cache.insert(state_key, completed_drops);
            heights.push(rocktris.height());
            rocktris.simulate_drop(&mut movements);
        }
    }

    rocktris.height()
}

fn main() {
    let movements = collect_movements(&read_data());
    println!("Part 1: {}", height_after_n_drops_naive(2022, movements));

    let movements = collect_movements(&read_data());
    println!("Part 2: {}", height_after_n_drops_short_circuit(1_000_000_000_000, movements));
}
