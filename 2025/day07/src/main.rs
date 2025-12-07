use std::collections::{HashSet, VecDeque};

use aoc_lib::{Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(grid: &Grid<CellType>) -> usize {
    let mut hit_splitters = HashSet::new();
    let start_point = grid.find_item(&CellType::Start).unwrap();

    let mut laser_starts = VecDeque::new();
    let mut scanned = HashSet::new();
    laser_starts.push_back(start_point);
    let mut current_point;

    while !laser_starts.is_empty() {
        current_point = laser_starts.pop_front().unwrap();

        if let Some(south_splitter) = grid
            .view_from(&current_point, Direction::South)
            .find(|x| x.value == &CellType::Splitter)
        {
            hit_splitters.insert(south_splitter.location);

            if let Some(west_split) = grid.get_neighbour(south_splitter.location, Direction::West)
                && !scanned.contains(&west_split.location)
            {
                laser_starts.push_back(west_split.location);
                scanned.insert(west_split.location);
            }
            if let Some(east_split) = grid.get_neighbour(south_splitter.location, Direction::East)
                && !scanned.contains(&east_split.location)
            {
                laser_starts.push_back(east_split.location);
                scanned.insert(east_split.location);
            }
        }
    }

    hit_splitters.len()
}

fn part_2(input: &Grid<CellType>) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
enum CellType {
    Start,
    #[default]
    Empty,
    Splitter,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            'S' => CellType::Start,
            '.' => CellType::Empty,
            '^' => CellType::Splitter,
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
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
