use aoc_lib::{Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut warehouse_1, directions) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&mut warehouse_1, &directions);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let part_2_input = get_part_2_input(INPUT);
    let (mut warehouse_2, directions) = parse_input(&part_2_input);

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut warehouse_2, &directions);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(warehouse: &mut Grid<LocContents>, directions: &Vec<Direction>) -> usize {
    let mut robot_coord = warehouse.find_item(&LocContents::Robot).unwrap();
    warehouse.set(robot_coord, LocContents::Empty);

    for direction in directions {
        let Some(next_location) = warehouse.get_neighbour(robot_coord, *direction) else {
            continue;
        };
        match next_location.value {
            LocContents::Wall => continue,
            LocContents::Box => {
                let view = warehouse.view_from(&robot_coord, *direction);
                let non_box = view.iter().find(|x| x.value != &LocContents::Box).unwrap();

                let non_box_loc = non_box.location;
                if non_box.value == &LocContents::Empty {
                    robot_coord = next_location.location;
                    warehouse.set(next_location.location, LocContents::Empty);
                    warehouse.set(non_box_loc, LocContents::Box);
                }
            }
            LocContents::Empty => robot_coord = next_location.location,
            _ => panic!(),
        }
    }

    // Calculate the GPS sums
    warehouse
        .find_all(&LocContents::Box)
        .map(|coord| coord.y * 100 + coord.x)
        .sum()
}

fn part_2(mut warehouse: &mut Grid<LocContents>, directions: &Vec<Direction>) -> usize {
    let mut robot_coord = warehouse.find_item(&LocContents::Robot).unwrap();
    warehouse.set(robot_coord, LocContents::Empty);

    for direction in directions {
        let Some(next_location) = warehouse.get_neighbour(robot_coord, *direction) else {
            continue;
        };
        match next_location.value {
            LocContents::Wall => continue,
            LocContents::BoxLeft | LocContents::BoxRight => {
                let view = warehouse.view_from(&robot_coord, *direction);
                let non_box = view.iter().find(|x| x.value != &LocContents::Box).unwrap();

                let non_box_loc = non_box.location;
                if non_box.value == &LocContents::Empty {
                    robot_coord = next_location.location;
                    warehouse.set(next_location.location, LocContents::Empty);
                    warehouse.set(non_box_loc, LocContents::Box);
                }
            }
            LocContents::Empty => robot_coord = next_location.location,
            _ => panic!(),
        }
    }

    // Calculate the GPS sums
    warehouse
        .find_all(&LocContents::BoxLeft)
        .map(|coord| coord.y * 100 + coord.x)
        .sum()
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

    #[test]
    fn part_1_test_1() {
        let (mut warehouse, directions) = parse_input(TESTINPUT);
        assert_eq!(part_1(&mut warehouse, &directions), 2028);
    }

    #[test]
    fn part_1_test_2() {
        let (mut warehouse, directions) = parse_input(TESTINPUT2);
        assert_eq!(part_1(&mut warehouse, &directions), 10092);
    }

    #[test]
    fn part_2_test_1() {
        let input = get_part_2_input(TESTINPUT);
        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(part_2(&mut warehouse, &directions), 105);
    }

    #[test]
    fn part_2_test_2() {
        let input = get_part_2_input(TESTINPUT2);
        let (mut warehouse, directions) = parse_input(&input);
        assert_eq!(part_2(&mut warehouse, &directions), 9021);
    }
}
