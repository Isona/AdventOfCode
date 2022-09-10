const INPUT: &str = include_str!("input.txt");
use std::str::FromStr;

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &[Command]) -> i32 {
    let mut x = 0;
    let mut aim = 0;

    for command in input {
        match command.direction {
            Direction::Down => aim += command.distance,
            Direction::Up => aim -= command.distance,
            Direction::Forward => x += command.distance,
        }
    }
    x * aim
}

fn part_2(input: &[Command]) -> i32 {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in input {
        match command.direction {
            Direction::Down => aim += command.distance,
            Direction::Up => aim -= command.distance,
            Direction::Forward => {
                x += command.distance;
                depth += aim * command.distance;
            }
        }
    }
    x * depth
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            Command {
                direction: split.next().unwrap().parse().unwrap(),
                distance: split.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

struct Command {
    direction: Direction,
    distance: i32,
}

enum Direction {
    Down,
    Up,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 150);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 900);
    }
}
