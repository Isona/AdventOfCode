use aoc_lib::{Coordinate, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (grid, operands) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&grid, &operands);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(grid: &Grid<u128>, operands: &[char]) -> u128 {
    let mut total_of_sums = 0;

    for i in 0..grid.get_width() {
        let current_sum: u128 = match operands[i] {
            '+' => grid.get_column(i).map(|x| grid.get(x)).sum(),
            '*' => grid.get_column(i).map(|x| grid.get(x)).product(),
            _ => panic!(),
        };

        total_of_sums += current_sum;
    }

    total_of_sums
}

fn part_2(input: &str) -> u128 {
    let mut grid: Grid<char> = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().rev().collect());
    }

    let mut current_sum_digits = Vec::new();

    let mut total_of_sums = 0;

    for i in 0..grid.get_width() {
        let current_value = grid
            .get_column(i)
            .map(|x| grid.get(x))
            .filter_map(|x| x.to_digit(10))
            .fold(0u128, |acc, x| acc * 10 + x as u128);

        if current_value != 0 {
            current_sum_digits.push(current_value)
        };

        let current_sum: u128 = match grid.get(Coordinate {
            x: i,
            y: grid.get_height() - 1,
        }) {
            '+' => current_sum_digits.drain(..).sum(),
            '*' => current_sum_digits.drain(..).product(),
            _ => {
                continue;
            }
        };

        total_of_sums += current_sum;
    }

    total_of_sums
}

fn parse_input(input: &str) -> (Grid<u128>, Vec<char>) {
    let mut line_iter = input.lines().rev();
    let operands_line = line_iter.next().unwrap();
    let operands = operands_line
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();
    let mut grid = Grid::new();
    for line in line_iter.rev() {
        grid.push_row(
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }
    (grid, operands)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (grid, operands) = parse_input(TESTINPUT);
        assert_eq!(part_1(&grid, &operands), 4277556);
    }

    #[test]
    fn part_2_test() {
        //let (grid, operands) = parse_input(TESTINPUT);
        assert_eq!(part_2(&TESTINPUT), 3263827);
    }
}
