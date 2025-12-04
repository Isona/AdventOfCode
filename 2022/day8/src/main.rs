use std::collections::HashSet;

use aoc_lib::{Grid, CARDINALS};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&mut input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &mut Grid<i32>) -> usize {
    // Horizontal, left to right
    let mut visible = HashSet::new();

    for row_index in 0..input.get_height() {
        let row: Vec<aoc_lib::Coordinate> = input.get_row(row_index).collect();
        let mut visible_height = -1;

        for coord in row.iter().copied() {
            let current_size = *input.get(coord);
            if current_size > visible_height {
                visible_height = current_size;
                visible.insert(coord);
            }
        }

        visible_height = -1;
        for coord in row.iter().rev().copied() {
            let current_size = *input.get(coord);
            if current_size > visible_height {
                visible_height = current_size;
                visible.insert(coord);
            }
        }
    }

    for column_index in 0..input.get_width() {
        let column: Vec<aoc_lib::Coordinate> = input.get_column(column_index).collect();
        let mut visible_height = -1;

        for coord in column.iter().copied() {
            let current_size = *input.get(coord);
            if current_size > visible_height {
                visible_height = current_size;
                visible.insert(coord);
            }
        }

        visible_height = -1;
        for coord in column.iter().rev().copied() {
            let current_size = *input.get(coord);
            if current_size > visible_height {
                visible_height = current_size;
                visible.insert(coord);
            }
        }
    }

    visible.len()
}

fn part_2(input: &Grid<i32>) -> usize {
    let mut highest_view_score = 0;
    'main_loop: for (current_coord, current_tree) in input.indexed_iter() {
        let mut current_score = 1;
        for view_direction in CARDINALS {
            let mut view_coord = current_coord;
            let mut view_distance = 0;
            while let Some(neighbour) = input.get_neighbour(view_coord, view_direction) {
                view_coord = neighbour.location;
                view_distance += 1;
                if neighbour.value >= current_tree {
                    break;
                }
            }

            if view_distance == 0 {
                continue 'main_loop;
            }
            current_score *= view_distance;
        }

        highest_view_score = highest_view_score.max(current_score);
    }

    highest_view_score
}

fn parse_input(input: &str) -> Grid<i32> {
    let mut grid = Grid::new();
    for line in input.lines() {
        let new_row = line
            .chars()
            .map(|y| y.to_digit(10).unwrap() as i32)
            .collect();

        grid.push_row(new_row);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let mut input = parse_input(TESTINPUT);
        assert_eq!(part_1(&mut input), 21);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 8);
    }
}
