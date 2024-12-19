use cached::proc_macro::cached;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = std::time::Instant::now();
    let matched_patterns = part_1(INPUT);
    let part_1_answer = matched_patterns.len();
    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(INPUT, matched_patterns);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &str) -> Vec<&str> {
    let mut iter = input.lines();
    let regex_pattern = get_part_1_regex(iter.next().unwrap());
    let regex = Regex::new(&regex_pattern).unwrap();

    iter.skip(1)
        .filter(|pattern| regex.is_match(pattern))
        .collect()
}

fn get_part_1_regex(input: &str) -> String {
    // For the test data the regex we want looks like this:
    // ^(r|wr|b|g|bwu|rb|gb|br)+$
    // The input looks like this:
    // r, wr, b, g, bwu, rb, gb, br
    format!(r"^({})+$", input.replace(", ", "|"))
}

fn part_2(input: &str, matched_patterns: Vec<&str>) -> u64 {
    let towels: Vec<_> = input.lines().next().unwrap().split(", ").collect();

    matched_patterns
        .iter()
        .map(|pattern| get_ways(pattern, &towels))
        .sum()
}

#[cached(key = "String", convert = r#"{ String::from(pattern_to_match) }"#)]
fn get_ways(pattern_to_match: &str, towel_types: &Vec<&str>) -> u64 {
    let mut sum = 0;
    for towel_type in towel_types {
        if &pattern_to_match == towel_type {
            sum += 1;
        } else if pattern_to_match.starts_with(towel_type) {
            sum += get_ways(
                pattern_to_match.strip_prefix(towel_type).unwrap(),
                towel_types,
            )
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TESTINPUT).len(), 6);
    }

    #[test]
    fn part_2_test() {
        let matched_patterns = part_1(TESTINPUT);
        assert_eq!(part_2(TESTINPUT, matched_patterns), 16);
    }

    #[test]
    fn part_2_1_pattern() {
        let input = "r, rr

rr";
        let matched_patterns = part_1(input);
        assert_eq!(part_2(input, matched_patterns), 2);
    }
}
