#![feature(iter_intersperse)]
use std::{collections::VecDeque, fmt::Display};

use aoc_lib::{Coordinate, Direction, Grid};
use intcode::{IntCodePC, IntCodeProgramState};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);

    let start = std::time::Instant::now();
    let (grid, part_1_answer) = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut pc, &grid);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> (Grid<char>, usize) {
    let (_, output) = pc.run_with_input([].into());
    let output: Vec<u8> = output.iter().map(|x| (*x as u8)).collect();
    let input = std::str::from_utf8(&output).unwrap();

    let mut grid = Grid::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        grid.push_row(line.trim().chars().collect());
    }

    let part_1_result = grid
        .find_all(&'#')
        .filter(|scaffold| {
            grid.get_cardinal_neighbours(*scaffold)
                .all(|x| x.value == &'#')
        })
        .map(|intersection| intersection.x * intersection.y)
        .sum();
    (grid, part_1_result)
}

fn part_2(pc: &mut IntCodePC, grid: &Grid<char>) -> i128 {
    let path = pathfinding(grid);

    pc.reset_all();
    pc.set(0, 2);

    let movement_routine = MovementRoutine::new(&path);
    let input = movement_routine.get_robot_input();

    let (state, output) = pc.run_with_input(input);
    assert_eq!(state, IntCodeProgramState::Halted);

    *output.back().unwrap()
}

fn pathfinding(grid: &Grid<char>) -> Vec<Movement> {
    let mut current_coord = grid.find_item(&'^').unwrap();
    let mut current_direction = Direction::North;
    let mut path = Vec::new();

    while let Some(next_movement) = get_next_movement(grid, current_coord, current_direction) {
        match next_movement {
            Movement::Right => current_direction = current_direction.turn_right(),
            Movement::Left => current_direction = current_direction.turn_left(),
            Movement::Forward(steps) => {
                for _ in 0..steps {
                    current_coord = grid
                        .get_neighbour(current_coord, current_direction)
                        .unwrap()
                        .location;
                }
            }
        }

        path.push(next_movement);
    }

    path
}

fn get_next_movement(
    grid: &Grid<char>,
    current_coord: Coordinate,
    current_direction: Direction,
) -> Option<Movement> {
    // If we can move forward do it
    let mut current_coord = current_coord;
    let mut steps = 0;
    while let Some(neighbour) = grid.get_neighbour(current_coord, current_direction) {
        if neighbour.value == &'#' {
            steps += 1;
            current_coord = neighbour.location;
        } else {
            break;
        }
    }

    if steps != 0 {
        return Some(Movement::Forward(steps));
    }

    // Try turning right
    if let Some(neighbour) = grid.get_neighbour(current_coord, current_direction.turn_right()) {
        if neighbour.value == &'#' {
            return Some(Movement::Right);
        }
    }

    // Try turning left
    if let Some(neighbour) = grid.get_neighbour(current_coord, current_direction.turn_left()) {
        if neighbour.value == &'#' {
            return Some(Movement::Left);
        }
    }

    None
}

#[derive(PartialEq, Eq, Debug)]
enum Movement {
    Right,
    Left,
    Forward(u8),
}

impl Movement {
    // Converts a list of movements into a comma separated list with newline at end
    fn get_robot_input(input: &[Movement]) -> VecDeque<i128> {
        let mut output = VecDeque::new();
        for value in input {
            match value {
                Movement::Right => output.push_back(82i128),
                Movement::Left => output.push_back(76i128),
                Movement::Forward(i) => match i {
                    0..10 => output.push_back(48 + Into::<i128>::into(*i)),
                    10..15 => {
                        output.push_back(49);
                        output.push_back(48i128 + Into::<i128>::into(i % 10));
                    }
                    _ => panic!(),
                },
            }
            output.push_back(44);
        }
        output.pop_back();
        output.push_back(10);
        output
    }

    fn from_char(x: char) -> Movement {
        match x {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => Self::Forward(x.to_digit(16).unwrap().try_into().unwrap()),
        }
    }
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Movement::Right => write!(f, "R"),
            Movement::Left => write!(f, "L"),
            Movement::Forward(i) => write!(f, "{i:x}"),
        }
    }
}

struct MovementRoutine {
    main_routine: String,
    a: Vec<Movement>,
    b: Vec<Movement>,
    c: Vec<Movement>,
}

impl MovementRoutine {
    fn new(path: &[Movement]) -> Self {
        let path_string = path.iter().map(Movement::to_string).collect::<String>();
        println!("{path_string}");
        let (a, b, c) = compact_path(&path_string).unwrap();
        println!("A: {a}, B: {b}, C: {c}");

        let main_routine = path_string
            .replace(&a, "A")
            .replace(&b, "B")
            .replace(&c, "C");

        let a = a.chars().map(Movement::from_char).collect();
        let b = b.chars().map(Movement::from_char).collect();
        let c = c.chars().map(Movement::from_char).collect();

        Self {
            main_routine,
            a,
            b,
            c,
        }
    }

    fn get_robot_input(&self) -> VecDeque<i128> {
        // Convert the main routine chars into ascii and intersperse with commas
        let mut input: VecDeque<i128> = self
            .main_routine
            .chars()
            .map(|x| match x {
                'A' => 65,
                'B' => 66,
                'C' => 67,
                _ => panic!(),
            })
            .intersperse(44)
            .collect();
        input.push_back(10);

        // Add A to the buffer
        input.append(&mut Movement::get_robot_input(&self.a));

        // Add B to the buffer
        input.append(&mut Movement::get_robot_input(&self.b));

        // Add C to the buffer
        input.append(&mut Movement::get_robot_input(&self.c));

        // No I don't want video thanks
        input.push_back(110);
        input.push_back(10);

        input
    }
}

fn compact_path(path_string: &str) -> Option<(String, String, String)> {
    // Loop over potential a,b,c patterns
    for a_len in 1..=10 {
        // Try the first n characters of the path as a potential pattern
        let a: String = path_string.chars().take(a_len).collect();
        let a_matches = path_string.match_indices(&a).count();

        // If A occurs more than 10 times then we can skip this - it won't fit
        if a_matches > 10 {
            continue;
        }

        // Split the string to remove occurences of our current A
        // Remove empty splits
        let a_split = path_string
            .split(&a)
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        assert!(!a_split.is_empty());

        // Iterate over potential Bs
        for b_len in 1..=a_split[0].len().min(10) {
            // Take the first n characters of the b
            let b: String = a_split[0].chars().take(b_len).collect();

            let b_matches: usize = a_split
                .iter()
                .map(|split| split.match_indices(&b).count())
                .sum();

            // Check that the number of A and B matches fit into memory
            if b_matches + a_matches > 10 {
                continue;
            }

            // Split each of the a splits using a flat map
            let b_split: Vec<&str> = a_split
                .iter()
                .flat_map(|initial_split| initial_split.split(&b).filter(|x| !x.is_empty()))
                .collect();

            // Try potential values of C
            for c_len in 1..=b_split[0].len().min(10) {
                let c: String = b_split[0].chars().take(c_len).collect();
                let c_matches: usize = b_split
                    .iter()
                    .map(|split| split.match_indices(&c).count())
                    .sum();

                // Check that A + B + C matches fit into memory
                if a_matches + b_matches + c_matches > 10 {
                    continue;
                }

                // See if current C will use the rest of the string
                let c_split_len: usize = b_split.iter().map(|x| x.len()).sum();
                if c_matches * c_len == c_split_len {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}
