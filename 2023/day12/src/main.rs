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

fn part_1(input: &Vec<Gears>) -> u64 {
    let mut output = 0;

    for record in input {
        println!("{:?}", record);
        output += record.get_arrangements();
    }

    output
}

fn part_2(input: &Vec<Gears>) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Vec<Gears> {
    input.lines().map(Gears::from).collect()
}

#[derive(Debug)]
struct Gears {
    springs: Vec<Condition>,
    condition_record: Vec<usize>,
    record_pattern_len: usize,
    potential_damaged_springs: usize,
}

impl Gears {
    fn get_arrangements(&self) -> u64 {
        let mut springs = self.springs.as_slice();
        let mut record = self.condition_record.as_slice();

        let mut updated = true;

        while updated {
            updated = false;

            // If the first element is a ., then we can remove it
            if springs[0] == Condition::Operational {
                springs = &springs[1..];
                updated = true;
            }

            // If the first element is #, and the first n elements are unknown or damaged,
            // And the n+1th element is operational or unknown
            // then remove the first record and the first n+1 springs
            if springs[0] == Condition::Damaged {
                if springs[1..record[0]]
                    .iter()
                    .all(|x| x != &Condition::Operational)
                    && (springs[record[0]] != Condition::Damaged)
                {
                    springs = &springs[record[0]..];
                    record = &record[1..];
                    updated = true;
                } else {
                    // Impossible
                    return 0;
                }
            }

            if springs[record[0]] == Condition::Damaged {
                // Impossible, the first element is too long
                if springs[record[0]] == Condition::Damaged {
                    return 0;
                }
                springs = &springs[1..];
                updated = true;
            }

            if springs.last().unwrap() == &Condition::Operational {
                springs = &springs[..springs.len() - 1];
                updated = true;
            }

            if springs.last().unwrap() == &Condition::Damaged {
                if springs[springs.len() - record.last().unwrap()..springs.len() - 1]
                    .iter()
                    .all(|x| x != &Condition::Operational)
                    && (springs[*record.last().unwrap()] != Condition::Damaged)
                {
                    springs = &springs[..springs.len() - record.last().unwrap()];
                    record = &record[..record.len() - 1];
                    updated = true;
                }
            }

            if springs[springs.len() - 1 - record.last().unwrap()] == Condition::Damaged {
                // This is an impossible layout - the last element would be too long
                if springs[springs.len() - 1] == Condition::Damaged {
                    return 0;
                }
                springs = &springs[..springs.len() - 1]
            }
        }

        println!("{springs:?}, {record:?}");

        todo!()
    }
}

impl From<&str> for Gears {
    fn from(value: &str) -> Self {
        let mut split = value.split_ascii_whitespace();
        let springs: Vec<Condition> = split
            .next()
            .unwrap()
            .to_string()
            .chars()
            .map(Condition::from)
            .collect();

        let condition_record: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let record_pattern_len =
            condition_record.iter().sum::<usize>() + condition_record.len() - 1;

        let potential_damaged_springs = springs
            .iter()
            .filter(|x| x != &&Condition::Operational)
            .count();
        Self {
            springs,
            condition_record,
            record_pattern_len,
            potential_damaged_springs,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Condition {
    Unknown,
    Operational,
    Damaged,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 23);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
