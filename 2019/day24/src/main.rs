use rustc_hash::FxHashSet as HashSet;
use std::{fmt::Display, mem};

use aoc_lib::Direction;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const GRIDSIZE: usize = 25;

fn main() {
    let input = BitGrid::new(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input, 200);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &BitGrid) -> usize {
    let mut previous_patterns = HashSet::default();
    previous_patterns.insert(*input);
    let mut current_input = *input;

    loop {
        current_input = current_input.do_iteration();
        if !previous_patterns.insert(current_input) {
            return current_input.get_score();
        }
    }
}

fn part_2(input: &BitGrid, iteration_count: usize) -> usize {
    // Create working layer lists - we know that 249 is ok
    // It takes 2 iterations to reach a new layer
    let mut layers = Vec::from([BitGrid::new_default(); 249]);
    let mut new_layers = Vec::from([BitGrid::new_default(); 249]);
    // Set the center layer to be the start input
    layers[124] = *input;

    // 200 iterations
    for _ in 0..iteration_count {
        // Loop over sets of outer, current and inner layers
        // We don't deal with the first and last layers - that's ok, they're always empty
        for ((_, outer_layer), (index, layer), (_, inner_layer)) in
            layers.iter().enumerate().tuple_windows()
        {
            // If all 3 layers are empty, skip this one
            if outer_layer.is_zero() && layer.is_zero() && inner_layer.is_zero() {
                continue;
            }

            new_layers[index] = layer.do_iteration_part_2(outer_layer, inner_layer);
        }
        // Swap the layer lists to avoid reallocating the vector
        mem::swap(&mut layers, &mut new_layers);
    }

    layers
        .iter()
        .filter(|x| !x.is_zero())
        .map(BitGrid::get_bug_count)
        .sum()
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct BitGrid {
    grid: usize,
}

impl BitGrid {
    fn new(input: &str) -> Self {
        let mut grid = 0;
        let mut current_index = 0;
        for character in input.chars() {
            if !character.is_ascii_punctuation() {
                continue;
            } else if character == '#' {
                grid += 2 << (GRIDSIZE - 1 - current_index);
            }

            current_index += 1;
        }

        Self { grid }
    }

    fn new_default() -> Self {
        Self { grid: 0 }
    }

    fn is_zero(&self) -> bool {
        self.grid == 0
    }

    fn get(&self, index: usize) -> bool {
        self.grid & (2 << (GRIDSIZE - 1 - index)) != 0
    }

    fn get_as_usize(&self, index: usize) -> usize {
        self.grid >> (GRIDSIZE - index) & 1
    }

    fn set(&mut self, index: usize, value: bool) {
        if !value && self.get(index) {
            self.grid -= 2 << (GRIDSIZE - 1 - index);
        } else if value {
            self.grid += 2 << (GRIDSIZE - 1 - index);
        }
    }

    fn get_score(&mut self) -> usize {
        let mut score = 0;
        for index in 0..GRIDSIZE {
            if self.get(index) {
                score += 1 << index;
            }
        }

        score
    }

    fn get_bug_count(&self) -> usize {
        (0..GRIDSIZE)
            .filter_map(|i| self.get(i).then_some(0))
            .count()
    }

    fn do_iteration(&self) -> Self {
        let mut new_grid = BitGrid::new_default();
        for i in 0..GRIDSIZE {
            let neighbouring_bugs: usize = self.get_cardinal_neighbours(i).sum();

            // It's only a bug if it's already a bug and neighbours = 1
            // or if it's empty and there are exactly 1 or 2 adjacent bugs
            if (self.get(i) && neighbouring_bugs == 1)
                || (!self.get(i) && (neighbouring_bugs == 1 || neighbouring_bugs == 2))
            {
                new_grid.set(i, true);
            }
        }

        new_grid
    }

    fn do_iteration_part_2(&self, outer_layer: &BitGrid, inner_layer: &BitGrid) -> Self {
        let mut new_grid = BitGrid::new_default();
        for i in 0..GRIDSIZE {
            // Skip the center, it's always empty
            if i == 12 {
                continue;
            }

            let neighbouring_bugs = self.get_neighbouring_bug_count(i, outer_layer, inner_layer);
            // It's only a bug if it's already a bug and neighbours = 1
            // or if it's empty and there are exactly 1 or 2 adjacent bugs
            if (self.get(i) && neighbouring_bugs == 1)
                || (!self.get(i) && (neighbouring_bugs == 1 || neighbouring_bugs == 2))
            {
                new_grid.set(i, true);
            }
        }

        new_grid
    }

    pub fn get_cardinal_neighbours(&self, index: usize) -> impl Iterator<Item = usize> {
        Direction::get_cardinals().filter_map(move |direction| self.get_neighbour(index, direction))
    }

    pub fn get_neighbour(&self, index: usize, direction: Direction) -> Option<usize> {
        let x = index % 5;
        let y = index / 5;

        let new_x = match direction {
            Direction::East => {
                if x != 4 {
                    x + 1
                } else {
                    return None;
                }
            }
            Direction::West => {
                if x != 0 {
                    x - 1
                } else {
                    return None;
                }
            }
            _ => x,
        };

        let new_y = match direction {
            Direction::North => {
                if y != 0 {
                    y - 1
                } else {
                    return None;
                }
            }
            Direction::South => {
                if y != 4 {
                    y + 1
                } else {
                    return None;
                }
            }
            _ => y,
        };
        Some(self.get_as_usize(new_y * 5 + new_x))
    }

    pub fn get_neighbouring_bug_count(
        &self,
        index: usize,
        outer_layer: &BitGrid,
        inner_layer: &BitGrid,
    ) -> usize {
        Direction::get_cardinals()
            .map(move |direction| {
                self.get_bug_count_in_direction(index, direction, outer_layer, inner_layer)
            })
            .sum()
    }

    pub fn get_bug_count_in_direction(
        &self,
        index: usize,
        direction: Direction,
        outer_layer: &BitGrid,
        inner_layer: &BitGrid,
    ) -> usize {
        let x = index % 5;
        let y = index / 5;
        let new_x = match direction {
            Direction::East => {
                if x == 4 {
                    return outer_layer.get_as_usize(13);
                }
                x + 1
            }
            Direction::West => {
                if x == 0 {
                    return outer_layer.get_as_usize(11);
                }

                x - 1
            }
            _ => x,
        };

        let new_y = match direction {
            Direction::North => {
                if y == 0 {
                    return outer_layer.get_as_usize(7);
                }
                y - 1
            }
            Direction::South => {
                if y == 4 {
                    return outer_layer.get_as_usize(17);
                }
                y + 1
            }
            _ => y,
        };

        let neighbour_index = new_y * 5 + new_x;
        if neighbour_index == 12 {
            inner_layer.get_outer_edge_count(direction)
        } else {
            self.get_as_usize(neighbour_index)
        }
    }

    pub fn get_outer_edge_count(&self, direction: Direction) -> usize {
        match direction {
            Direction::South => [0, 1, 2, 3, 4]
                .into_iter()
                .map(|i| self.get_as_usize(i))
                .sum(),
            Direction::North => [20, 21, 22, 23, 24]
                .into_iter()
                .map(|i| self.get_as_usize(i))
                .sum(),
            Direction::West => [4, 9, 14, 19, 24]
                .into_iter()
                .map(|i| self.get_as_usize(i))
                .sum(),
            Direction::East => [0, 5, 10, 15, 20]
                .into_iter()
                .map(|i| self.get_as_usize(i))
                .sum(),
            _ => panic!(),
        }
    }
}

impl Display for BitGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for index in 0..GRIDSIZE {
            if index % 5 == 0 {
                output.push('\n');
            }

            if self.get(index) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = BitGrid::new(TESTINPUT);
        println!("{}", input);
        // let input = input.do_iteration();
        // println!("{}", input);
        assert_eq!(part_1(&input), 2129920);
    }

    #[test]
    fn part_2_test() {
        let input = BitGrid::new(TESTINPUT);
        assert_eq!(part_2(&input, 10), 99);
    }
}
