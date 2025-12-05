use aoc_lib::{Coordinate, Grid};

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

fn part_1(input: &Grid<u32>) -> u32 {
    let mut sum = 0;
    for (coord, current_value) in input.indexed_iter() {
        if input
            .get_cardinal_neighbours(coord)
            .all(|x| current_value < x.value)
        {
            sum += current_value + 1;
        }
    }

    sum
}

fn part_2(grid: &Grid<u32>) -> u64 {
    let mut visited_grid = Grid::new();
    let grid_width = grid.get_width();
    for _ in 0..grid.get_height() {
        visited_grid.push_row(vec![false; grid_width]);
    }

    let mut basin_sizes = Vec::new();
    for (current_coord, _) in grid.indexed_iter().filter(|x| x.1 != &9) {
        if !visited_grid.get(current_coord) {
            let mut visited_count = 0;
            dfs(grid, current_coord, &mut visited_grid, &mut visited_count);
            basin_sizes.push(visited_count);
        }
    }

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

fn dfs(
    grid: &Grid<u32>,
    current_coord: Coordinate,
    visited: &mut Grid<bool>,
    basin_size: &mut u64,
) {
    // Set this node to visited, keeping track of the basin size
    visited.set(current_coord, true);
    *basin_size += 1;

    // Loop over all unvisited neighbours that aren't 9 and recurse on them
    for neighbour in grid
        .get_cardinal_neighbours(current_coord)
        .filter(|neighbour| neighbour.value != &9)
    {
        if !visited.get(neighbour.location) {
            dfs(grid, neighbour.location, visited, basin_size);
        }
    }
}

fn parse_input(input: &str) -> Grid<u32> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
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
        assert_eq!(part_1(&input), 15);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 1134);
    }
}
