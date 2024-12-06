use aoc_lib::{Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let part_1_answer = part_1(&mut input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &mut Grid<Location>) -> usize {
    let mut guard_location = input.find_item(&Location::Guard).unwrap();
    let mut trajectory = Direction::North;

    while let Some(next_location) = input.get_neighbour(&guard_location, &trajectory) {
        if next_location.value == &Location::Obstacle {
            trajectory = trajectory.turn_right();
        }
        else {
            guard_location = next_location.location;
            if next_location.value != &Location::Guard {
                input.set(guard_location, Location::Visited);
            }
        }
    }

    // + 1 for the initial guard location
    input.indexed_iter().filter(|(_, x) | x == &&Location::Visited).count() + 1
}

fn part_2(input: Grid<Location>) -> u64 {

    let initial_guard_location = input.find_item(&Location::Guard).unwrap();
    let mut guard_location ;

    let mut potential_obstacles = 0;
    let mut trajectory;

    for (new_obstacle_coord, value) in input.indexed_iter() {
        if value != &Location::Visited {
            continue;
        }

        let mut visited = false;
        guard_location = initial_guard_location;
        trajectory = Direction::North;

        while let Some(next_location) = input.get_neighbour(&guard_location, &trajectory) {
            if next_location.location == new_obstacle_coord && !visited{
                visited = true;
                trajectory = trajectory.turn_right();
            }
            else if next_location.location == new_obstacle_coord && visited {
                println!("New obstacle at {new_obstacle_coord:#?}");
                potential_obstacles += 1;
                break;
            }
            else if next_location.value == &Location::Obstacle {
                trajectory = trajectory.turn_right();
            }
            else {
                guard_location = next_location.location;
            }
        }
    }

    potential_obstacles
}

fn parse_input(input: &str) -> Grid<Location> {
    let mut grid: Grid<Location> = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(Location::new).collect());
    }

    grid
}

#[derive(Default, PartialEq, Eq, Clone)]
enum Location {
    #[default]
    Empty,
    Visited,
    Guard,
    Obstacle,
}

impl Location {
    pub fn new(input: char) -> Self {
        match input {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard,
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
        let mut input = parse_input(TESTINPUT);
        assert_eq!(part_1(&mut input), 41);
    }

    #[test]
    fn part_2_test() {
        let mut input = parse_input(TESTINPUT);
        let _ = part_1(&mut input); // Part 1 mutates the input
        assert_eq!(part_2(input), 6);
    }
}
