const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for line in input {
        sum += get_calibration(line);
    }
    sum
}

fn get_calibration(input: &str) -> u32 {
    let digits: Vec<char> = input.chars().filter(|x| x.is_ascii_digit()).collect();
    format!("{}{}", digits[0], digits[digits.len() - 1])
        .parse::<u32>()
        .unwrap()
}

fn part_2(input: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for line in input {
        sum += get_calibration(&part2_digit_replace(line));
    }
    sum
}

fn part2_digit_replace(input: &str) -> String {
    let mut output = str::replace(input, "one", "o1ne");
    output = str::replace(&output, "two", "tw2o");
    output = str::replace(&output, "three", "thr3ee");
    output = str::replace(&output, "four", "fo4ur");
    output = str::replace(&output, "five", "fi5ve");
    output = str::replace(&output, "six", "si6x");
    output = str::replace(&output, "seven", "se7ven");
    output = str::replace(&output, "eight", "eig8ht");
    output = str::replace(&output, "nine", "ni9ne");
    output
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 142);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_2(&input), 281);
    }
}
