use aoc_lib::{Direction, Vector};
use rustc_hash::FxHashSet as HashSet;

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

fn part_1(input: &[Travel]) -> i128 {
    let mut current_direction = Direction::North;
    let mut current_coordinate = Vector::new(0, 0);

    for block in input {
        match block {
            Travel::Right(magnitude) => {
                current_direction = current_direction.turn_right();
                current_coordinate =
                    current_coordinate.go_in_direction(current_direction, *magnitude);
            }
            Travel::Left(magnitude) => {
                current_direction = current_direction.turn_left();
                current_coordinate =
                    current_coordinate.go_in_direction(current_direction, *magnitude);
            }
        }
    }

    current_coordinate.x.abs() + current_coordinate.y.abs()
}

fn part_2(input: &[Travel]) -> i128 {
    let mut current_direction = Direction::North;
    let mut current_coordinate = Vector::new(0, 0);
    let mut visited_coordinates = HashSet::default();
    visited_coordinates.insert(current_coordinate);
    for block in input {
        match block {
            Travel::Right(magnitude) => {
                current_direction = current_direction.turn_right();

                for _ in 0..*magnitude {
                    current_coordinate = current_coordinate.go_in_direction(current_direction, 1);
                    if !visited_coordinates.insert(current_coordinate) {
                        println!("{current_coordinate}");
                        return current_coordinate.x.abs() + current_coordinate.y.abs();
                    }
                }
            }
            Travel::Left(magnitude) => {
                current_direction = current_direction.turn_left();

                for _ in 0..*magnitude {
                    current_coordinate = current_coordinate.go_in_direction(current_direction, 1);
                    if !visited_coordinates.insert(current_coordinate) {
                        println!("{current_coordinate}");
                        return current_coordinate.x.abs() + current_coordinate.y.abs();
                    }
                }
            }
        }
    }

    panic!()
}

fn parse_input(input: &str) -> Vec<Travel> {
    let mut travelled = Vec::new();
    for direction in input.split(", ") {
        if direction.starts_with('R') {
            let value = direction.strip_prefix("R").unwrap().parse().unwrap();
            travelled.push(Travel::Right(value));
        } else {
            let value = direction.strip_prefix("L").unwrap().parse().unwrap();
            travelled.push(Travel::Left(value));
        }
    }

    travelled
}

#[derive(Debug)]
enum Travel {
    Right(i128),
    Left(i128),
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "R5, L5, R5, R3";

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 12);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input("R8, R4, R4, R8");
        assert_eq!(part_2(&input), 4);
    }
}
