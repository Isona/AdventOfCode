use core::fmt;
use std::collections::{HashMap, HashSet};

use aoc_lib::{Coordinate, Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut warehouse_1, directions) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = traverse_warehouse(&mut warehouse_1, &directions, &LocContents::Box);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let part_2_input = get_part_2_input(INPUT);
    let (mut warehouse_2, directions) = parse_input(&part_2_input);

    let start = std::time::Instant::now();
    let part_2_answer = traverse_warehouse(&mut warehouse_2, &directions, &LocContents::BoxLeft);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn traverse_warehouse(
    warehouse: &mut Grid<LocContents>,
    directions: &Vec<Direction>,
    score_item: &LocContents,
) -> usize {
    let mut robot_coord = warehouse.find_item(&LocContents::Robot).unwrap();
    warehouse.set(robot_coord, LocContents::Empty);

    for direction in directions {
        let Some(next_location) = warehouse.get_neighbour(robot_coord, *direction) else {
            continue;
        };
        match next_location.value {
            LocContents::Wall => continue,
            LocContents::Box | LocContents::BoxLeft | LocContents::BoxRight => {
                if let Some(push_results) =
                    push_result(warehouse, &next_location.location, *direction)
                {
                    robot_coord = next_location.location;
                    let next_value = *next_location.value;
                    for (coord, value) in push_results {
                        warehouse.set(coord, value);
                    }

                    // Handle pushing half a box
                    if direction == &Direction::North || direction == &Direction::South {
                        if next_value == LocContents::BoxLeft {
                            let box_right = warehouse
                                .get_neighbour(robot_coord, Direction::East)
                                .unwrap()
                                .location;
                            warehouse.set(box_right, LocContents::Empty);
                        } else if next_value == LocContents::BoxRight {
                            let box_left = warehouse
                                .get_neighbour(robot_coord, Direction::West)
                                .unwrap()
                                .location;
                            warehouse.set(box_left, LocContents::Empty);
                        }
                    }
                }
            }
            LocContents::Empty => robot_coord = next_location.location,
            _ => panic!(),
        }
    }

    println!("Final warehouse!");
    println!("{warehouse}");
    // Calculate the GPS sums
    warehouse
        .find_all(score_item)
        .map(|coord| coord.y * 100 + coord.x)
        .sum()
}

fn push_result(
    warehouse: &Grid<LocContents>,
    coord: &Coordinate,
    direction: Direction,
) -> Option<HashMap<Coordinate, LocContents>> {
    let changed_cells = changed_cells(warehouse, coord, direction)?;

    let mut cell_updates = HashMap::new();
    for changed_cell in &changed_cells {
        let behind = warehouse
            .get_neighbour(*changed_cell, direction.get_opposite())
            .unwrap();

        if changed_cells.contains(&behind.location) {
            cell_updates.insert(*changed_cell, *behind.value);
        } else {
            cell_updates.insert(*changed_cell, LocContents::Empty);
        }
    }

    Some(cell_updates)
}

// Returns a hashset of coordinates which change in the push
fn changed_cells(
    warehouse: &Grid<LocContents>,
    coord: &Coordinate,
    direction: Direction,
) -> Option<HashSet<Coordinate>> {
    let current_value = warehouse.get(*coord);
    match current_value {
        // If it's a wall, then we want to return None because nothing moves
        LocContents::Wall => None,
        LocContents::Box => {
            // Get the neighbour coordinate
            let neighbour_coord = warehouse.get_neighbour(*coord, direction).unwrap().location;

            // Try pushing the neighbour, if we get Some back then we know we can push this as well
            if let Some(mut results) = changed_cells(warehouse, &neighbour_coord, direction) {
                results.insert(*coord);
                Some(results)
            } else {
                None
            }
        }
        LocContents::Robot => panic!(),
        LocContents::BoxLeft | LocContents::BoxRight => {
            // First work on the left half of this
            // Get the neighbour coordinate
            let neighbour_coord = warehouse.get_neighbour(*coord, direction).unwrap().location;

            // Try pushing the neighbour, if we get Some back then we know we can push this as well
            let mut results = changed_cells(warehouse, &neighbour_coord, direction)?;
            results.insert(*coord);

            // We only do the horrible cascade if pushing north or south, otherwise the larger boxes behave like boxes
            if direction == Direction::North || direction == Direction::South {
                // Work out what direction the other half of the box is in
                let other_half_direction = if current_value == &LocContents::BoxLeft {
                    Direction::East
                } else {
                    Direction::West
                };
                // Get the other half of the box
                let other_half = warehouse
                    .get_neighbour(*coord, other_half_direction)
                    .unwrap();

                assert!(
                    other_half.value == &LocContents::BoxLeft
                        || other_half.value == &LocContents::BoxRight
                );

                // Get the neighbour ahead of the other half of the box
                let other_half_neighbour = warehouse
                    .get_neighbour(other_half.location, direction)
                    .unwrap();

                // Get the results of if we push other half's neighbour
                let other_half_results =
                    changed_cells(warehouse, &other_half_neighbour.location, direction)?;

                results.extend(other_half_results.iter());
                results.insert(other_half.location);
            }

            Some(results)
        }
        // If the current space is empty, then we return Some() and move the space behind us into this space
        LocContents::Empty => Some(HashSet::from([*coord])),
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
enum LocContents {
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
    #[default]
    Empty,
}

impl LocContents {
    fn new(input: char) -> Self {
        match input {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => Self::Empty,
        }
    }
}

impl fmt::Display for LocContents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocContents::Wall => write!(f, "#"),
            LocContents::Box => write!(f, "O"),
            LocContents::Robot => write!(f, "@"),
            LocContents::BoxLeft => write!(f, "["),
            LocContents::BoxRight => write!(f, "]"),
            LocContents::Empty => write!(f, "."),
        }
    }
}

