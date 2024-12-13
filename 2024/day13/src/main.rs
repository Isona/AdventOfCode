use regex::Regex;
use std::collections::HashSet;

use aoc_lib::{Coordinate, Vector};

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

fn part_1(input: &[ClawMachine]) -> i128 {
    input.iter().filter_map(ClawMachine::get_prize).sum()
}

fn part_2(input: &[ClawMachine]) -> u64 {
    todo!();
}

/** Input looks like this, with a blank line between machines:
 * Button A: X+71, Y+36
 * Button B: X+28, Y+62
 * Prize: X=11070, Y=4082
 */
fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut button1_iter = input.lines().step_by(4);
    let mut button2_iter = input.lines().skip(1).step_by(4);
    let prize_iter = input.lines().skip(2).step_by(4);

    let regex = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();

    let mut machines = Vec::new();
    for prize_str in prize_iter {
        // Get button 1
        let button1_caps = regex.captures(button1_iter.next().unwrap()).unwrap();
        let (_, [x, y]) = button1_caps.extract();
        let button1 = Vector::new(x.parse().unwrap(), y.parse().unwrap());

        // Get button 2
        let button2_caps = regex.captures(button2_iter.next().unwrap()).unwrap();
        let (_, [x, y]) = button2_caps.extract();
        let button2 = Vector::new(x.parse().unwrap(), y.parse().unwrap());

        // Get prize
        let prize_caps = regex.captures(prize_str).unwrap();
        let (_, [x, y]) = prize_caps.extract();
        let prize = Coordinate::new(x.parse().unwrap(), y.parse().unwrap());

        machines.push(ClawMachine {
            button1,
            button2,
            prize,
        });
    }

    machines
}

#[derive(Debug, PartialEq, Eq)]
struct ClawMachine {
    prize: Coordinate,
    button1: Vector,
    button2: Vector,
}

impl ClawMachine {
    fn get_prize(&self) -> Option<i128> {
        // No more than 100 presses of each button
        let start = Coordinate::default();
        let mut winning_ways: HashSet<(i128, i128)> = HashSet::new();

        for button1_presses in 1..100 {
            let current_base_loc = start.add_vec(&(self.button1 * button1_presses)).unwrap();

            if current_base_loc.x > self.prize.x || current_base_loc.y > self.prize.y {
                break;
            }

            for button2_presses in 1..100 {
                let current_location = current_base_loc
                    .add_vec(&(self.button2 * button2_presses))
                    .unwrap();
                if current_location.x > self.prize.x || current_location.y > self.prize.y {
                    break;
                } else if current_location == self.prize {
                    winning_ways.insert((button1_presses, button2_presses));
                }
            }
        }

        winning_ways.iter().map(|(x, y)| x * 3 + y).min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn get_claw_machine() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let machines = parse_input(input);
        assert_eq!(machines.len(), 1);
        let test_machine = ClawMachine {
            prize: Coordinate::new(8400, 5400),
            button1: Vector::new(94, 34),
            button2: Vector::new(22, 67),
        };

        assert_eq!(machines[0], test_machine);
    }

    #[test]
    fn machine_test_winnable() {
        let machine = ClawMachine {
            prize: Coordinate::new(8400, 5400),
            button1: Vector::new(94, 34),
            button2: Vector::new(22, 67),
        };

        assert_eq!(machine.get_prize(), Some(280));
    }

    #[test]
    fn machine_test_unwinnable() {
        let machine = ClawMachine {
            prize: Coordinate::new(12748, 12176),
            button1: Vector::new(26, 66),
            button2: Vector::new(67, 21),
        };

        assert_eq!(machine.get_prize(), None);
    }

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 480);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
