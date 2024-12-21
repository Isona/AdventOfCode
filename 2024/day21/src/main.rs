use core::fmt;
use std::fmt::Display;

use aoc_lib::{Coordinate, Direction, Grid};
use cached::proc_macro::cached;
use itertools::Itertools;
use petgraph::{algo::all_simple_paths, graphmap::DiGraphMap, prelude::GraphMap};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = std::time::Instant::now();
    let part_1_answer = part_1(INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

// 184712 is too high
fn part_1(input: &str) -> u64 {
    let numeric_keypad = NumericKeypad::new();
    let directional_keypad = DirectionalKeypad::new();

    let mut output = 0;
    for line in input.lines() {
        // Get the individual buttons to push, as coords
        let mut buttons = Vec::from([numeric_keypad[10]]);
        buttons.extend(
            line.chars()
                .map(|x| numeric_keypad[x.to_digit(16).unwrap() as usize]),
        );
        println!("{buttons:?}");

        let second_buttons = get_next_buttons(&directional_keypad, &buttons, true);
        println!("Second Buttons: {second_buttons:?}");

        let third_buttons = get_next_buttons(&directional_keypad, &second_buttons, false);
        println!("Third Buttons: {third_buttons:?}");

        let fourth_buttons = get_next_buttons(&directional_keypad, &third_buttons, false);
        println!("Fourth Buttons: {fourth_buttons:?}");

        let input_number: usize = line[0..line.len() - 1].parse().unwrap();
        println!();
        println!(
            "{line}: {input_number}, first_buttons_len:{}, second_buttons_len: {}, third_buttons_len: {}, fourth_buttons_len: {}",
            buttons.len(),
            second_buttons.len(),
            third_buttons.len(),
            fourth_buttons.len() - 1
        );
        println!();
        output += (input_number * (fourth_buttons.len() - 1)) as u64;
    }

    output
}

fn part_2(input: &str) -> u64 {
    todo!();
}

fn get_next_buttons(
    directional_keypad: &[Coordinate],
    initial_buttons: &[Coordinate],
    is_num_keypad: bool,
) -> Vec<Coordinate> {
    let mut buttons = Vec::from([directional_keypad[KeypadDirection::A as usize]]);
    for (start_button, end_button) in initial_buttons.iter().tuple_windows() {
        buttons.extend(
            get_directions(*start_button, *end_button, is_num_keypad)
                .iter()
                .map(|direction| directional_keypad[*direction as usize]),
        );
    }

    buttons
}

// Wrapper for caching
#[cached(key = "String", convert = r#"{ format!("{start},{end}") }"#)]
fn get_numeric_paths(
    numeric_keypad: &NumericKeypad,
    start: u32,
    end: u32,
) -> Vec<Vec<KeypadDirection>> {
    numeric_keypad.get_paths(start, end)
}

// Wrapper for caching
#[cached(key = "String", convert = r#"{ format!("{start},{end}") }"#)]
fn get_directional_paths(
    directional_keypad: &DirectionalKeypad,
    start: KeypadDirection,
    end: KeypadDirection,
) -> Vec<Vec<KeypadDirection>> {
    directional_keypad.get_paths(start, end)
}

struct NumericKeypad {
    grid: Grid<u32>,
    graph: DiGraphMap<Coordinate, KeypadDirection>,
}

impl NumericKeypad {
    fn new() -> Self {
        let mut grid = Grid::new();
        grid.push_row(Vec::from([7, 8, 9]));
        grid.push_row(Vec::from([4, 5, 6]));
        grid.push_row(Vec::from([1, 2, 3]));
        grid.push_row(Vec::from([11, 0, 10]));

        let mut graph = DiGraphMap::new();
        for (coord, value) in grid.indexed_iter() {
            if value == &11 {
                continue;
            }

            for neighbour in grid
                .get_cardinal_neighbours(coord)
                .filter(|x| x.value != &11)
            {
                graph.add_edge(
                    coord,
                    neighbour.location,
                    KeypadDirection::from(neighbour.direction),
                );
            }
        }

        Self { grid, graph }
    }

    fn get_paths(&self, start: u32, end: u32) -> Vec<Vec<KeypadDirection>> {
        let start_coord = self.grid.find_item(&start).unwrap();
        let end_coord = self.grid.find_item(&end).unwrap();
        let manhattan = start_coord.get_manhattan_distance(end_coord);

        let paths: Vec<Vec<Coordinate>> = all_simple_paths::<Vec<_>, _>(
            &self.graph,
            start_coord,
            end_coord,
            manhattan - 1,
            Some(manhattan - 1),
        )
        .collect();

        let mut direction_lists = Vec::new();

        for path in paths {
            direction_lists.push(
                path.iter()
                    .tuple_windows()
                    .map(|(coord_1, coord_2)| {
                        KeypadDirection::from(coord_1.get_direction_to(coord_2).unwrap())
                    })
                    .collect(),
            );
        }

        direction_lists
    }
}

struct DirectionalKeypad {
    grid: Grid<u32>,
    graph: DiGraphMap<Coordinate, KeypadDirection>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        let mut grid = Grid::new();
        grid.push_row(Vec::from([11, 0, 4]));
        grid.push_row(Vec::from([2, 1, 3]));

        let mut graph = DiGraphMap::new();
        for (coord, value) in grid.indexed_iter() {
            if value == &11 {
                continue;
            }

            for neighbour in grid
                .get_cardinal_neighbours(coord)
                .filter(|x| x.value != &11)
            {
                graph.add_edge(
                    coord,
                    neighbour.location,
                    KeypadDirection::from(neighbour.direction),
                );
            }
        }

        Self { grid, graph }
    }

    fn get_paths(&self, start: KeypadDirection, end: KeypadDirection) -> Vec<Vec<KeypadDirection>> {
        let start_coord = self.grid.find_item(&(start as u32)).unwrap();
        let end_coord = self.grid.find_item(&(end as u32)).unwrap();
        let manhattan = start_coord.get_manhattan_distance(end_coord);

        let paths: Vec<Vec<Coordinate>> = all_simple_paths::<Vec<_>, _>(
            &self.graph,
            start_coord,
            end_coord,
            manhattan - 1,
            Some(manhattan - 1),
        )
        .collect();

        let mut direction_lists = Vec::new();

        for path in paths {
            direction_lists.push(
                path.iter()
                    .tuple_windows()
                    .map(|(coord_1, coord_2)| {
                        KeypadDirection::from(coord_1.get_direction_to(coord_2).unwrap())
                    })
                    .collect(),
            );
        }

        direction_lists
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum KeypadDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    A = 4,
}

impl From<Direction> for KeypadDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self::Up,
            Direction::South => Self::Down,
            Direction::East => Self::Right,
            Direction::West => Self::Left,
            _ => panic!(),
        }
    }
}

