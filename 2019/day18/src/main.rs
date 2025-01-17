use aoc_lib::{Coordinate, Direction, Grid, Visited};
use bitvec::prelude::*;
use petgraph::{algo::dijkstra, prelude::UnGraphMap};
use rustc_hash::FxHashMap;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    mem::discriminant,
    ops::{Shl, Shr},
};

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
    let mut distances = Vec::default();

    let key_locations = get_key_locations(grid);
    for key_location in key_locations.iter() {
        distances.push(dijkstra(&graph, *key_location, None, |_| 1));
    }

    let mut visited = grid.create_visited_list();
    visited.set(&current_coordinate, true);
    let mut keys_required = vec![0; key_locations.len()];
    let mut current_doors = 0;
    let mut keys_visited = bitvec![0; key_locations.len()];

    get_key_requirements(
        grid,
        current_coordinate,
        &mut visited,
        &mut keys_required,
        &mut current_doors,
        &mut keys_visited,
    );

    println!("{keys_required:?}");
    let mut states = FxHashMap::default();
    states.insert((0, current_coordinate), 0);

    find_shortest_path_to_all_keys(&distances, &key_locations, &keys_required, &states)
}

fn part_2(grid: &mut Grid<CellType>) -> usize {
    // Update to 4 vaults instead of 1
    update_grid(grid);

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


    let start_points: Vec<Coordinate> = grid.find_all(&CellType::Entrance).collect();

    // Find distances from keys to all other points
    let mut distances = Vec::default();
    let key_locations = get_key_locations(grid);
    for key_location in key_locations.iter() {
        distances.push(dijkstra(&graph, *key_location, None, |_| 1));
    }

    let mut vault_keys = Vec::new();
    let mut keys_required = vec![0; key_locations.len()];

    for start_point in start_points.iter() {
        //let mut vault_keys_required = FxHashMap::default();
        let mut visited = grid.create_visited_list();
        let mut keys_visited = bitvec![0; key_locations.len()];
        visited.set(start_point, true);
        get_key_requirements(
            grid,
            *start_point,
            &mut visited,
            &mut keys_required,
            &mut 0,
            &mut keys_visited,
        );
        vault_keys.push(keys_visited);
    }

    let mut states = FxHashMap::default();
    let start_hashed = hash_coords(&start_points);
    states.insert((0, start_hashed), 0);
    find_shortest_path_to_all_keys_robots(
        &distances,
        &key_locations,
        &keys_required,
        &vault_keys,
        states,
    )
}

// Update the grid for part 2
// Replace .../.@./... with @#@/###/@#@
fn update_grid(grid: &mut Grid<CellType>) {
    let current_coordinate = grid.find_item(&CellType::Entrance).unwrap();
    grid.set(current_coordinate, CellType::Wall);
    for direction in Direction::get_cardinals() {
        if let Some(neighbour) = current_coordinate.get_next_in_direction(direction) {
            grid.set(neighbour, CellType::Wall);
        }
    }

    for direction in Direction::get_intercardinals() {
        if let Some(neighbour) = current_coordinate.get_next_in_direction(direction) {
            grid.set(neighbour, CellType::Entrance);
        }
    }
}

// Does DFS, storing the keys required to obtain each key in keys_required
fn get_key_requirements(
    grid: &Grid<CellType>,
    current_location: Coordinate,
    visited: &mut Visited,
    keys_required: &mut Vec<usize>,
    current_doors: &mut usize,
    keys_visited: &mut BitVec,
) {
    let current_value = grid.get(current_location);

    match current_value {
        CellType::Door(door) => {
            *current_doors += door;
        }
        CellType::Key(key) => {
            keys_required[key.ilog2() as usize] = *current_doors;
            keys_visited.set(key.ilog2() as usize, true);
        }
        _ => {}
    }

    for neighbour in grid
        .get_cardinal_neighbours(current_location)
        .filter(|neighbour| neighbour.value != &CellType::Wall)
    {
        if !visited.set(&neighbour.location, true) {
            get_key_requirements(
                grid,
                neighbour.location,
                visited,
                keys_required,
                current_doors,
                keys_visited,
            );
        }
    }

    if let CellType::Door(door) = current_value {
        *current_doors -= door;
    }
}

