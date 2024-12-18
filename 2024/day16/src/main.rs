use std::collections::{HashSet, VecDeque};

use aoc_lib::{Coordinate, Direction, Grid};
use petgraph::{Direction::Incoming, algo::dijkstra, prelude::DiGraphMap};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = both_parts(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn both_parts(input: &Grid<CellType>) -> (i32, usize) {
    // Create a Directed Graph
    let mut graph = DiGraphMap::new();

    // Add the start coordinate and its edges
    let start = input.find_item(&CellType::Start).unwrap();
    add_coord_edges(input, &mut graph, &start);

    // Add the end coordinate and its edges
    let end = input.find_item(&CellType::End).unwrap();
    add_coord_edges(input, &mut graph, &end);

    // Add all the empty coordinates
    for coordinate in input.find_all(&CellType::Empty) {
        add_coord_edges(input, &mut graph, &coordinate);
    }

    // Calculate the distance from start to every point
    let augmented_start = AugmentedCoord::new(&start, &Direction::East);
    let dijkstra = dijkstra(&graph, augmented_start, None, |x| *x.2);

    // Loop through the 4 versions of the end coordinate, getting the one corresponding to the minimum and its path length
    let (augmented_end, shortest_path_length) = Direction::get_cardinals()
        .map(|direction| {
            let augmented_end = AugmentedCoord::new(&end, &direction);
            (
                augmented_end,
                dijkstra
                    .get(&AugmentedCoord::new(&end, &direction))
                    .unwrap(),
            )
        })
        .min_by_key(|x| x.1)
        .unwrap();

    // Make a hashset to hold the coordinates we visit and a list of augmented coordinates to search starting from the end
    let mut visited_coords = HashSet::from([augmented_start.coordinate, augmented_end.coordinate]);
    let mut to_search = VecDeque::from([(augmented_end, shortest_path_length)]);

    // Iterate backwards along the path, adding a location to the queue if this is a valid shortest path
    while let Some((current_node, current_path_length)) = to_search.pop_front() {
        visited_coords.insert(current_node.coordinate);
        for neighbour in graph.neighbors_directed(current_node, Incoming) {
            let neighbour_shortest_path = dijkstra.get(&neighbour).unwrap();
            let weight = graph.edge_weight(neighbour, current_node).unwrap();
            if current_path_length - weight == *neighbour_shortest_path {
                to_search.push_back((neighbour, neighbour_shortest_path));
            }
        }
    }

    (*shortest_path_length, visited_coords.len())
}

fn add_coord_edges(
    input: &Grid<CellType>,
    graph: &mut DiGraphMap<AugmentedCoord, i32>,
    coordinate: &Coordinate,
) {
    for direction in Direction::get_cardinals() {
        let augmented = AugmentedCoord::new(coordinate, &direction);
        let augmented_left = AugmentedCoord::new(coordinate, &direction.turn_left());
        graph.add_edge(augmented, augmented_left, 1000);
        let augmented_right = AugmentedCoord::new(coordinate, &direction.turn_right());
        graph.add_edge(augmented, augmented_right, 1000);

        if let Some(neighbour) = input.get_neighbour(*coordinate, direction) {
            if neighbour.value == &CellType::Wall {
                continue;
            }
            let augmented_neighbour = AugmentedCoord::new(&neighbour.location, &direction);
            graph.add_edge(augmented, augmented_neighbour, 1);
        }
    }
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();

    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

#[derive(Default, PartialEq, Eq)]
enum CellType {
    Wall,
    #[default]
    Empty,
    Start,
    End,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct AugmentedCoord {
    coordinate: Coordinate,
    direction: Direction,
}

impl AugmentedCoord {
    fn new(coordinate: &Coordinate, direction: &Direction) -> Self {
        AugmentedCoord {
            coordinate: *coordinate,
            direction: *direction,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(both_parts(&input).0, 7036);
    }

    #[test]
    fn part_1_test_2() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(both_parts(&input).0, 11048);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(both_parts(&input).1, 64);
    }
}
