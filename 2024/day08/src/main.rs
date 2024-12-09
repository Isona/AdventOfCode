use std::collections::HashSet;

use aoc_lib::{Coordinate, Grid, Vector};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (input, unique_values) = parse_input(INPUT);

    let part_1_answer = part_1(&input, &unique_values);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input, &unique_values);
    println!("Part 2: {part_2_answer}");
}

// Produces too high an input
fn part_1(input: &Grid<char>, unique_values: &HashSet<char>) -> usize {
    let mut antinode_locations: HashSet<Coordinate> = HashSet::new();
    for satellite_type in unique_values {
        if satellite_type == &'.' {
            continue;
        }
        println!("Satellite: {satellite_type}");
        let signal_locations = input.find_all(satellite_type);

        for pair in signal_locations.iter().combinations(2) {
            assert!(pair.len() == 2);
            let difference = Vector::get_difference(pair[0], pair[1]);
            if let Some(subtracted) = pair[0].sub_vec(&difference) {
                if input.is_valid_coord(subtracted) {
                    antinode_locations.insert(subtracted);
                }
            }

            if let Some(added) = pair[1].add_vec(&difference) {
                if input.is_valid_coord(added) {
                    antinode_locations.insert(added);
                }
            }
        }
    }

    antinode_locations.len()
}

fn part_2(input: &Grid<char>, unique_values: &HashSet<char>) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> (Grid<char>, HashSet<char>) {
    let mut grid: Grid<char> = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    let mut unique_values = HashSet::new();
    for line in input.lines() {
        for char in line.chars() {
            unique_values.insert(char);
        }
    }

    (grid, unique_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (input, unique_values) = parse_input(TESTINPUT);
        assert_eq!(part_1(&input, &unique_values), 15);
    }

    #[test]
    fn part_2_test() {
        let (input, unique_values) = parse_input(TESTINPUT);
        assert_eq!(part_2(&input, &unique_values), 5);
    }
}
