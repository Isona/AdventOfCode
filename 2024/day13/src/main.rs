use regex::Regex;

use aoc_lib::Vector;

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
    input.iter().filter_map(|x| x.get_prize(0)).sum()
}

fn part_2(input: &[ClawMachine]) -> i128 {
    input
        .iter()
        .filter_map(|x| x.get_prize(10000000000000))
        .sum()
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
        let prize = Vector::new(x.parse().unwrap(), y.parse().unwrap());

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
    prize: Vector,
    button1: Vector,
    button2: Vector,
}

impl ClawMachine {
    fn get_prize(&self, prize_addition: i128) -> Option<i128> {
        // Get the correct prize location
        let prize = self.prize + prize_addition;

        // Example from test input:
        // Equation 1: 94a + 22b = 8400 : button1.x, button2.x, prize.x
        // Equation 2: 34a + 67b = 5400 : button1.y, button2.y, prize.y

        // Multiply the button 1 x and prize x values by button2.y
        // 67*(94a + 22b = 8400) => 6298a + 1474b = 562800
        let equation_1_a = self.button1.x * self.button2.y;
        let equation_1_prize = prize.x * self.button2.y;

        // Multiply the button 1.y and prize.y values by button2.x
        // 22*(34a + 67b = 5400) => 748a + 1474b = 118800
        let equation_2_a = self.button1.y * self.button2.x;
        let equation_2_prize = prize.y * self.button2.x;

        // Subtract the a and prize values
        // (6298a + 1474b = 562800) - (748a + 1474b = 118800) => 5550a = 444000
        let equation_3_a = equation_1_a - equation_2_a;
        let equation_3_prize = equation_1_prize - equation_2_prize;

        // If the prize value isn't divisible by a then it's not a valid solution
        if equation_3_prize % equation_3_a != 0 {
            return None;
        }

        // (5550a = 444000) / 5550 => a = 80
        let presses_1 = equation_3_prize / equation_3_a;

        // Look at equation 1 again: 94a + 22b = 8400
        // 94(80) + 22b = 8400
        // 22b = 8400 - 94(80) = 7520
        let equation_4_prize = prize.x - (self.button1.x * presses_1);

        // If the prize value isn't divisible by b then it's not valid
        if equation_4_prize % self.button2.x != 0 {
            return None;
        }

        // (22b = 8400) / 22
        // b = 40
        let presses_2 = equation_4_prize / self.button2.x;

        Some(presses_1 * 3 + presses_2)
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
            prize: Vector::new(8400, 5400),
            button1: Vector::new(94, 34),
            button2: Vector::new(22, 67),
        };

        assert_eq!(machines[0], test_machine);
    }

    #[test]
    fn machine_test_winnable() {
        let machine = ClawMachine {
            prize: Vector::new(8400, 5400),
            button1: Vector::new(94, 34),
            button2: Vector::new(22, 67),
        };

        assert_eq!(machine.get_prize(0), Some(280));
    }

    #[test]
    fn machine_test_unwinnable() {
        let machine = ClawMachine {
            prize: Vector::new(12748, 12176),
            button1: Vector::new(26, 66),
            button2: Vector::new(67, 21),
        };

        assert_eq!(machine.get_prize(0), None);
    }

    #[test]
    fn machine_test_part_2() {
        let machine = ClawMachine {
            prize: Vector::new(12748, 12176),
            button1: Vector::new(26, 66),
            button2: Vector::new(67, 21),
        };

        assert_eq!(machine.get_prize(10000000000000), Some(459236326669));
    }

    #[test]
    fn machine_test_part_2_unwinnable() {
        let machine = ClawMachine {
            prize: Vector::new(8400, 5400),
            button1: Vector::new(94, 34),
            button2: Vector::new(22, 67),
        };

        assert_eq!(machine.get_prize(10000000000000), None);
    }

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 480);
    }
}
