use aoc_lib::Grid;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &Grid<u32>) -> u64 {
    todo!();
}

fn part_2(input: &Grid<u32>) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Grid<u32> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().map(|x| x.to_digit(10).unwrap()).collect())
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
        assert_eq!(part_1(&input), 36);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
