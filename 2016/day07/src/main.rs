use fancy_regex::Regex;

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

#[allow(clippy::needless_raw_string_hashes)]
fn part_1(input: &str) -> u64 {
    let brackets_check = Regex::new(r#"\[\w*(\w)(?!\1)(\w)\2\1\w*\]"#).unwrap();
    let regex = Regex::new(r#"(\w)(?!\1)(\w)\2\1"#).unwrap();

    let mut count = 0;
    for line in input.lines() {
        if !brackets_check.is_match(line).unwrap() && regex.is_match(line).unwrap() {
            count += 1;
        }
    }
    count
}

#[allow(clippy::needless_raw_string_hashes)]
fn part_2(input: &str) -> u64 {
    let regex = Regex::new(r#"(\w)(?!\1)(\w)\1(?![^\[]*\]).*\2\1\2"#).unwrap();

    let mut count = 0;
    for line in input.lines() {
        if regex.is_match(line).unwrap() {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TESTINPUT), 2);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TESTINPUT2), 3);
    }
}
