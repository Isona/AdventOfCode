#![feature(let_chains)]

use grid::*;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let (part_1_answer, part_2_answer) = part_1_and_2(&input);
    println!("Part 1: {part_1_answer}");
    println!("Part 2: {part_2_answer}");
}

fn part_1_and_2(input: &Grid<char>) -> (u32, u32) {
    let mut part_1_out = 0;
    let mut part_2_out = 0;

    // Hashmap of potential gear coordinates for part 2
    let mut gear_map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut part_number = String::new();
    let mut symbol_adjacent = false;
    let mut gear_coords: HashSet<(usize, usize)> = HashSet::new();

    for ((x, y), value) in input.indexed_iter() {
        if value.is_ascii_digit() {
            part_number.push(*value);

            let neighbours = get_neighbour_coords(x, y);

            for (neighbour_x, neighbour_y) in neighbours {
                if let Some(neighbour) = input.get(neighbour_x, neighbour_y)
                    && is_symbol(neighbour)
                {
                    symbol_adjacent = true;

                    if neighbour == &'*' {
                        gear_coords.insert((neighbour_x, neighbour_y));
                    }
                }
            }

            // If the neighbour to the right either doesn't exist or isn't a number
            // We need to do calculations for both parts and cleanup

            let right_neighbour = input.get(x, y + 1);

            if right_neighbour.is_none() || !right_neighbour.unwrap().is_alphanumeric() {
                // If the part is next to a symbol, parse and add to the part 1 total
                if !part_number.is_empty() && symbol_adjacent {
                    part_1_out += part_number.parse::<u32>().unwrap();
                }

                // For every adjacent gear, if it's in the hashmap then add to the sum
                // Or add the newly found gear to the map
                for &coord in &gear_coords {
                    let part_num_u32 = part_number.parse::<u32>().unwrap();
                    if let Some(other_part_num) = gear_map.get(&coord) {
                        part_2_out += part_num_u32 * other_part_num;
                    } else {
                        gear_map.insert(coord, part_num_u32);
                    }
                }

                // Clear values ready for next part number
                part_number.clear();
                symbol_adjacent = false;
                gear_coords.clear();
            }
        }
    }

    (part_1_out, part_2_out)
}

// Get the 8 surrounding cells
// Doesn't check if indexes are past the ends of the grid
// Won't return negative values or the original cell
fn get_neighbour_coords(x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut neighbours = HashSet::new();

    for neighbour_x in x.saturating_sub(1)..=x + 1 {
        for neighbour_y in y.saturating_sub(1)..=y + 1 {
            if neighbour_x == x && neighbour_y == y {
                continue;
            }
            neighbours.insert((neighbour_x, neighbour_y));
        }
    }

    neighbours
}

fn is_symbol(input: &char) -> bool {
    !input.is_ascii_digit() && input != &'.'
}

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = grid![];
    for line in input.lines() {
        grid.push_row(line.chars().collect())
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_and_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1_and_2(&input), (4361, 467835));
    }

    #[test]
    fn real_input_test() {
        let input = parse_input(INPUT);
        assert_eq!(part_1_and_2(&input), (528799, 84907174));
    }
}
