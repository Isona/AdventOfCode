use core::fmt;
use std::collections::HashMap;

use aoc_lib::{Coordinate, Grid};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input, 20, 100);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &Grid<CellType>) -> usize {
    let distance_from_start = get_distances_from_start(input);
    let mut cheats = HashMap::new();

    for wall_coord in input.find_all(&CellType::Wall) {
        // Get all non-wall neighbours of a wall
        let neighbour_permutations = input
            .get_cardinal_neighbours(wall_coord)
            .filter(|neighbour| neighbour.value != &CellType::Wall)
            .permutations(2);

        for neighbour_pair in neighbour_permutations {
            let neighbour_1_distance = distance_from_start
                .get(&neighbour_pair[0].location)
                .unwrap();
            let neighbour_2_distance = distance_from_start
                .get(&neighbour_pair[1].location)
                .unwrap();

            if neighbour_1_distance > neighbour_2_distance {
                continue;
            }
            let cheat_value = neighbour_2_distance - neighbour_1_distance - 2;
            if cheat_value >= 100 {
                cheats.insert(
                    (neighbour_pair[0].location, neighbour_pair[1].location),
                    cheat_value,
                );
            }
        }
    }

    cheats.len()
}

fn part_2(input: &Grid<CellType>, max_cheat_size: i32, cheat_threshold: i32) -> u64 {
    let distance_from_start = get_distances_from_start(input);
    // let mut cheats = HashMap::new();
    let mut cheat_count = 0;

    for ((first_coord, _), (second_coord, _)) in input
        .indexed_iter()
        .filter(|(_, value)| **value != CellType::Wall)
        .tuple_combinations()
    {
        let manhattan: i32 = first_coord
            .get_manhattan_distance(second_coord)
            .try_into()
            .unwrap();
        if manhattan > max_cheat_size {
            continue;
        }

        let first_dist = distance_from_start.get(&first_coord).unwrap();
        let second_dist = distance_from_start.get(&second_coord).unwrap();
        let cheat_value = (first_dist - second_dist).abs() - manhattan;
        if cheat_value >= cheat_threshold {
            cheat_count += 1;
        }
    }

    cheat_count
}

fn get_distances_from_start(grid: &Grid<CellType>) -> HashMap<Coordinate, i32> {
    let mut distance_from_start = HashMap::new();
    let mut current_position = grid.find_item(&CellType::Start).unwrap();
    let mut current_distance = 0;

    while grid.get(current_position) != &CellType::End {
        distance_from_start.insert(current_position, current_distance);

        current_distance += 1;
        current_position = grid
            .get_cardinal_neighbours(current_position)
            .filter(|neighbour| {
                neighbour.value != &CellType::Wall
                    && !distance_from_start.contains_key(&neighbour.location)
            })
            .map(|neighbour| neighbour.location)
            .next()
            .unwrap();
    }

    distance_from_start.insert(current_position, current_distance);
    distance_from_start
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();

    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum CellType {
    Wall,
    #[default]
    Empty,
    Start,
    End,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!(),
        }
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellType::Wall => write!(f, "#"),
            CellType::Empty => write!(f, "."),
            CellType::Start => write!(f, "S"),
            CellType::End => write!(f, "E"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input, 20, 50), 285);
    }
}
