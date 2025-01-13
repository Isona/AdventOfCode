use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
    mem::discriminant,
};

use aoc_lib::{Coordinate, Grid};
use petgraph::{algo::dijkstra, prelude::UnGraphMap};

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
    // Make a graph
    let mut graph = UnGraphMap::new();
    for coordinate in grid.find_all_by(|x| x != &CellType::Wall) {
        for neighbour in grid
            .get_cardinal_neighbours(coordinate)
            .filter(|neighbour| neighbour.value != &CellType::Wall)
        {
            graph.add_edge(coordinate, neighbour.location, 1);
        }
    }

    // Find the start location
    let current_coordinate = grid.find_item(&CellType::Entrance).unwrap();

    // Find the distances from the start point and all keys to all other points
    let mut distances = HashMap::new();
    distances.insert(
        current_coordinate,
        dijkstra(&graph, current_coordinate, None, |_| 1),
    );

    for key in grid.find_all_by(|x| discriminant(x) == discriminant(&CellType::Key('a'))) {
        distances.insert(key, dijkstra(&graph, key, None, |_| 1));
    }

    let mut visited = HashSet::from([current_coordinate]);
    let mut keys_required = HashMap::new();
    let mut current_doors = BTreeSet::new();

    get_key_requirements(
        grid,
        current_coordinate,
        &mut visited,
        &mut keys_required,
        &mut current_doors,
    );

    println!("{keys_required:?}");

    let mut states = HashMap::new();
    states.insert((BTreeSet::new(), current_coordinate), 0);

    find_shortest_path_to_all_keys(grid, &distances, &keys_required, &states)
}

fn part_2(input: &Grid<CellType>) -> u64 {
    todo!();
}

// Does DFS, storing the keys required to obtain each key in keys_required
fn get_key_requirements(
    grid: &Grid<CellType>,
    current_location: Coordinate,
    visited: &mut HashSet<Coordinate>,
    keys_required: &mut HashMap<char, BTreeSet<char>>,
    current_doors: &mut BTreeSet<char>,
) {
    let current_value = grid.get(current_location);

    match current_value {
        CellType::Door(door) => {
            current_doors.insert(*door);
        }
        CellType::Key(key) => {
            keys_required.insert(*key, current_doors.clone());
        }
        _ => {}
    }

    for neighbour in grid
        .get_cardinal_neighbours(current_location)
        .filter(|neighbour| neighbour.value != &CellType::Wall)
    {
        if visited.insert(neighbour.location) {
            get_key_requirements(
                grid,
                neighbour.location,
                visited,
                keys_required,
                current_doors,
            );
        }
    }

    if let CellType::Door(door) = current_value {
        current_doors.remove(door);
    }
}

// Performs breadth first search
// Uses states to ensure no redundant paths are searched
fn find_shortest_path_to_all_keys(
    grid: &Grid<CellType>,
    distances: &HashMap<Coordinate, HashMap<Coordinate, usize>>,
    keys_required: &HashMap<char, BTreeSet<char>>,
    states: &HashMap<(BTreeSet<char>, Coordinate), usize>,
) -> usize {
    // Initialise next states
    let mut next_states: HashMap<(BTreeSet<char>, Coordinate), usize> = HashMap::new();

    for ((keys_collected, current_location), distance) in states {
        let possible_next_keys: Vec<char> = keys_required
            .iter()
            .filter(|(key, requirements)| {
                !keys_collected.contains(key) && requirements.is_subset(keys_collected)
            })
            .map(|(key, _)| *key)
            .collect();

        // If there are no more keys to pick up, then this is the base case - all states have the same length, we have all keys
        // Return the minimum path length from amongst the states
        if possible_next_keys.is_empty() {
            return *states.iter().map(|x| x.1).min().unwrap();
        }

        // Get the dijkstra results for the current location
        let distances_from_current = distances.get(current_location).unwrap();

        // Find potential next keys
        for key in possible_next_keys {
            // Get the location of the key and calculate the path len
            let key_location = grid.find_item(&CellType::Key(key)).unwrap();
            let new_path_len = distance + distances_from_current.get(&key_location).unwrap();

            // Create the new state with the key inserted
            let mut new_state = keys_collected.clone();
            new_state.insert(key);
            let new_state = (new_state, key_location);

            // Add the new state to the table for the next iteration
            // Keep the minimum path length if it already exists
            if let Some(other_state_dist) = next_states.get(&new_state) {
                if new_path_len < *other_state_dist {
                    next_states.insert(new_state, new_path_len);
                }
            } else {
                next_states.insert(new_state, new_path_len);
            }
        }
    }

    // Iterate using the next set of states
    find_shortest_path_to_all_keys(grid, distances, keys_required, &next_states)
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
            'A'..='Z' => Self::Door(value.to_ascii_lowercase()),
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
