use aoc_lib::{Coordinate, Grid};
use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &Grid<CellType>) -> u64 {
    let mut reachable = 0;
    for (current_coord, _) in input.indexed_iter().filter(|x| x.1 == &CellType::Paper) {
        if input
            .get_all_neighbours(current_coord)
            .filter(|x| x.value == &CellType::Paper)
            .count()
            < 4
        {
            reachable += 1;
        }
    }

    reachable
}

fn part_2(input: &mut Grid<CellType>) -> u64 {
    let mut reachable_count = 0;
    let mut found_more_to_remove = true;
    let mut current_reachable = VecDeque::new();
    let mut remaining_rolls: VecDeque<Coordinate> = input
        .indexed_iter()
        .filter(|x| x.1 == &CellType::Paper)
        .map(|x| x.0)
        .collect();

    let mut next_remaining = VecDeque::new();

    while found_more_to_remove {
        found_more_to_remove = false;
        for current_coord in remaining_rolls.drain(0..remaining_rolls.len()) {
            if input
                .get_all_neighbours(current_coord)
                .filter(|x| x.value == &CellType::Paper)
                .count()
                < 4
            {
                current_reachable.push_back(current_coord);
                reachable_count += 1;
                found_more_to_remove = true;
            } else {
                next_remaining.push_back(current_coord);
            }
        }

        for removable_paper in current_reachable.drain(0..current_reachable.len()) {
            input.set(removable_paper, CellType::Empty);
        }

        std::mem::swap(&mut remaining_rolls, &mut next_remaining);
    }

    reachable_count
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
enum CellType {
    #[default]
    Empty,
    Paper,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn part_2_test() {
        let mut input = parse_input(TESTINPUT);
        assert_eq!(part_2(&mut input), 43);
    }
}
