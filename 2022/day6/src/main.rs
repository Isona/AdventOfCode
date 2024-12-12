use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &[char]) -> usize {
    find_unique_window(input, 4)
}

fn part_2(input: &[char]) -> usize {
    find_unique_window(input, 14)
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn find_unique_window(input: &[char], window_len: usize) -> usize {
    for (index, window) in input.windows(window_len).enumerate() {
        let unique_values = HashSet::<&char>::from_iter(window.iter());
        if unique_values.len() == window_len {
            return index + window_len;
        }
    }

    0
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
        assert_eq!(part_2(&input), 19);
    }
}
