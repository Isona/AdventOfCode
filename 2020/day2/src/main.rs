const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &[PasswordPolicy]) -> i32 {
    let mut valid_passwords = 0;

    for policy in input {
        if policy.is_valid_part1() {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn part_2(input: &[PasswordPolicy]) -> i32 {
    let mut valid_passwords = 0;

    for policy in input {
        if policy.is_valid_part2() {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

struct PasswordPolicy {
    minimum: usize,
    maximum: usize,
    letter: char,
    password: Vec<char>,
}

impl PasswordPolicy {
    fn get_password_policy(input: &str) -> PasswordPolicy {
        // 1-3 a: abcde
        let mut parts = input.split(" ");
        let mut numbers = parts.next().unwrap().split("-");

        let minimum = numbers.next().unwrap().parse().unwrap();
        let maximum = numbers.next().unwrap().parse().unwrap();

        let letter = parts.next().unwrap().chars().next().unwrap();

        let password = parts.next().unwrap().chars().collect();

        PasswordPolicy {
            minimum,
            maximum,
            letter,
            password,
        }
    }

    fn is_valid_part1(&self) -> bool {
        let letter_count = self.password.iter().filter(|x| x == &&self.letter).count();

        letter_count >= self.minimum && letter_count <= self.maximum
    }

    fn is_valid_part2(&self) -> bool {
        (self.password[self.minimum - 1] == self.letter)
            ^ (self.password[self.maximum - 1] == self.letter)
    }
}

fn parse_input(input: &str) -> Vec<PasswordPolicy> {
    input
        .lines()
        .map(PasswordPolicy::get_password_policy)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 1);
    }
}
