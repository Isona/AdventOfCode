const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &[CalibrationEquation]) -> u64 {
    // Get all the binary permutations of values from 0 to 2048 (2^11)
    // The input never has any lists of components longer than 12
    // So for each we only need 11 operators (fence posts)
    // We reverse the binary strings so for shorter lists we get the useful values
    let operator_permutations: Vec<String> = (0..2048)
        .map(|x| format!("{x:011b}").chars().rev().collect())
        .collect();

    let mut output_total = 0;
    for equation in input {
        // We want to loop through 2^(n-1) of the operator sets
        // Based on the length of the component list
        let operators_needed: usize =
            2usize.pow((equation.components.len() - 1).try_into().unwrap());

        // Compiler optimisations yay
        assert!(operator_permutations.len() >= operators_needed);

        // Loop through each operator set of the ones we needed
        for operator_set in 0..operators_needed {
            let mut components = equation.components.iter();
            let mut total = *components.next().unwrap();
            let mut operators = operator_permutations.get(operator_set).unwrap().chars();

            // Treat 0 as +
            // Treat 1 as *
            for component in components {
                match operators.next().unwrap() {
                    '0' => total += component,
                    '1' => total *= component,
                    _ => {}
                }
            }
            if total == equation.test_value {
                println!("{equation:#?}");
                output_total += equation.test_value;
                break;
            }
        }
    }

    output_total
}

fn part_2(input: &[CalibrationEquation]) -> u64 {
    let mut output_total = 0;
    for equation in input {
        if equation.calculate_part2(0, 0) {
            output_total += equation.test_value;
        }
    }

    output_total
}

fn parse_input(input: &str) -> Vec<CalibrationEquation> {
    input.lines().map(CalibrationEquation::new).collect()
}

#[derive(Debug)]
struct CalibrationEquation {
    test_value: u64,
    components: Vec<u64>,
}

impl CalibrationEquation {
    fn new(input: &str) -> Self {
        let (test_value, components) = input.split_once(':').unwrap();

        CalibrationEquation {
            test_value: test_value.parse::<u64>().unwrap(),
            components: components
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect(),
        }
    }

    fn calculate_part2(&self, current_sum: u64, index: usize) -> bool {
        // Deal with the starting point
        if index == 0 {
            self.calculate_part2(*self.components.get(index).unwrap(), index + 1)
        }
        // Deal with end case
        else if index >= self.components.len() {
            current_sum == self.test_value
        }
        // Terminate early if we're over
        else if current_sum > self.test_value {
            false
        }
        // Try the 3 different operators
        else {
            let current_value = self.components.get(index).unwrap();
            // Add
            if self.calculate_part2(current_sum + current_value, index + 1) {
                return true;
            }

            // Mul
            if self.calculate_part2(current_sum * current_value, index + 1) {
                return true;
            }

            // Concat
            let concatenated = format!("{current_sum}{current_value}")
                .parse::<u64>()
                .unwrap();

            return self.calculate_part2(concatenated, index + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn input_parse_test() {
        let calibration_equation = CalibrationEquation::new("3267: 81 40 27");

        assert_eq!(calibration_equation.test_value, 3267);
        assert_eq!(calibration_equation.components, vec![81, 40, 27]);
    }

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 3749);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 11387);
    }
}
