use std::cmp::{max, min};

use aoc_lib::{Coordinate, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, grid) = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input, grid);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &[Line]) -> (usize, Grid<i32>) {
    let mut grid = Grid::new();
    for _ in 0..1000 {
        grid.push_row(vec![0; 1000]);
    }

    for line in input.iter().filter(|line| line.is_vertical_or_horizontal()) {
        if line.line_type == LineType::Horizontal {
            let mut current_coord = Coordinate {
                x: 0,
                y: line.start.y,
            };

            for x in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
                current_coord.x = x;
                let current_value = *grid.get(current_coord);
                grid.set(current_coord, current_value + 1);
            }
        } else if line.line_type == LineType::Vertical {
            let mut current_coord = Coordinate {
                x: line.start.x,
                y: 0,
            };

            for y in min(line.start.y, line.end.y)..=max(line.start.y, line.end.y) {
                current_coord.y = y;
                let current_value = *grid.get(current_coord);
                grid.set(current_coord, current_value + 1);
            }
        }
    }
    (grid.find_all_by(|x| x >= &2).count(), grid)
}

fn part_2(input: &[Line], mut grid: Grid<i32>) -> usize {
    for line in input
        .iter()
        .filter(|line| !line.is_vertical_or_horizontal())
    {
        //Positive diagonal
        if (line.start.x > line.end.x && line.start.y > line.end.y)
            || (line.start.x < line.end.x && line.start.y < line.end.y)
        {
            let mut current_pos = Coordinate {
                x: min(line.start.x, line.end.x),
                y: min(line.start.y, line.end.y),
            };

            for _ in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
                let current_value = *grid.get(current_pos);
                grid.set(current_pos, current_value + 1);
                current_pos.x += 1;
                current_pos.y += 1;
            }
        }
        // Negative diagonal
        else {
            let mut current_pos = Coordinate {
                x: min(line.start.x, line.end.x),
                y: max(line.start.y, line.end.y),
            };

            for _ in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
                let current_value = *grid.get(current_pos);
                grid.set(current_pos, current_value + 1);
                if current_pos.y == 0 {
                    break;
                }
                current_pos.x += 1;
                current_pos.y -= 1;
            }
        }
    }

    grid.find_all_by(|x| x >= &2).count()
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut parsed_lines = Vec::new();
    for input_line in input.lines() {
        let mut split = input_line.split(" -> ");

        let mut start_split = split.next().unwrap().split(",");
        let start_x = start_split.next().unwrap().parse().unwrap();
        let start_y = start_split.next().unwrap().parse().unwrap();
        let start = Coordinate {
            x: start_x,
            y: start_y,
        };

        let mut end_split = split.next().unwrap().split(",");
        let end_x = end_split.next().unwrap().parse().unwrap();
        let end_y = end_split.next().unwrap().parse().unwrap();
        let end = Coordinate { x: end_x, y: end_y };

        let line_type = if start.x == end.x {
            LineType::Vertical
        } else if start.y == end.y {
            LineType::Horizontal
        } else {
            LineType::Diagonal
        };

        let line = Line {
            start,
            end,
            line_type,
        };
        parsed_lines.push(line);
    }
    parsed_lines
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Coordinate,
    end: Coordinate,
    line_type: LineType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineType {
    Vertical,
    Horizontal,
    Diagonal,
}

impl Line {
    fn is_vertical_or_horizontal(&self) -> bool {
        self.line_type == LineType::Horizontal || self.line_type == LineType::Vertical
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).0, 5);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        let (_, grid) = part_1(&input);
        assert_eq!(part_2(&input, grid), 12);
    }
}
