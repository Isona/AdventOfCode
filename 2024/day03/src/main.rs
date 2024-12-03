use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,}),(\d{1,})\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let re = Regex::new(r"mul\((?<x>\d{1,}),(?<y>\d{1,})\)|(do\(\))|(don't\(\))").unwrap();

    let mut mul_enabled = true;
    let mut sum = 0;
    for capture in re.captures_iter(input) {
        let command = capture.get(0).unwrap().as_str();
        if command.starts_with("don't") {
            mul_enabled = false;
        } else if command.starts_with("do") {
            mul_enabled = true;
        } else if command.starts_with("mul") && mul_enabled {
            sum += capture.name("x").unwrap().as_str().parse::<u64>().unwrap()
                * capture.name("y").unwrap().as_str().parse::<u64>().unwrap();
        }
    }

    sum
}

fn parse_input(input: &str) -> String {
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 161);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_2(&input), 48);
    }
}
