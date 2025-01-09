use std::{collections::HashSet, fmt::Display, mem::discriminant};

use aoc_lib::{Coordinate, Grid};
use petgraph::{
    prelude::UnGraphMap,
    visit::{Dfs, Visitable, Walker},
};

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

fn part_1(grid: &Grid<CellType>) -> u64 {
    let mut current_coordinate = grid.find_item(&CellType::Entrance).unwrap();

    // Make a graph but exclude all doors
    let mut graph = UnGraphMap::new();
    for coordinate in grid.find_all(&CellType::Empty) {
        for neighbour in grid
            .get_cardinal_neighbours(coordinate)
            .filter(|neighbour| {
                neighbour.value != &CellType::Wall
                    && discriminant(neighbour.value) != discriminant(&CellType::Door('a'))
            })
        {
            graph.add_edge(coordinate, neighbour.location, 1);
        }
    }

    let keys: HashSet<Coordinate> = grid
        .find_all_by(|cell| discriminant(cell) == discriminant(&CellType::Key('a')))
        .collect();

    let doors: HashSet<Coordinate> = grid
        .find_all_by(|cell| discriminant(cell) == discriminant(&CellType::Door('a')))
        .collect();

    let visitable_keys: Vec<CellType> = Dfs::new(&graph, current_coordinate)
        .iter(&graph)
        .filter(|x| keys.contains(x))
        .map(|x| *grid.get(x))
        .collect();

    println!("{visitable_keys:?}");

    todo!()
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

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
enum CellType {
    #[default]
    Empty,
    Door(char),
    Key(char),
    Entrance,
    Wall,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            '@' => Self::Entrance,
            'A'..='Z' => Self::Door(value),
            'a'..='z' => Self::Key(value),
            _ => panic!(),
        }
    }
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellType::Empty => write!(f, "."),
            CellType::Door(door) => write!(f, "{door}"),
            CellType::Key(key) => write!(f, "{key}"),
            CellType::Entrance => write!(f, "@"),
            CellType::Wall => write!(f, " "),
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
        assert_eq!(part_1(&input), 132);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
