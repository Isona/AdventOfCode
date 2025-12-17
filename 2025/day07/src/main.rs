use std::collections::HashMap;

use aoc_lib::{Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1_and_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} - calculated at the same time");
}

fn part_1_and_2(grid: &Grid<CellType>) -> (usize, u64) {
    let mut total_paths = 0;

    // Add the first splitter hit from the start point to hit_splitters
    //
    let mut hit_splitters = HashMap::new();
    let start_point = grid.find_item(&CellType::Start).unwrap();
    let first_splitter = grid
        .view_from(&start_point, Direction::South)
        .find(|x| x.value == &CellType::Splitter)
        .unwrap();
    hit_splitters.insert(first_splitter.location, 1);

    // Just iterate over each even row looking for splitters
    // Store them in hit_splitters with the number of paths that could reach them
    for row_number in (0..grid.get_height()).step_by(2) {
        // Get the splitters in the current row and iterate over their coordinates
        for current_splitter in grid
            .get_row(row_number)
            .filter(|coord| grid.get(*coord) == &CellType::Splitter)
        {
            // Get the current splitter value, copy it using shadowing to avoid borrowing hit_splitters
            let Some(current_splitter_value) = hit_splitters.get(&current_splitter) else {
                continue;
            };
            let current_splitter_value = *current_splitter_value;

            // If there is a west neighbour then we have a laser!
            if let Some(west_laser) = grid.get_neighbour(current_splitter, Direction::West) {
                // Try to find the splitter below, if we find it, then add it to hit_splitters
                // or add the current_value to that entry in hit_splitters
                if let Some(sw_splitter) = grid
                    .view_from(&west_laser.location, Direction::South)
                    .find(|x| x.value == &CellType::Splitter)
                {
                    hit_splitters
                        .entry(sw_splitter.location)
                        .and_modify(|x| *x += current_splitter_value)
                        .or_insert(current_splitter_value);
                // If we couldn't find a splitter below, then this is the end of a path, add it to total_paths
                } else {
                    total_paths += current_splitter_value;
                }
            }

            // If there is an east neighbour then we have a laser!
            if let Some(east_laser) = grid.get_neighbour(current_splitter, Direction::East) {
                // Try to find the splitter below, if we find it, then add it to hit_splitters
                // or add the current_value to that entry in hit_splitters
                if let Some(sw_splitter) = grid
                    .view_from(&east_laser.location, Direction::South)
                    .find(|x| x.value == &CellType::Splitter)
                {
                    hit_splitters
                        .entry(sw_splitter.location)
                        .and_modify(|x| *x += current_splitter_value)
                        .or_insert(current_splitter_value);
                // If we couldn't find a splitter below, then this is the end of a path, add it to total_paths
                } else {
                    total_paths += current_splitter_value;
                }
            }
        }
    }

    // The part 1 answer is the length of hit_splitters
    (hit_splitters.len(), total_paths)
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
enum CellType {
    Start,
    #[default]
    Empty,
    Splitter,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            'S' => CellType::Start,
            '.' => CellType::Empty,
            '^' => CellType::Splitter,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1_and_2(&input).0, 21);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1_and_2(&input).1, 40);
    }
}
