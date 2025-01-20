use std::collections::HashMap;

use aoc_lib::{Direction, Grid};
use petgraph::{algo::dijkstra, prelude::UnGraphMap};

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

fn part_1(grid: &Grid<char>) -> usize {
    let mut graph = UnGraphMap::new();
    let mut warp_points = HashMap::new();

    // Create a graph out of the grid
    for location in grid.find_all(&'.') {
        for neighbour in grid.get_cardinal_neighbours(location) {
            // If this is a warp point then calculate the string for it
            // Then either add it to the warp_points hashmap
            // if we know the other half of the warp_point then add to the graph
            if neighbour.value.is_ascii_uppercase() {
                let other_half = grid
                    .get_neighbour(neighbour.location, neighbour.direction)
                    .unwrap();

                let warp_string = {
                    match neighbour.direction {
                        Direction::North => format!("{}{}", other_half.value, neighbour.value),
                        Direction::South => format!("{}{}", neighbour.value, other_half.value),
                        Direction::East => format!("{}{}", neighbour.value, other_half.value),
                        Direction::West => format!("{}{}", other_half.value, neighbour.value),
                        _ => panic!(),
                    }
                };

                if let Some(other_warp_location) = warp_points.get(&warp_string) {
                    graph.add_edge(location, *other_warp_location, 1);
                } else {
                    warp_points.insert(warp_string, location);
                }
            // If the neighbour is a . then add it to the graph
            } else if neighbour.value == &'.' {
                graph.add_edge(location, neighbour.location, 1);
            }
        }
    }

    let aa_location = warp_points.get("AA").unwrap();
    let zz_location = warp_points.get("ZZ").unwrap();
    *dijkstra(&graph, *aa_location, Some(*zz_location), |_| 1)
        .get(zz_location)
        .unwrap()
}

fn part_2(grid: &Grid<char>) -> u64 {
    let mut graph = UnGraphMap::new();
    let mut outer_warp_points = HashMap::new();
    let mut inner_warp_points = HashMap::new();
    // Creating this many layers of the grid
    let layer_count = 26;

    for location in grid.find_all(&'.') {
        for neighbour in grid.get_cardinal_neighbours(location) {
            // If this is a warp point then calculate the string for it
            // Then work out if it's an outer or inner warp point and store in the relevant hashmap
            if neighbour.value.is_ascii_uppercase() {
                let other_half = grid
                    .get_neighbour(neighbour.location, neighbour.direction)
                    .unwrap();

                let warp_string = {
                    match neighbour.direction {
                        Direction::North => format!("{}{}", other_half.value, neighbour.value),
                        Direction::South => format!("{}{}", neighbour.value, other_half.value),
                        Direction::East => format!("{}{}", neighbour.value, other_half.value),
                        Direction::West => format!("{}{}", other_half.value, neighbour.value),
                        _ => panic!(),
                    }
                };

                // If the further part of the warp label is on an edge
                // then we know it's an outer warp point
                if grid.is_on_edge(&other_half.location) {
                    outer_warp_points.insert(warp_string, location);
                } else {
                    inner_warp_points.insert(warp_string, location);
                }
            // If the neighbour is a . then add it to the graph
            } else if neighbour.value == &'.' {
                // Add this pair for each layer we need
                for layer_number in 0..layer_count {
                    graph.add_edge(
                        (location, layer_number),
                        (neighbour.location, layer_number),
                        1,
                    );
                }
            }
        }
    }

    // Add the warp points with the correct layer numbers to the graph
    for (warp_string, inner_warp_point) in &inner_warp_points {
        if let Some(outer_warp_point) = outer_warp_points.get(warp_string) {
            for layer_number in 0..layer_count {
                graph.add_edge(
                    (*inner_warp_point, layer_number),
                    (*outer_warp_point, layer_number + 1),
                    1,
                );
            }
        }
    }

    let aa_location = outer_warp_points.get("AA").unwrap();
    let zz_location = outer_warp_points.get("ZZ").unwrap();
    *dijkstra(&graph, (*aa_location, 0), Some((*zz_location, 0)), |_| 1)
        .get(&(*zz_location, 0))
        .unwrap()
}

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 58);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_2(&input), 396);
    }
}
