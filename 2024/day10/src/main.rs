use std::collections::HashSet;

use aoc_lib::{Coordinate, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let (part_1_answer, part_2_answer) = part_1(&input);
    println!("Part 1: {part_1_answer}");
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &Grid<u32>) -> (usize, usize) {
    let start_locations = input.find_all(&0);

    let mut part1_total = 0;
    let mut part2_total = 0;

    for start_location in start_locations {
        let (found_nines, route_count) = find_route(input, start_location);
        part1_total += found_nines.len();
        part2_total += route_count;
    }

    (part1_total, part2_total)
}


fn find_route(input: &Grid<u32>, current_coord:Coordinate) -> (HashSet<Coordinate>, usize) {
    let current_value = *input.get(current_coord);
    if current_value == 9 {
        return (HashSet::from([current_coord]), 1);
    }

    let mut found_nines = HashSet::new();
    let mut route_count = 0;

    for neighbour in input.get_cardinal_neighbours(current_coord) {
        if *neighbour.value == current_value + 1 {
            let (neighbour_nines, neighbour_routes) = find_route(input, neighbour.location);
            found_nines.extend(neighbour_nines);
            route_count += neighbour_routes;
        }
    }

    (found_nines, route_count)
}

fn parse_input(input: &str) -> Grid<u32> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(|x| x.to_digit(10).unwrap()).collect())
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).0, 36);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).1, 81);
    }
}
