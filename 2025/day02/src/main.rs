use std::{collections::HashSet, ops::RangeInclusive};

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

fn part_1(input: &[IDRange]) -> u64 {
    input.iter().map(|x| x.invalid_sum()).sum()
}

fn part_2(input: &[IDRange]) -> u64 {
    input.iter().map(|x| x.invalid_sum_part_2()).sum()
}

fn parse_input(input: &str) -> Vec<IDRange> {
    let mut id_ranges = vec![];
    let ranges_split = input.split(',');
    for range in ranges_split {
        let mut current_split = range.split('-');
        let start = current_split.next().unwrap().parse().unwrap();
        let end = current_split.next().unwrap().parse().unwrap();
        id_ranges.push(IDRange { start, end });
    }

    id_ranges
}

#[derive(Debug, Clone, Copy)]
struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    fn invalid_sum(&self) -> u64 {
        let mut invalid_sum = 0;
        let range = self.start..=self.end;

        let start_invalid_index = get_invalid_index(self.start);
        let end_invalid_index = get_invalid_index(self.end);

        for invalid_index in start_invalid_index..=end_invalid_index {
            let invalid = get_invalid_by_index(invalid_index);
            if range.contains(&invalid) {
                invalid_sum += invalid;
            }
        }
        invalid_sum
    }

    fn invalid_sum_part_2(&self) -> u64 {
        // Use a hashset rather than just summing straight away because of possible duplicates
        // Example: 1 and 11 would both generate 1111 for 4 digits
        let mut invalids_in_range = HashSet::new();

        // Convert for example 95-115 into 95..=99, 100..=115
        let internal_ranges = self.get_internal_ranges();

        for internal_range in internal_ranges {
            // Get the digit count and loop over its factors - these are the number of digits that form possible patterns
            let digit_count = get_digit_count(*internal_range.start());
            // For each possible pattern length, get the first and last pattern
            for pattern_length in get_factors(get_digit_count(*internal_range.start())) {
                let pattern_start =
                    get_first_n_digits(*internal_range.start(), (pattern_length).into());
                let pattern_end =
                    get_first_n_digits(*internal_range.end(), (pattern_length).into());

                // Loop over the possible patterns, duplicate them and add them to the hashset if they're in the current range
                for pattern_index in pattern_start..=pattern_end {
                    let invalid = get_invalid(pattern_index, (digit_count / pattern_length).into());
                    if internal_range.contains(&invalid) {
                        invalids_in_range.insert(invalid);
                    }
                }
            }
        }
        invalids_in_range.iter().sum()
    }

    fn get_internal_ranges(&self) -> Vec<RangeInclusive<u64>> {
        let mut current_start = self.start;
        let current_end = self.end;
        let mut ranges = Vec::new();

        while get_digit_count(current_start) != get_digit_count(current_end) {
            let next_power_of_10 = get_next_power_of_10(current_start);
            ranges.push(current_start..=next_power_of_10 - 1);
            current_start = next_power_of_10;
        }

        ranges.push(current_start..=current_end);

        ranges
    }
}

fn get_invalid_by_index(input: u64) -> u64 {
    input * 10_u64.pow(input.ilog10() + 1) + input
}

fn get_invalid(input: u64, repeats: u64) -> u64 {
    let next_power_of_10 = get_next_power_of_10(input);
    (0..repeats).fold(0, |acc, _| acc * next_power_of_10 + input)
}

fn get_next_power_of_10(input: u64) -> u64 {
    10_u64.pow(get_digit_count(input))
}

fn get_first_n_digits(input: u64, digit_count: u64) -> u64 {
    input / 10u64.pow(get_digit_count(input) - digit_count as u32)
}

fn get_invalid_index(input: u64) -> u64 {
    let input_log10 = input.ilog10();
    if input_log10.is_multiple_of(2) {
        10_u64.pow(input_log10 / 2)
    } else {
        input / 10_u64.pow(input_log10.div_ceil(2))
    }
}

fn get_digit_count(input: u64) -> u32 {
    input.ilog10() + 1
}

fn get_factors(input: u32) -> impl Iterator<Item = u32> {
    (1..=input / 2).filter(move |i| input.is_multiple_of(*i))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        dbg!(&input);
        assert_eq!(part_1(&input), 1227775554);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 4174379265);
    }

    #[test]
    fn test_factors() {
        assert_eq!(get_factors(10).collect::<Vec<u32>>(), vec![1, 2, 5]);
        assert_eq!(get_factors(7).collect::<Vec<u32>>(), vec![1]);
        assert_eq!(get_factors(9).collect::<Vec<u32>>(), vec![1, 3]);
        assert_eq!(
            get_factors(24).collect::<Vec<u32>>(),
            vec![1, 2, 3, 4, 6, 8, 12]
        );
    }

    #[test]
    fn test_digit_count() {
        assert_eq!(get_digit_count(96952600), 8);
        assert_eq!(get_digit_count(1235), 4);
        assert_eq!(get_digit_count(12356), 5);
        assert_eq!(get_digit_count(9), 1);
    }

    #[test]
    fn test_next_power_of_10() {
        assert_eq!(get_next_power_of_10(96952600), 100000000);
        assert_eq!(get_next_power_of_10(15), 100);
    }

    #[test]
    fn test_get_invalid() {
        assert_eq!(get_invalid(10, 5), 1010101010);
        assert_eq!(get_invalid(1, 7), 1111111);
        assert_eq!(get_invalid(1234, 2), 12341234);
    }

    #[test]
    fn test_get_n_digits() {
        assert_eq!(get_first_n_digits(900, 1), 9);
        assert_eq!(get_first_n_digits(900, 2), 90);
        assert_eq!(get_first_n_digits(900, 3), 900);
        assert_eq!(get_first_n_digits(1188511880, 5), 11885);
    }
}
