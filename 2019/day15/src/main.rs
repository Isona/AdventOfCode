use aoc_lib::{Coordinate, Direction, Grid};
use intcode::IntCodePC;
use petgraph::{algo::dijkstra, prelude::UnGraphMap};
use rustc_hash::FxHashMap as HashMap;
use std::fmt::Display;
const INPUT: &str = include_str!("input.txt");
const GRIDSIZE: usize = 40;

fn main() {
    let mut pc = IntCodePC::new(INPUT);

    let start = std::time::Instant::now();
    let (grid, part_1_answer) = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(grid);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> (Grid<CellType>, usize) {
    let mut grid = Grid::new();
    for _ in 0..GRIDSIZE {
        grid.push_row([CellType::Unknown; GRIDSIZE].to_vec());
    }

    let current_coord = Coordinate::new(GRIDSIZE / 2, GRIDSIZE / 2);
    grid.set(current_coord, CellType::Empty);

    let mut visited = HashMap::default();

    dfs(pc, &mut grid, current_coord, 0, &mut visited);

    // println!("{grid}");

    let oxygen: Coordinate = grid.find_item(&CellType::Oxygen).unwrap();
    (grid, *visited.get(&oxygen).unwrap())
}

fn part_2(grid: Grid<CellType>) -> i128 {
    let mut graph = UnGraphMap::new();
    for coordinate in grid.find_all(&CellType::Empty) {
        for neighbour in grid.get_cardinal_neighbours(coordinate) {
            if neighbour.value != &CellType::Wall {
                graph.add_edge(coordinate, neighbour.location, 1);
            }
        }
    }

    let oxygen: Coordinate = grid.find_item(&CellType::Oxygen).unwrap();

    *dijkstra(&graph, oxygen, None, |_| 1)
        .iter()
        .max_by_key(|x| x.1)
        .unwrap()
        .1
}

// Takes a mutable pc and grid, the starting coordinate, and the path taken to get here
// Visited tracks the shortest path
fn dfs(
    pc: &mut IntCodePC,
    grid: &mut Grid<CellType>,
    current_coord: Coordinate,
    path_len: usize,
    visited: &mut HashMap<Coordinate, usize>,
) {
    // Add this node to visited, keeping track of the shortest path
    if let Some(existing_path_len) = visited.get(&current_coord) {
        if path_len < *existing_path_len {
            visited.insert(current_coord, path_len);
        }
    } else {
        visited.insert(current_coord, path_len);
    }

    let mut neighbour_updates = Vec::new();

    // Loop over all the unknown neighbours and get the value from the PC, keep track of cells to update
    for neighbour in grid
        .get_cardinal_neighbours(current_coord)
        .filter(|neighbour| neighbour.value == &CellType::Unknown)
    {
        let neighbour_value = pc_get_neighbour(pc, &neighbour.direction);
        neighbour_updates.push((neighbour.location, neighbour.direction, neighbour_value));
    }

    // Update the neighbours in the grid
    for neighbour_update in &neighbour_updates {
        grid.set(neighbour_update.0, neighbour_update.2);
    }

    for (neighbour_coord, neighbour_direction, neighbour_value) in neighbour_updates {
        // Don't search the neighbour if it's a wall or it's been visited
        if neighbour_value == CellType::Wall || visited.contains_key(&neighbour_coord) {
            continue;
        }

        // Set up the new path, and set up the pc to be at the correct coordinate
        //path.push_back(direction_to_int(&neighbour_direction));
        pc.run_with_input([direction_to_int(&neighbour_direction)].into());
        dfs(pc, grid, neighbour_coord, path_len + 1, visited);

        // Reset the PC to be at the coordinate for this iteration
        //path.pop_back();
        pc.run_with_input([direction_to_int(&neighbour_direction.get_opposite())].into());
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
enum CellType {
    Wall = 0,
    Empty = 1,
    Oxygen = 2,
    #[default]
    Unknown = 3,
}

impl From<i128> for CellType {
    fn from(value: i128) -> Self {
        match value {
            0 => Self::Wall,
            1 => Self::Empty,
            2 => Self::Oxygen,
            _ => panic!(),
        }
    }
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellType::Wall => write!(f, "#"),
            CellType::Empty => write!(f, " "),
            CellType::Oxygen => write!(f, "O"),
            CellType::Unknown => write!(f, "?"),
        }
    }
}

fn direction_to_int(direction: &Direction) -> i128 {
    match direction {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
        _ => panic!(),
    }
}

// Gets the neighbour and resets the location if necessary
fn pc_get_neighbour(pc: &mut IntCodePC, direction: &Direction) -> CellType {
    let (_, output) = pc.run_with_input([direction_to_int(direction)].into());

    let output = CellType::from(output[0]);
    if output != CellType::Wall {
        pc.run_with_input([direction_to_int(&direction.get_opposite())].into());
    }
    output
}
