use aoc_lib::{Coordinate, Direction, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = std::time::Instant::now();
    let part_1_answer = part_1(INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &str) -> usize {
    let mut current_total = 0;
    let mut grid: Grid<usize> = Grid::new();
    grid.push_row([1, 2, 3].into());
    grid.push_row([4, 5, 6].into());
    grid.push_row([7, 8, 9].into());

    let mut current_coordinate = Coordinate::new(1, 1);
    for line in input.lines() {
        for value in line.chars() {
            let direction = get_direction(value);
            if let Some(neighbour) = grid.get_neighbour(current_coordinate, direction) {
                current_coordinate = neighbour.location;
            }
        }
        current_total = current_total * 10 + grid.get(current_coordinate);
    }

    current_total
}

fn part_2(input: &str) -> String {
    let mut current_total = String::new();
    let mut grid: Grid<Option<char>> = Grid::new();

    grid.push_row([None, None, Some('1'), None, None].into());
    grid.push_row([None, Some('2'), Some('3'), Some('4'), None].into());
    grid.push_row([Some('5'), Some('6'), Some('7'), Some('8'), Some('9')].into());
    grid.push_row([None, Some('A'), Some('B'), Some('C'), None].into());
    grid.push_row([None, None, Some('D'), None, None].into());

    let mut current_coordinate = Coordinate::new(0, 2);
    for line in input.lines() {
        for value in line.chars() {
            let direction = get_direction(value);
            if let Some(neighbour) = grid.get_neighbour(current_coordinate, direction) {
                if neighbour.value.is_some() {
                    current_coordinate = neighbour.location;
                }
            }
        }
        current_total.push(grid.get(current_coordinate).unwrap());
    }

    current_total
}

fn get_direction(input: char) -> Direction {
    match input {
        'U' => Direction::North,
        'L' => Direction::West,
        'R' => Direction::East,
        'D' => Direction::South,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TESTINPUT), 1985);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TESTINPUT), "5DB3");
    }
}
