use std::collections::HashMap;

use aoc_lib::Vector;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (line_1, line_2) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1(&line_1, &line_2);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(wire_1: &Vec<Line>, wire_2: &Vec<Line>) -> (i128, i128) {
    let mut current_coord = Vector::new(0, 0);

    // Get line 1's visited points
    let mut visited_coords = HashMap::new();
    let mut steps = 0;
    for segment in wire_1 {
        for _ in 0..segment.magnitude {
            current_coord = current_coord + segment.direction;
            steps += 1;
            visited_coords.insert(current_coord, steps);
        }
    }

    let mut min_intersection = i128::MAX;
    let mut min_steps = i128::MAX;

    current_coord = Vector::new(0, 0);
    steps = 0;
    for segment in wire_2 {
        for _ in 0..segment.magnitude {
            current_coord = current_coord + segment.direction;
            steps += 1;
            if let Some(line_1_steps) = visited_coords.get(&current_coord) {
                min_intersection =
                    min_intersection.min(current_coord.x.abs() + current_coord.y.abs());
                min_steps = min_steps.min(line_1_steps + steps);
            }
        }
    }

    (min_intersection, min_steps)
}

fn parse_input(input: &str) -> (Vec<Line>, Vec<Line>) {
    let mut lines = input.lines();
    let line_1 = lines.next().unwrap().split(",").map(Line::from).collect();
    let line_2 = lines.next().unwrap().split(",").map(Line::from).collect();
    (line_1, line_2)
}

struct Line {
    direction: Vector,
    magnitude: usize,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let magnitude = value[1..].parse().unwrap();
        let direction = match value.chars().next().unwrap() {
            'R' => Vector::new(1, 0),
            'L' => Vector::new(-1, 0),
            'U' => Vector::new(0, 1),
            'D' => Vector::new(0, -1),
            _ => panic!(),
        };
        Self {
            direction,
            magnitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (line_1, line_2) = parse_input(
            "R8,U5,L5,D3
U7,R6,D4,L4",
        );
        assert_eq!(part_1(&line_1, &line_2).0, 6);
    }
    #[test]
    fn part_1_test_2() {
        let (line_1, line_2) = parse_input(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        );
        assert_eq!(part_1(&line_1, &line_2).0, 159);
    }
    #[test]
    fn part_1_test_3() {
        let (line_1, line_2) = parse_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(part_1(&line_1, &line_2).0, 135);
    }

    #[test]
    fn part_2_test() {
        let (line_1, line_2) = parse_input(
            "R8,U5,L5,D3
U7,R6,D4,L4",
        );
        assert_eq!(part_1(&line_1, &line_2).1, 30);
    }
    #[test]
    fn part_2_test_2() {
        let (line_1, line_2) = parse_input(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        );
        assert_eq!(part_1(&line_1, &line_2).1, 610);
    }
    #[test]
    fn part_2_test_3() {
        let (line_1, line_2) = parse_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(part_1(&line_1, &line_2).1, 410);
    }

    // #[test]
    // fn part_2_test() {
    //     let input = parse_input(TESTINPUT);
    //     assert_eq!(part_2(&input), 5);
    // }
}
