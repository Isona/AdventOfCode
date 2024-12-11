use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &HashMap<u64, u64>) -> u64 {
    do_n_iterations(input, 25)
}

fn part_2(input: &HashMap<u64, u64>) -> u64 {
    do_n_iterations(input, 75)
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();
    for stone in input.split_whitespace() {
        stones.insert(stone.parse::<u64>().unwrap(), 1);
    }

    stones
}

fn do_n_iterations(input: &HashMap<u64, u64>, count: u64) -> u64 {
    assert!(count >= 1);

    let mut output_map = do_iteration(input);
    for _ in 0..count - 1 {
        output_map = do_iteration(&output_map);
    }

    output_map.iter().map(|x| x.1).sum()
}

fn do_iteration(input: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut output: HashMap<u64, u64> = HashMap::new();

    for (stone_type, stone_count) in input.iter() {
        // If the stone is 0
        if *stone_type == 0 {
            output
                .entry(1)
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);
        }
        // If the string length is divisible by 2
        else if stone_type.to_string().len() % 2 == 0 {
            let stone_string = stone_type.to_string();
            let (first_half, second_half) = stone_string.split_at(stone_string.len() / 2);
            output
                .entry(first_half.parse::<u64>().unwrap())
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);
            output
                .entry(second_half.parse::<u64>().unwrap())
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);
        // Otherwise multiply by 2024
        } else {
            output
                .entry(stone_type * 2024)
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_1_iteration() {
        let input = parse_input("0 1 10 99 999");
        let output = do_n_iterations(&input, 1);
        assert_eq!(output, 7);
    }

    #[test]
    fn part_1_6_iterations() {
        let input = parse_input("125 17");
        let output = do_n_iterations(&input, 6);
        assert_eq!(output, 22);
    }

    #[test]
    fn part_1_25_iterations() {
        let input = parse_input("125 17");
        let output = do_n_iterations(&input, 25);
        assert_eq!(output, 55312);
    }
}