impl Display for KeypadDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeypadDirection::Up => write!(f, "^"),
            KeypadDirection::Down => write!(f, "v"),
            KeypadDirection::Left => write!(f, "<"),
            KeypadDirection::Right => write!(f, ">"),
            KeypadDirection::A => write!(f, "A"),
        }
    }
}

fn get_directional_keypad() -> Vec<Coordinate> {
    let up_coord = Coordinate::new(1, 0);
    let down_coord = Coordinate::new(1, 1);
    let left_coord = Coordinate::new(0, 1);
    let right_coord = Coordinate::new(2, 1);
    let a_coord = Coordinate::new(2, 0);

    Vec::from([up_coord, down_coord, left_coord, right_coord, a_coord])
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(&TESTINPUT), 126384);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(&TESTINPUT), 5);
    }

    #[test]
    fn num_keypad_get_paths_test() {
        let keypad = NumericKeypad::new();
        let paths = keypad.get_paths(0, 7);
        println!("{paths:#?}");
        assert_eq!(paths.len(), 3);
    }

    #[test]
    fn dir_keypad_get_paths_test() {
        let dir_keypad = DirectionalKeypad::new();
        let paths = dir_keypad.get_paths(KeypadDirection::A, KeypadDirection::Left);
        assert_eq!(paths.len(), 2);
    }
}