// Performs breadth first search
// Uses states to ensure no redundant paths are searched
fn find_shortest_path_to_all_keys(
    distances: &Vec<HashMap<Coordinate, usize>>,
    key_locations: &Vec<Coordinate>,
    keys_required: &Vec<usize>,
    states: &FxHashMap<(usize, Coordinate), usize>,
) -> usize {
    // Initialise next states
    let mut next_states: FxHashMap<(usize, Coordinate), usize> = FxHashMap::default();

    for ((keys_collected, current_location), distance) in states {
        let mut possible_next_keys = keys_required
            .iter()
            .enumerate()
            .filter(|(key, requirements)| {
                (*keys_collected & 2usize.pow((*key).try_into().unwrap()) == 0)
                    && (*requirements & *keys_collected) == **requirements
            })
            .map(|(key, _)| 2usize.pow((key).try_into().unwrap()))
            .peekable();

        // If there are no more keys to pick up, then this is the base case - all states have the same length, we have all keys
        // Return the minimum path length from amongst the states
        if possible_next_keys.peek().is_none() {
            return *states.iter().map(|x| x.1).min().unwrap();
        }

        // Find potential next keys
        for key in possible_next_keys {
            // Get the location of the key and calculate the path len
            let key_location = key_locations[key.ilog2() as usize];
            let new_path_len = distance + distances[key.ilog2() as usize].get(&current_location).unwrap();

            // Create the new state with the key inserted
            let new_state = (keys_collected + key, key_location);

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
    find_shortest_path_to_all_keys(distances, key_locations, keys_required, &next_states)
}

// Performs breadth first search
// Uses states to ensure no redundant paths are searched
fn find_shortest_path_to_all_keys_robots(
    distances: &Vec<HashMap<Coordinate, usize>>,
    key_locations: &Vec<Coordinate>,
    keys_required: &Vec<usize>,
    vault_keys: &Vec<BitVec>,
    mut states: FxHashMap<(usize, u64), usize>,
) -> usize {
    // Initialise next states
    let mut next_states: FxHashMap<(usize, u64), usize> = FxHashMap::default();

    for ((keys_collected, current_locations), distance) in states.iter_mut() {
        let mut possible_next_keys = keys_required
            .iter()
            .enumerate()
            .filter(|(key, requirements)| {
                (*keys_collected & 2usize.pow((*key).try_into().unwrap()) == 0)
                    && (*requirements & *keys_collected) == **requirements
            })
            .map(|(key, _)| 2usize.pow((key).try_into().unwrap()))
            .peekable();

        // If there are no more keys to pick up, then this is the base case - all states have the same length, we have all keys
        // Return the minimum path length from amongst the states
        if possible_next_keys.peek().is_none() {
            return *states.iter().map(|x| x.1).min().unwrap();
        }

        // Find potential next keys
        for key in possible_next_keys {
            // Get the location of the key and calculate the path len
            let key_location = key_locations[key.ilog2() as usize];

            // Find which robot to move
            let robot_number = vault_keys
                .iter()
                .enumerate()
                .find_map(|(robot_number, keys_in_vault)| {
                    if *keys_in_vault.get::<usize>(key.ilog2() as usize).unwrap() {
                        Some(robot_number)
                    } else {
                        None
                    }
                })
                .unwrap();

            let current_location = get_coord_from_hash(*current_locations, robot_number);
            let new_path_len = *distance + distances[key.ilog2() as usize].get(&current_location).unwrap();

            // Create the new state with the key inserted
            let new_collected = keys_collected + key;
            let new_locations = set_index_in_hash(*current_locations, key_location, robot_number);

            let new_state = (new_collected, new_locations);

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
    find_shortest_path_to_all_keys_robots(
        distances,
        key_locations,
        keys_required,
        vault_keys,
        next_states,
    )
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

// Finds all the keys in the grid and returns a vector of them in order
// Can be indexed into with: locations[key.ilog2()]
fn get_key_locations(grid: &Grid<CellType>) -> Vec<Coordinate> {
    let mut key_locations: BTreeMap<usize, Coordinate> = BTreeMap::new();

    let initial_key_locations =
        grid.find_all_by(|x| discriminant(x) == discriminant(&CellType::Key(1)));

    for key_location in initial_key_locations {
        let value = match grid.get(key_location) {
            CellType::Key(key_value) => *key_value,
            _ => panic!(),
        };
        key_locations.insert(value, key_location);
    }

    key_locations.values().copied().collect()
}

// We know that the grids in the input are 81x81
// And the tests are all smaller
// We can compress 4 coordinates into 8 * 7 bit numbers = 1 64 bit number
fn hash_coords(coordinates: &[Coordinate]) -> u64 {
    assert_eq!(coordinates.len(), 4);
    let mut output = 0;
    for coordinate in coordinates {
        output = output.shl(14);
        output += coordinate.x.shl(7);
        output += coordinate.y;
    }

    output.try_into().unwrap()
}

// Recover the coordinate at the specified index from the hash
fn get_coord_from_hash(hashed: u64, index: usize) -> Coordinate {
    Coordinate {
        x: (hashed.shr(7 + 14 * (3 - index)) % 128).try_into().unwrap(),
        y: (hashed.shr(14 * (3 - index)) % 128).try_into().unwrap(),
    }
}

// Change a coordinate at the specified position in the hash
fn set_index_in_hash(hashed: u64, coord: Coordinate, index: usize) -> u64 {
    // Set the bits for that index to 0 ANDing with a mask
    // Calculate the mask by taking 14 ones, left shift them to the correct location
    let mask: u64 = !(0x3fffu64.shl(14 * (3 - index)));
    let interim = hashed & mask;

    // Calculate the hashed coordinate value and left shift it to the correct place
    let coord_value: u64 = (coord.x.shl(7) as u64 + coord.y as u64).shl(14 * (3 - index));

    interim + coord_value
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
enum CellType {
    #[default]
    Empty,
    Door(usize),
    Key(usize),
    Entrance,
    Wall,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            '@' => Self::Entrance,
            'A'..='Z' => Self::Door(1 << (value.to_ascii_lowercase() as usize - 97)),
            'a'..='z' => Self::Key(1 << (value as usize - 97)),
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
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 136);
    }

    #[test]
    fn part_2_test() {
        let mut input = parse_input(TESTINPUT2);
        assert_eq!(part_2(&mut input), 24);
    }

    #[test]
    fn coord_hash_test() {
        let coord1 = Coordinate::new(1, 2);
        let coord2 = Coordinate::new(3, 4);
        let coord3 = Coordinate::new(5, 6);
        let coord4 = Coordinate::new(7, 8);

        let hashed_coords = hash_coords(&[coord1, coord2, coord3, coord4]);
        assert_eq!(get_coord_from_hash(hashed_coords, 0), coord1);
        assert_eq!(get_coord_from_hash(hashed_coords, 1), coord2);
        assert_eq!(get_coord_from_hash(hashed_coords, 2), coord3);
        assert_eq!(get_coord_from_hash(hashed_coords, 3), coord4);
    }
}
