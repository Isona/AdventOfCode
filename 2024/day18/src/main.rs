use core::fmt;
use std::fmt::{Display, Formatter};

use aoc_lib::{Coordinate, Grid};
use petgraph::{algo::dijkstra, prelude::UnGraphMap};
const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = solve_maze(&input, 70, 1024);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn solve_maze(input: &[Coordinate], maze_size: usize, blocks_to_place: usize) -> (u64, Coordinate) {
    // Generate a grid with the correct size
    let mut grid = Grid::new();
    for _ in 0..maze_size + 1 {
        grid.push_row(vec![CellType::Empty; maze_size + 1]);
    }

    // Push the first n blocks in the grid as walls
    for coordinate in input.iter().take(blocks_to_place) {
        grid.set(*coordinate, CellType::Wall);
    }

    // Add all the edges between empty cells in the grid to a graph
    let mut graph = UnGraphMap::new();
    for coordinate in grid.find_all(&CellType::Empty) {
        for neighbour in grid.get_cardinal_neighbours(coordinate) {
            if neighbour.value != &CellType::Wall {
                graph.add_edge(coordinate, neighbour.location, 1);
            }
        }
    }

    let start = Coordinate::new(0, 0);
    let end = Coordinate::new(maze_size, maze_size);
    // Get the shortest route for the part 1 answer using dijkstra's algorithm
    let part_1_answer = *dijkstra(&graph, start, Some(end), |x| *x.2)
        .get(&end)
        .unwrap();

    // Loop over the remaining n blocks in the input coords
    // Remove each node, then run dijkstra from start to end - if we don't get a path length this is the answer
    for coordinate in input.iter().skip(blocks_to_place) {
        graph.remove_node(*coordinate);
        if !dijkstra(&graph, start, Some(end), |x| *x.2).contains_key(&end) {
            return (part_1_answer, *coordinate);
        }
    }

    panic!()
}

fn parse_input(input: &str) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();
    for line in input.lines() {
        let mut elements = line.split(',');
        let x = elements.next().unwrap().parse().unwrap();
        let y = elements.next().unwrap().parse().unwrap();
        let coord = Coordinate::new(x, y);

        coordinates.push(coord);
    }

    coordinates
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum CellType {
    #[default]
    Empty,
    Wall,
}

impl Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CellType::Empty => write!(f, "."),
            CellType::Wall => write!(f, "#"),
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
        assert_eq!(solve_maze(&input, 6, 12).0, 22);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(solve_maze(&input, 6, 12).1, Coordinate::new(6, 1));
    }
}
