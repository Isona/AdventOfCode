use aoc_lib::Vector;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let grid_size = (101, 103);
    let part_1_answer = part_1(&input, &grid_size);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let (part_2_seconds, part_2_danger_level) = part_2(&input, &grid_size);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_seconds}, danger: {part_2_danger_level} in {time_taken:.3} ms");

    print_grid_at_second(&input, &grid_size, part_2_seconds);
}

fn part_1(robots: &[GuardRobot], grid_size: &(i128, i128)) -> i128 {
    get_danger_level_at_second(robots, grid_size, 100)
}

fn part_2(robots: &[GuardRobot], grid_size: &(i128, i128)) -> (i128, i128) {
    (0..10000)
        .map(|i| (i, get_danger_level_at_second(robots, grid_size, i)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
}

fn get_danger_level_at_second(
    robots: &[GuardRobot],
    grid_size: &(i128, i128),
    seconds: i128,
) -> i128 {
    let mut quadrants: Vec<i128> = Vec::from([0, 0, 0, 0]);
    for robot in robots {
        let total_motion = robot.velocity * seconds;
        let final_location_raw = robot.start + total_motion;
        let final_location = torus_vec(&final_location_raw, grid_size);

        if let Some(quadrant) = get_quadrant(&final_location, grid_size) {
            quadrants[quadrant] += 1;
        }
    }

    quadrants.iter().product()
}

fn parse_input(input: &str) -> Vec<GuardRobot> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut guards = Vec::new();
    for line in input.lines() {
        // Get button 1
        let guard_caps = regex.captures(line).unwrap();
        let (_, [start_x, start_y, vel_x, vel_y]) = guard_caps.extract();
        let start = Vector::new(start_x.parse().unwrap(), start_y.parse().unwrap());
        let velocity = Vector::new(vel_x.parse().unwrap(), vel_y.parse().unwrap());

        guards.push(GuardRobot { start, velocity });
    }

    guards
}

// Fit coordinates to a grid of size 101,103
fn torus_vec(input: &Vector, grid_size: &(i128, i128)) -> Vector {
    let mut x = input.x % grid_size.0;
    if x < 0 {
        x += grid_size.0;
    }

    let mut y = input.y % grid_size.1;
    if y < 0 {
        y += grid_size.1;
    }

    Vector::new(x, y)
}

fn get_quadrant(input: &Vector, grid_size: &(i128, i128)) -> Option<usize> {
    let midpoint_x = grid_size.0 / 2;

    let midpoint_y = grid_size.1 / 2;
    if input.x == midpoint_x || input.y == midpoint_y {
        return None;
    }

    if input.x < midpoint_x {
        if input.y < midpoint_y {
            Some(0)
        } else {
            Some(1)
        }
    } else if input.y < midpoint_y {
        Some(2)
    } else {
        Some(3)
    }
}

fn print_grid_at_second(robots: &[GuardRobot], grid_size: &(i128, i128), seconds: i128) {
    let mut guard_positions = Vec::new();
    for robot in robots {
        let total_motion = robot.velocity * seconds;
        let final_location_raw = robot.start + total_motion;
        let final_location = torus_vec(&final_location_raw, grid_size);
        guard_positions.push(final_location);
    }

    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            if guard_positions.contains(&Vector::new(x, y)) {
                print!("🤖");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GuardRobot {
    start: Vector,
    velocity: Vector,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn parse_test() {
        let input = "p=0,4 v=3,-3";
        let guards = parse_input(input);

        assert_eq!(guards.len(), 1);

        let test_guard = GuardRobot {
            start: Vector::new(0, 4),
            velocity: Vector::new(3, -3),
        };

        assert_eq!(test_guard, guards[0]);
    }

    #[test]
    fn torus_test() {
        // Grid is size 101, 103
        let test = Vector::new(-1, 104);
        let expected_result = Vector::new(100, 1);
        assert_eq!(torus_vec(&test, &(101, 103)), expected_result);
    }

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input, &(11, 7)), 12);
    }
}
