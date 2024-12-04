use aoc_lib::{Coordinate, Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &Grid<char>) -> u64 {
    let mut count = 0;
    for (coord, value) in input.indexed_iter() {
        if value == &'X' {
            count += xmas_matches_from_coord(input, coord)
        }
    }

    count
}

fn xmas_matches_from_coord(input: &Grid<char>, x_coord: Coordinate) -> u64 {
    let mut count = 0;

    for (m_coord, direction) in input.get_all_neighbour_coords_with_direction(x_coord) {
        if input.get(m_coord) == &'M' {
            if let Some(a_coord) = input.get_neighbour(m_coord, direction) {
                if input.get(a_coord) == &'A' {
                    if let Some(s_coord) = input.get_neighbour(a_coord, direction) {
                        if input.get(s_coord) == &'S' {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn part_2(input: &Grid<char>) -> u64 {
    let mut count = 0;
    for (coord, value) in input.indexed_iter() {
        if value == &'A' && is_cross_mas(input, coord) {
            count += 1
        }
    }

    count
}

fn is_cross_mas(input: &Grid<char>, a_coord: Coordinate) -> bool {
    let neighbours = input.get_all_neighbour_coords_with_direction(a_coord);
    // If there are fewer than 8 neighbours then this is an edge and can't be a cross
    if neighbours.len() != 8 {
        return false;
    }
    let mut arm_count = 0;
    let intercardinals = Direction::get_intercardinals();
    for (m_coord, m_direction) in &neighbours {
        if intercardinals.contains(m_direction) && input.get(*m_coord) == &'M' {
            let s_direction = m_direction.get_opposite();
            for (s_coord, check_direction) in &neighbours {
                if check_direction == &s_direction && input.get(*s_coord) == &'S' {
                    arm_count += 1;
                }
            }
        }
    }

    arm_count == 2
}

fn parse_input(input: &str) -> Grid<char> {
    let mut grid: Grid<char> = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().collect());
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
        assert_eq!(part_1(&input), 18);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 9);
    }
}