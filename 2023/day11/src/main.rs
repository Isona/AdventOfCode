use aoc_lib::Grid;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = get_distances(&input, 2);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = get_distances(&input, 1_000_000);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn get_distances(input: &Grid<Space>, expansion_size: usize) -> i64 {
    let mut score = 0;

    let mut x_values = Vec::new();
    let mut empty_columns = 0;
    for column in (0..input.get_width()).map(|col_num| input.get_column(col_num)) {
        let mut galaxy_found = false;
        for galaxy in column.filter(|x| input.get(*x) == &Space::Galaxy) {
            galaxy_found = true;
            x_values.push(galaxy.x + empty_columns * (expansion_size - 1));
        }

        if !galaxy_found {
            empty_columns += 1;
        }
    }

    for (index, x_value) in x_values.iter().rev().enumerate() {
        score += *x_value as i64 * (x_values.len() as i64 - 1 - index as i64 * 2);
    }

    let mut y_values = Vec::new();
    let mut empty_rows = 0;
    for row in (0..input.get_height()).map(|row_num| input.get_row(row_num)) {
        let mut galaxy_found = false;
        for galaxy in row.filter(|x| input.get(*x) == &Space::Galaxy) {
            galaxy_found = true;
            y_values.push(galaxy.y + empty_rows * (expansion_size - 1));
        }

        if !galaxy_found {
            empty_rows += 1;
        }
    }

    for (index, y_value) in y_values.iter().rev().enumerate() {
        score += *y_value as i64 * (y_values.len() as i64 - 1 - index as i64 * 2);
    }

    score
}

fn parse_input(input: &str) -> Grid<Space> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(Space::from).collect());
    }

    grid
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Space {
    Galaxy,
    #[default]
    Void, // So dead, so black
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '#' => Space::Galaxy,
            '.' => Space::Void,
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
        assert_eq!(get_distances(&input, 2), 374);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(get_distances(&input, 10), 1030);

        assert_eq!(get_distances(&input, 100), 8410);
    }
}