fn parse_direction(input: char) -> Direction {
    match input {
        '^' => Direction::North,
        'v' => Direction::South,
        '>' => Direction::East,
        '<' => Direction::West,
        _ => panic!(),
    }
}

fn get_part_2_input(input: &str) -> String {
    input
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.")
}

enum ParseState {
    ParsingGrid,
    ParsingMovement,
}

fn parse_input(input: &str) -> (Grid<LocContents>, Vec<Direction>) {
    let mut warehouse = Grid::new();
    let mut directions = Vec::new();
    let mut current_state = ParseState::ParsingGrid;

    for line in input.lines() {
        match current_state {
            ParseState::ParsingGrid => {
                if line.is_empty() {
                    current_state = ParseState::ParsingMovement;
                    continue;
                }

                warehouse.push_row(line.chars().map(LocContents::new).collect());
            }
            ParseState::ParsingMovement => directions.extend(line.chars().map(parse_direction)),
        }
    }

    (warehouse, directions)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");
    const TESTINPUT_PART2_SMALL: &str = include_str!("testinput_part2_small.txt");

    #[test]
    fn part_1_test_1() {
        let (mut warehouse, directions) = parse_input(TESTINPUT);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::Box),
            2028
        );
    }

    #[test]
    fn part_1_test_2() {
        let (mut warehouse, directions) = parse_input(TESTINPUT2);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::Box),
            10092
        );
    }

    #[test]
    fn part_2_test_1() {
        let input = get_part_2_input(TESTINPUT);
        let (mut warehouse, directions) = parse_input(&input);
        println!("{warehouse}");
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            1751
        );
    }

    #[test]
    fn part_2_test_2() {
        let input = get_part_2_input(TESTINPUT2);
        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            9021
        );
    }

    #[test]
    fn part_2_test_small() {
        let input = get_part_2_input(TESTINPUT_PART2_SMALL);
        dbg!(&input);
        let (mut warehouse, directions) = parse_input(&input);
        println!("{warehouse}");
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            618
        );
    }

    #[test]
    fn part_2_test_3() {
        let input = "#####
#...#
#.O@#
#OO.#
#O#.#
#...#
#####

<^<<v";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            1211
        );
    }

    #[test]
    fn part_2_test_4() {
        let input = "#####
#...#
#.O@#
#OO.#
##O.#
#...#
#####

<^<<v";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            1213
        );
    }

    #[test]
    fn part_2_test_5() {
        let input = "#######
#.....#
#.OO@.#
#.....#
#######

<<";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            406
        );
    }

    #[test]
    fn part_2_test_6() {
        let input = "#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            509
        );
    }

    #[test]
    fn part_2_test_7() {
        let input = "######
#....#
#..#.#
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^^^";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        println!("{}", warehouse);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            1216
        );
    }

    #[test]
    fn part_2_test_8() {
        let input = "########
#......#
#OO....#
#.O....#
#.O....#
##O....#
#O..O@.#
#......#
########

<^^<<>^^^<v";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        println!("{}", warehouse);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            2827
        );
    }

    #[test]
    fn part_2_test_9() {
        let input = "#######
#.....#
#.....#
#.@O..#
#..#O.#
#...O.#
#..O..#
#.....#
#######

>><vvv>v>^^^";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        println!("{}", warehouse);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            1430
        );
    }

    #[test]
    fn part_2_test_10() {
        let input = "#######
#.....#
#.O.O@#
#..O..#
#..O..#
#.....#
#######

<v<<>vv<^^";
        let input = get_part_2_input(input);

        let (mut warehouse, directions) = parse_input(&input);
        println!("{}", warehouse);
        assert_eq!(
            traverse_warehouse(&mut warehouse, &directions, &LocContents::BoxLeft),
            822
        );
    }
}
