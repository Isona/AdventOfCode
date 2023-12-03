#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let (part_1_answer, part_2_answer) = part_1_and_2(&input);
    println!("Part 1: {part_1_answer}");
    println!("Part 2: {part_2_answer}");
}

fn part_1_and_2(input: &Vec<Vec<char>>) -> (u32, u32) {
    let mut part_1_out = 0;
    let mut part_2_out = 0;
    let mut gear_map: HashMap<(usize, usize), u32> = HashMap::new();

    for x in 0..input.len() {
        let mut part_number = String::new();
        let mut symbol_adjacent = false;
        let mut gear_coords: HashSet<(usize, usize)> = HashSet::new();

        for y in 0..input[0].len() {
            if input[x][y].is_digit(10) {
                part_number.push(input[x][y]);

                for neighbour_x in x.saturating_sub(1)..=x + 1 {
                    for neighbour_y in y.saturating_sub(1)..=y + 1 {
                        if let Some(row) = input.get(neighbour_x)
                            && let Some(neighbour) = row.get(neighbour_y)
                        {
                            if is_symbol(neighbour) {
                                symbol_adjacent = true;

                                if neighbour == &'*' {
                                    gear_coords.insert((neighbour_x, neighbour_y));
                                }
                            }
                        }
                    }
                }
            } else {
                if !part_number.is_empty() && symbol_adjacent {
                    println!("{part_number}");
                    part_1_out += part_number.parse::<u32>().unwrap();
                }

                for &coord in &gear_coords {
                    let part_num_u32 = part_number.parse::<u32>().unwrap();
                    if let Some(other_part_num) = gear_map.get(&coord) {
                        part_2_out += part_num_u32 * other_part_num;
                    } else {
                        gear_map.insert(coord, part_num_u32);
                    }
                }
                part_number.clear();
                symbol_adjacent = false;
                gear_coords.clear();
            }
        }
        if !part_number.is_empty() && symbol_adjacent {
            println!("{part_number}");
            part_1_out += part_number.parse::<u32>().unwrap();
        }
        for &coord in &gear_coords {
            let part_num_u32 = part_number.parse::<u32>().unwrap();
            if let Some(other_part_num) = gear_map.get(&coord) {
                part_2_out += part_num_u32 * other_part_num;
            } else {
                gear_map.insert(coord, part_num_u32);
            }
        }
    }

    (part_1_out, part_2_out)
}

fn is_symbol(input: &char) -> bool {
    !input.is_digit(10) && !(input == &'.')
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1_and_2(&input), (4361, 467835));
    }
}
