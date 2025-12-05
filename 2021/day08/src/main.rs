use std::collections::HashSet;

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

fn part_1(input: &[SevenSegment]) -> u64 {
    let mut out_count = 0;
    for seven_segment in input {
        for output_val in &seven_segment.output {
            match output_val.len() {
                2 | 3 | 4 | 7 => out_count += 1,
                _ => {}
            }
        }
    }

    out_count
}

fn part_2(input: &[SevenSegment]) -> usize {
    input.iter().map(SevenSegment::get_output_value).sum()
}

struct SevenSegment {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl SevenSegment {
    fn get_output_value(&self) -> usize {
        let empty_hashset = HashSet::new();
        let mut digits: Vec<&HashSet<char>> = vec![&empty_hashset; 10];

        for input_val in &self.input {
            match input_val.len() {
                // One
                2 => digits[1] = input_val,
                // Seven
                3 => digits[7] = input_val,
                // Four
                4 => digits[4] = input_val,
                // Eight
                7 => digits[8] = input_val,
                _ => {}
            }
        }

        for input_val in self.input.iter().filter(|x| x.len() == 6) {
            // Nine
            if input_val.is_superset(digits[4]) {
                digits[9] = input_val;
            }
            // Zero
            else if input_val.is_superset(digits[7]) {
                digits[0] = input_val;
            }
            // Six
            else {
                digits[6] = input_val;
            }
        }

        for input_val in self.input.iter().filter(|x| x.len() == 5) {
            // Three
            if input_val.is_superset(digits[1]) {
                digits[3] = input_val;
            } else if input_val.is_subset(digits[9]) {
                digits[5] = input_val;
            } else {
                digits[2] = input_val;
            }
        }

        //        println!("{digits:?}");
        let mut output_total = 0;
        for output_val in &self.output {
            let (index, _) = digits
                .iter()
                .enumerate()
                .find(|x| **x.1 == *output_val)
                .unwrap();
            output_total = output_total * 10 + index;
        }

        output_total
    }
}

fn parse_input(input: &str) -> Vec<SevenSegment> {
    let mut seven_segments = Vec::new();
    for line in input.lines() {
        let mut split = line.split(" | ");
        let line_input = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string().chars().collect())
            .collect();
        let line_output = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string().chars().collect())
            .collect();

        seven_segments.push(SevenSegment {
            input: line_input,
            output: line_output,
        });
    }

    seven_segments
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 26);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 61229);
    }
}
