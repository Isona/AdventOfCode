use aoc_lib::Coordinate;
use itertools::Itertools;

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

fn part_1(input: &[Coordinate]) -> usize {
    let mut max_square_size = 0;
    for (first, second) in input.iter().tuple_combinations() {
        let square_size = (first.x.abs_diff(second.x) + 1) * (first.y.abs_diff(second.y) + 1);
        max_square_size = max_square_size.max(square_size);
    }

    max_square_size
}

fn part_2(input: &[Coordinate]) -> usize {
    let mut max_square_size = 0;

    for (first, second) in input.iter().tuple_combinations() {
        println!("{first}, {second}");
        if !input
            .iter()
            .any(|other| square_contains(first, second, other))
        {
            println!("No other corners in here!");
            let square_size = (first.x.abs_diff(second.x) + 1) * (first.y.abs_diff(second.y) + 1);
            max_square_size = max_square_size.max(square_size);
        }
    }

    max_square_size
}

fn square_contains(corner_1: &Coordinate, corner_2: &Coordinate, other: &Coordinate) -> bool {
    if other == corner_1 || other == corner_2 {
        return false;
    }

    let contains_x = corner_1.x.min(corner_2.x) <= other.x && corner_1.x.max(corner_2.x) >= other.x;
    let contains_y = corner_1.y.min(corner_2.y) <= other.y && corner_1.y.max(corner_2.y) >= other.y;

    if contains_x && contains_y {
        println!("{corner_1} {corner_2} contains {other}");
    }
    contains_x && contains_y
}

fn parse_input(input: &str) -> Vec<Coordinate> {
    input.lines().map(get_coord).collect()
}

fn get_coord(input: &str) -> Coordinate {
    let mut split = input.split(',');
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();
    Coordinate { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 50);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 24);
    }
}
