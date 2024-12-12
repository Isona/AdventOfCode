use std::collections::{HashMap, VecDeque};

use aoc_lib::{Coordinate, Direction, Grid, Neighbour};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1(&input);
    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

#[expect(clippy::map_entry)]
fn part_1(input: &Grid<char>) -> (usize, usize) {
    // Hashmap will contain the region and the number of non-region neighbours (for the part 1perimeter)
    let mut regions: HashMap<Coordinate, (u32, u32)> = HashMap::new();
    let mut next_region_number = 0u32;

    for (current_coord, _current_value) in input.indexed_iter() {
        // If we've already checked this one then we can move on
        if regions.contains_key(&current_coord) {
            continue;
        }

        let mut to_search = VecDeque::from([current_coord]);
        // We want to find all the locations in this region
        while let Some(search_coord) = to_search.pop_front() {
            let value = input.get(search_coord);
            let mut perimeter = 0;
            let neighbours: Vec<Neighbour<char>> =
                input.get_cardinal_neighbours(search_coord).collect();

            // Account for neighbours in invalid coordinates
            perimeter += 4 - neighbours.len();

            for neighbour in neighbours {
                if neighbour.value != value {
                    perimeter += 1;
                } else if !regions.contains_key(&neighbour.location) {
                    regions.insert(neighbour.location, (next_region_number, 0));
                    to_search.push_back(neighbour.location);
                }
            }

            regions.insert(
                search_coord,
                (next_region_number, perimeter.try_into().unwrap()),
            );
        }

        next_region_number += 1;
    }

    let mut part_1_output = 0;
    let mut part_2_output = 0;
    for region_number in 0..next_region_number {
        let region_members: Vec<(&Coordinate, &(u32, u32))> = regions
            .iter()
            .filter(|(_, (region, _))| *region == region_number)
            .collect();

        let area = region_members.len();

        // Part 1 perimeter calculation
        let part_1_perimeter: u32 = region_members
            .iter()
            .map(|(_, (_, coord_edges))| coord_edges)
            .sum();

        part_1_output += area * part_1_perimeter as usize;

        // Part 2 perimeter calculation
        let mut corner_count = 0;
        for (member_coord, _) in region_members {
            // Get cardinal matches
            let north_match = input.matches_neighbour(*member_coord, Direction::North);
            let south_match = input.matches_neighbour(*member_coord, Direction::South);
            let east_match = input.matches_neighbour(*member_coord, Direction::East);
            let west_match = input.matches_neighbour(*member_coord, Direction::West);

            // Exterior corners
            let mut north_south = 0;
            if !north_match {
                north_south += 1;
            }
            if !south_match {
                north_south += 1;
            }

            if !east_match {
                corner_count += north_south;
            }
            if !west_match {
                corner_count += north_south;
            }

            // Interior corners

            // NE Interior
            if north_match
                && east_match
                && !input.matches_neighbour(*member_coord, Direction::NorthEast)
            {
                corner_count += 1;
            }

            // NW Interior
            if north_match
                && west_match
                && !input.matches_neighbour(*member_coord, Direction::NorthWest)
            {
                corner_count += 1;
            }

            // SE Interior
            if south_match
                && east_match
                && !input.matches_neighbour(*member_coord, Direction::SouthEast)
            {
                corner_count += 1;
            }

            // SW Interior
            if south_match
                && west_match
                && !input.matches_neighbour(*member_coord, Direction::SouthWest)
            {
                corner_count += 1;
            }
        }

        // println!("Region {region_number}, corners: {corner_count}, area: {area}");
        part_2_output += area * corner_count;
    }

    (part_1_output, part_2_output)
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
    const TESTINPUT1: &str = include_str!("testinput1.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test_1() {
        let input = parse_input(TESTINPUT1);
        assert_eq!(part_1(&input).0, 140);
    }

    #[test]
    fn part_1_test_2() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_1(&input).0, 1930);
    }

    #[test]
    fn part_2_test_1() {
        let input = parse_input(TESTINPUT1);
        assert_eq!(part_1(&input).1, 80);
    }

    #[test]
    fn part_2_test_2() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_1(&input).1, 1206);
    }
}
