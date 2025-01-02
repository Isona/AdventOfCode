use std::{collections::HashMap, fmt::Display};

use aoc_lib::{Coordinate, Direction, Grid};
use intcode::{IntCodePC, IntCodeProgramState};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);
    let start = std::time::Instant::now();

    let part_1_answer = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    pc.reset_all();
    let start = std::time::Instant::now();
    part_2(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2 in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> usize {
    let mut coordinate = Coordinate::new(1000, 1000);
    let mut direction = Direction::North;
    let mut painted_coordinates = HashMap::new();
    pc.set_input([0].into());

    while pc.run_program() != IntCodeProgramState::Halted {
        let mut output = pc.take_output();
        painted_coordinates.insert(coordinate, output.pop_front().unwrap());
        direction = match output.pop_front().unwrap() {
            0 => direction.turn_left(),
            1 => direction.turn_right(),
            _ => panic!(),
        };
        coordinate = coordinate.get_next_in_direction(direction).unwrap();
        let next_input = *painted_coordinates.get(&coordinate).unwrap_or(&0);
        pc.set_input([next_input].into());
    }

    painted_coordinates.len()
}

fn part_2(pc: &mut IntCodePC) {
    let mut grid = Grid::new();
    for _ in 0..10 {
        grid.push_row([PaintColour::Black; 50].to_vec());
    }

    let mut current_coord = Coordinate::new(0, 0);
    let mut direction = Direction::North;
    grid.set(current_coord, PaintColour::White);
    pc.set_input([1].into());

    while pc.run_program() != IntCodeProgramState::Halted {
        let mut output = pc.take_output();
        grid.set(current_coord, output.pop_front().unwrap().into());
        direction = match output.pop_front().unwrap() {
            0 => direction.turn_left(),
            1 => direction.turn_right(),
            _ => panic!(),
        };
        current_coord = grid
            .get_neighbour(current_coord, direction)
            .unwrap()
            .location;
        let next_input = *grid.get(current_coord);
        pc.set_input([next_input as i128].into());
    }

    println!("{grid}");
}

#[derive(Default, Copy, Clone)]
enum PaintColour {
    #[default]
    Black = 0,
    White = 1,
}

impl From<i128> for PaintColour {
    fn from(value: i128) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!(),
        }
    }
}

impl Display for PaintColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaintColour::Black => write!(f, " "),
            PaintColour::White => write!(f, "#"),
        }
    }
}
