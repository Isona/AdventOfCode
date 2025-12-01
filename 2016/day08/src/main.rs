use aoc_lib::{Coordinate, Grid};
use std::fmt::Write;
const INPUT: &str = include_str!("input.txt");
const ROW_SIZE: usize = 50;
const COL_SIZE: usize = 6;

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
}

fn part_1(input: &[Instruction]) -> usize {
    let mut grid = Grid::new();
    for _ in 0..COL_SIZE {
        grid.push_row(vec![false; ROW_SIZE]);
    }

    for instruction in input {
        instruction.do_instruction(&mut grid);
    }

    let mut output_string = String::new();
    grid.indexed_iter().for_each(|x| {
        if *x.1 {
            let _ = write!(output_string, "#");
        } else {
            let _ = write!(output_string, " ");
        }
        if x.0.x == ROW_SIZE - 1 {
            let _ = writeln!(output_string);
        }
    });
    println!("{output_string}");
    grid.indexed_iter().filter(|x| *x.1).count()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

#[derive(Debug)]
enum Instruction {
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
    Rect(usize, usize),
}

impl Instruction {
    fn new(input: &str) -> Self {
        if input.starts_with("rotate row") {
            let first_split = input.split('=').next_back().unwrap();
            let mut next_split = first_split.split_whitespace();
            let row_index = next_split.next().unwrap().parse().unwrap();
            let shift_size = next_split.last().unwrap().parse().unwrap();
            Instruction::RotateRow(row_index, shift_size)
        } else if input.starts_with("rotate column") {
            let first_split = input.split('=').next_back().unwrap();
            let mut next_split = first_split.split_whitespace();
            let column_index = next_split.next().unwrap().parse().unwrap();
            let shift_size = next_split.last().unwrap().parse().unwrap();
            Instruction::RotateColumn(column_index, shift_size)
        } else if input.starts_with("rect") {
            let first_split = input.split_whitespace().last().unwrap();
            let mut next_split = first_split.split('x');
            let row_size = next_split.next().unwrap().parse().unwrap();
            let col_size = next_split.next().unwrap().parse().unwrap();
            Instruction::Rect(row_size, col_size)
        } else {
            panic!()
        }
    }

    fn do_instruction(&self, grid: &mut Grid<bool>) {
        match self {
            Instruction::RotateRow(row_index, shift_size) => {
                let initial_row: Vec<bool> = grid
                    .get_row(*row_index)
                    .map(|coord| *grid.get(coord))
                    .collect();

                for (i, value_to_write) in initial_row.iter().enumerate() {
                    let coord_to_write = Coordinate::new((i + *shift_size) % ROW_SIZE, *row_index);
                    grid.set(coord_to_write, *value_to_write);
                }
            }
            Instruction::RotateColumn(column_index, shift_size) => {
                let initial_column: Vec<bool> = grid
                    .get_column(*column_index)
                    .map(|coord| *grid.get(coord))
                    .collect();

                for (i, value_to_write) in initial_column.iter().enumerate() {
                    let coord_to_write =
                        Coordinate::new(*column_index, (i + *shift_size) % COL_SIZE);
                    grid.set(coord_to_write, *value_to_write);
                }
            }
            Instruction::Rect(row_size, column_size) => {
                for x in 0..*row_size {
                    for y in 0..*column_size {
                        grid.set(Coordinate::new(x, y), true);
                    }
                }
            }
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
        dbg!(&input);
        assert_eq!(part_1(&input), 6);
    }
}
