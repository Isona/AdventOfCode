use std::collections::{HashMap, HashSet};

use aoc_lib::Direction;
use aoc_lib::{Coordinate, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let (part_1_answer, loop_coords) = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&mut input, loop_coords);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &Grid<Pipe>) -> (usize, HashSet<Coordinate>) {
    let coords_to_search = get_start_connections(input);

    let start_coord = input.find_item(&Pipe::Start).unwrap();
    let mut searched_coords: HashMap<Coordinate, usize> = HashMap::new();
    searched_coords.insert(start_coord, 0);
    let mut coords_to_search: Vec<(Coordinate, Direction, usize)> =
        coords_to_search.iter().map(|x| (x.0, x.1, 1)).collect();

    // Pop the item
    // Get the item
    while let Some((coord, trajectory, steps)) = coords_to_search.pop() {
        // If we haven't searched this coordinate before
        if !searched_coords.contains_key(&coord)
            || searched_coords.get(&coord).unwrap() > &(steps + 1)
        {
            searched_coords.insert(coord, steps);
            let current_pipe = input.get(coord);
            let new_trajectory = current_pipe.get_next_trajectory(trajectory).unwrap();
            let neighbour = input.get_neighbour(coord, new_trajectory).unwrap();
            let next_steps = steps + 1;
            coords_to_search.push((neighbour.location, new_trajectory, next_steps));
        }
    }

    (
        *searched_coords.values().max().unwrap(),
        searched_coords.keys().copied().collect(),
    )
}

fn part_2(input: &mut Grid<Pipe>, loop_coords: HashSet<Coordinate>) -> u64 {
    set_start(input);

    let mut inside_count = 0;
    for (coord, _item) in input.indexed_iter().filter(|x| !loop_coords.contains(&x.0)) {
        let mut vertical_count = 0;
        let mut last_corner = Pipe::Empty;
        for east_pipe in input
            .view_from(&coord, Direction::East)
            .filter(|x| loop_coords.contains(&x.location))
        {
            let east_pipe_type = input.get(east_pipe.location);
            match east_pipe_type {
                Pipe::Vertical => vertical_count += 1,
                Pipe::NE | Pipe::SE => last_corner = *east_pipe_type,
                Pipe::NW => {
                    if last_corner == Pipe::SE {
                        vertical_count += 1
                    };
                    last_corner = Pipe::Empty
                }
                Pipe::SW => {
                    if last_corner == Pipe::NE {
                        vertical_count += 1
                    };
                    last_corner = Pipe::Empty
                }
                Pipe::Start => {
                    panic!()
                }
                Pipe::Empty | Pipe::Horizontal => {}
            }
        }

        if vertical_count % 2 != 0 {
            inside_count += 1
        }
    }
    inside_count
}

fn get_start_connections(input: &Grid<Pipe>) -> Vec<(Coordinate, Direction)> {
    // Find start
    let start_coord = input.find_item(&Pipe::Start).unwrap();

    let all_neighbours = input.get_cardinal_neighbours(start_coord);

    let mut valid_neighbours: Vec<(Coordinate, Direction)> = vec![];

    for neighbour in all_neighbours {
        let pipe = input.get(neighbour.location);
        if pipe.get_next_trajectory(neighbour.direction).is_some() {
            valid_neighbours.push((neighbour.location, neighbour.direction));
        }
    }

    valid_neighbours
}

fn set_start(input: &mut Grid<Pipe>) {
    let start_loc = input.find_item(&Pipe::Start).unwrap();
    let start_neighbours = get_start_connections(input);
    assert_eq!(start_neighbours.len(), 2);

    let start_type = match start_neighbours[0].1 {
        Direction::North => match start_neighbours[1].1 {
            Direction::South => Pipe::Vertical,
            Direction::East => Pipe::NE,
            Direction::West => Pipe::NW,
            _ => {
                panic!()
            }
        },
        Direction::South => match start_neighbours[1].1 {
            Direction::North => Pipe::Vertical,
            Direction::East => Pipe::SE,
            Direction::West => Pipe::SW,
            _ => {
                panic!()
            }
        },
        Direction::East => match start_neighbours[1].1 {
            Direction::North => Pipe::NE,
            Direction::South => Pipe::SE,
            Direction::West => Pipe::Horizontal,
            _ => panic!(),
        },
        Direction::West => match start_neighbours[1].1 {
            Direction::North => Pipe::NW,
            Direction::South => Pipe::SW,
            Direction::East => Pipe::Horizontal,
            _ => panic!(),
        },
        _ => {
            panic!()
        }
    };

    input.set(start_loc, start_type);
}

fn parse_input(input: &str) -> Grid<Pipe> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(Pipe::get_pipe).collect());
    }
    grid
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Start,
    #[default]
    Empty,
}

impl Pipe {
    fn get_pipe(input: char) -> Pipe {
        match input {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            'S' => Pipe::Start,
            _ => Pipe::Empty,
        }
    }

    fn get_next_trajectory(
        self,
        // Direction we're moving in
        // So if East, we're on the west side of the pipe
        trajectory: Direction,
        // Returns the trajectory if valid
    ) -> Option<Direction> {
        match trajectory {
            Direction::North => match self {
                Pipe::Vertical => Some(Direction::North),
                Pipe::SE => Some(Direction::East),
                Pipe::SW => Some(Direction::West),
                _ => None,
            },
            Direction::South => match self {
                Pipe::Vertical => Some(Direction::South),
                Pipe::NE => Some(Direction::East),
                Pipe::NW => Some(Direction::West),
                _ => None,
            },
            Direction::East => match self {
                Pipe::Horizontal => Some(Direction::East),
                Pipe::NW => Some(Direction::North),
                Pipe::SW => Some(Direction::South),
                _ => None,
            },
            Direction::West => match self {
                Pipe::Horizontal => Some(Direction::West),
                Pipe::NE => Some(Direction::North),
                Pipe::SE => Some(Direction::South),
                _ => None,
            },
            _ => {
                panic!("Got a non-cardinal direction!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");
    const TESTINPUT3: &str = include_str!("testinput3.txt");
    const TESTINPUT4: &str = include_str!("testinput4.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).0, 4);
    }

    #[test]
    fn part_1_test2() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_1(&input).0, 8);
    }

    #[test]
    fn part_2_test() {
        let mut input = parse_input(TESTINPUT3);
        let (_, loop_coords) = part_1(&input);
        assert_eq!(part_2(&mut input, loop_coords), 4);
    }

    #[test]
    fn part_2_test2() {
        let mut input = parse_input(TESTINPUT4);
        let (_, loop_coords) = part_1(&input);
        assert_eq!(part_2(&mut input, loop_coords), 10);
    }
}
