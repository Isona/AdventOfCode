use bitvec::prelude::*;

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

fn part_1(input: &[Machine]) -> u64 {
    todo!();
}

fn part_2(input: &[Machine]) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::from).collect()
}

struct Machine {
    goal_pattern: BitVec,
    buttons: Vec<BitVec>,
    joltage: Vec<u64>,
}

impl From<&str> for Machine {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        let mut goal_pattern = bitvec!();
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for current_str in value.split_whitespace() {
            if current_str.starts_with('[') {
                goal_pattern = bitvec![0; current_str.len() - 2];
                let mut current_str_chars = current_str.chars();
                current_str_chars.next();
                current_str_chars.next_back();

                for (index, character) in current_str_chars.enumerate() {
                    if character == '#' {
                        goal_pattern.set(index, true);
                    }
                }
            } else if current_str.starts_with('(') {
            } else if current_str.starts_with('{') {
            }
        }

        Machine {
            goal_pattern,
            buttons,
            joltage,
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
        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
