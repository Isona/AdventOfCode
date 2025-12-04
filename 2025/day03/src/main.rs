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

fn part_1(input: &[BatteryBank]) -> u64 {
    input.iter().map(BatteryBank::get_max_joltage).sum()
}

fn part_2(input: &[BatteryBank]) -> u128 {
    input.iter().map(BatteryBank::get_max_joltage_part_2).sum()
}

fn parse_input(input: &str) -> Vec<BatteryBank> {
    input.lines().map(BatteryBank::from).collect()
}

struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    fn get_max_joltage(&self) -> u64 {
        assert!(self.batteries.len() > 5);
        let mut first_digit = self.batteries[self.batteries.len() - 2];
        let mut second_digit = *self.batteries.last().unwrap();

        for digit in self.batteries[0..self.batteries.len() - 2].iter().rev() {
            if *digit >= first_digit {
                if first_digit > second_digit {
                    second_digit = first_digit;
                }
                first_digit = *digit;
            }
        }

        (first_digit * 10 + second_digit).into()
    }

    fn get_max_joltage_part_2(&self) -> u128 {
        let mut max_digits: Vec<u32> =
            self.batteries[self.batteries.len() - 12..self.batteries.len()].to_vec();

        let mut search_start_index = 0;
        let mut last_possible_index = self.batteries.len() - 11;

        for current_digit in max_digits.iter_mut() {
            if search_start_index + 1 == last_possible_index {
                break;
            }

            let (last_digit_index, max_digit) = self.batteries[..last_possible_index]
                .iter()
                .enumerate()
                .skip(search_start_index)
                .rev()
                .max_by_key(|x| x.1)
                .unwrap();

            *current_digit = *max_digit;
            search_start_index = last_digit_index + 1;
            last_possible_index += 1;
        }

        max_digits.iter().fold(0, |acc, x| acc * 10 + *x as u128)
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries = value.chars().map(|x| x.to_digit(10).unwrap()).collect();
        BatteryBank { batteries }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 357);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 3121910778619);
    }
}
