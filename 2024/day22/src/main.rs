use std::ops::{Shl, Shr};

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

fn part_1(input: &[u64]) -> u64 {
    let mut part_1_output = 0;
    for value in input {
        let mut current = *value;
        for _ in 0..2000 {
            current = get_next_secret(current);
        }
        part_1_output += current;
    }

    part_1_output
}

fn part_2(input: &[u64]) -> i32 {
    let mut vendor_patterns = Vec::new();
    let mut vendor_digits = Vec::new();

    for value in input {
        let mut current = *value;
        let mut digits: Vec<i32> = Vec::with_capacity(2000);
        let mut differences: Vec<i32> = Vec::with_capacity(2000);

        digits.push((current % 10).try_into().unwrap());
        differences.push(0);

        for _ in 0..2000 {
            current = get_next_secret(current);
            let digit = (current % 10).try_into().unwrap();
            differences.push(digit - digits.last().unwrap());
            digits.push(digit);
        }

        assert_eq!(digits.len(), 2001);
        // println!("{differences:?}");

        vendor_digits.push(digits);
        vendor_patterns.push(differences);
    }

    let patterns = generate_valid_difference_patterns();

    let mut max_bananas = 0;
    for pattern in patterns {
        let mut current_banana_count = 0;
        for (vendor_index, vendor_pattern) in vendor_patterns.iter().enumerate() {
            if let Some((index, _)) = vendor_pattern
                .windows(4)
                .enumerate()
                .find(|x| *x.1 == pattern)
            {
                current_banana_count += vendor_digits[vendor_index][index + 3];
            }
        }
        max_bananas = max_bananas.max(current_banana_count);
    }

    max_bananas
}

#[inline]
fn get_next_secret(mut value: u64) -> u64 {
    value ^= value.shl(6); // Mix
    value &= 16777215; // Prune
    value ^= value.shr(5);
    value &= 16777215;
    value ^= value.shl(11);
    value &= 16777215;

    value
}

fn generate_valid_difference_patterns() -> Vec<Vec<i32>> {
    let mut patterns = Vec::new();
    for first in -9..=9 {
        for second in -9..=9 {
            let total = first + second;
            if !(-9..=9).contains(&total) {
                // This is invalid
                continue;
            }
            for third in -9..=9 {
                let total = total + third;
                if !(-9..=9).contains(&total) {
                    // This is invalid
                    continue;
                }
                for fourth in -9..=9 {
                    let total = total + fourth;
                    if !(-9..=9).contains(&total) {
                        // This is invalid
                        continue;
                    }
                    patterns.push(Vec::from([first, second, third, fourth]));
                }
            }
        }
    }
    patterns
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 37327623);
    }

    #[test]
    fn part_2_test() {
        let input = [1, 2, 3, 2024];
        assert_eq!(part_2(&input), 23);
    }

    #[test]
    fn get_secret_test() {
        let input = 123;
        let next_secret = get_next_secret(input);
        assert_eq!(next_secret, 15887950);
    }

    #[test]
    fn get_pattern_test() {
        let patterns = generate_valid_difference_patterns();
        println!("{}", patterns.len());
        panic!()
    }
}
