const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = input.part_1();

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer}");
    println!("Part 1 took {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = calc_part_2();

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn parse_input(input: &str) -> Computer {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let c = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    lines.next().unwrap();

    let instructions = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        program_counter: 0,
        instructions,
    }
}

#[derive(Clone, Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    program_counter: usize,
    instructions: Vec<i64>,
}

impl Computer {
    fn run_program(&mut self) -> Vec<i64> {
        // Run the program
        let mut output = Vec::new();

        while self.program_counter < self.instructions.len() {
            if let Some(output_value) = self.run_instruction() {
                output.push(output_value);
            }
        }

        output
    }

    fn run_instruction(&mut self) -> Option<i64> {
        let literal_operand = self.instructions[self.program_counter + 1];
        let combo_operand = self.get_operand_value(literal_operand.try_into().unwrap());
        match self.instructions[self.program_counter] {
            // adv
            0 => self.a /= 2i64.pow(combo_operand.try_into().unwrap()),
            // bxl
            1 => self.b ^= literal_operand,
            // bst
            2 => self.b = combo_operand % 8,
            // jnz
            3 => {
                if self.a != 0 {
                    self.program_counter = literal_operand.try_into().unwrap();
                    return None;
                }
            }
            // bxc
            4 => self.b ^= self.c,
            // out
            5 => {
                self.program_counter += 2;
                return Some(combo_operand % 8);
            }
            // bdv
            6 => self.b = self.a / 2i64.pow(combo_operand.try_into().unwrap()),
            // cdv
            7 => self.c = self.a / 2i64.pow(combo_operand.try_into().unwrap()),
            _ => panic!(),
        }

        self.program_counter += 2;
        None
    }

    fn get_operand_value(&self, literal_operand: usize) -> i64 {
        match literal_operand {
            // adv
            0 => 0,
            // bxl
            1 => 1,
            // bst
            2 => 2,
            // jnz
            3 => 3,
            // bxc
            4 => self.a,
            // out
            5 => self.b,
            // bdv
            6 => self.c,
            // cdv
            7 => panic!(),
            _ => panic!(),
        }
    }

    fn part_1(&mut self) -> String {
        self.run_program()
            .iter()
            .map(i64::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }
}

// Hardcoded input function
fn my_input(initial_a: u64) -> Vec<u64> {
    let mut output = Vec::new();
    let mut a: u64 = initial_a;
    let mut b;
    let mut c;

    // 3, 0 at end
    while a != 0 {
        // 2, 4
        b = a % 8;
        // 1, 5
        b ^= 5;
        // 7, 5
        c = a / 2u64.pow(b.try_into().unwrap());
        // 1, 6
        b ^= 6;
        // 0, 3
        a /= 2u64.pow(3); // Remove the last 3 bits from A
        // 4, 0
        b ^= c;
        // 5, 5
        output.push(b % 8);
        //3, 0 : jump
    }

    output
}

fn calc_part_2() -> u64 {
    let expected_out = Vec::from([2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 0, 5, 5, 3, 0]);
    do_calc(3, &expected_out).unwrap()
}

fn do_calc(a: u64, expected_out: &Vec<u64>) -> Option<u64> {
    for last_bytes in 0..8 {
        let test_output = my_input(a * 8 + last_bytes);
        if test_output == expected_out[expected_out.len() - test_output.len()..expected_out.len()] {
            if test_output.len() == expected_out.len() {
                return Some(a * 8 + last_bytes);
            } else if let Some(real_a) = do_calc(a * 8 + last_bytes, expected_out) {
                return Some(real_a);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn parse_test() {
        let computer = parse_input(TESTINPUT);
        assert_eq!(computer.a, 729);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.instructions.len(), 6);
        assert_eq!(computer.instructions[2], 5);
    }
    #[test]
    fn part_1_test() {
        let mut input = parse_input(TESTINPUT);
        let output = input.part_1();
        let test_output = "4,6,3,5,6,3,5,2,1,0";
        assert_eq!(output, test_output);
    }

    #[test]
    fn test_hardcoded_input_function() {
        let expected_out = Vec::from([7, 3, 1, 3, 6, 3, 6, 0, 2]);
        let initial_a = 24847151;
        let output = my_input(initial_a);
        assert_eq!(expected_out, output);
    }

    #[test]
    fn test_part_2() {
        let expected_out = Vec::from([2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 0, 5, 5, 3, 0]);
        let initial_a = calc_part_2();
        let output = my_input(initial_a);
        assert_eq!(output, expected_out);
    }
}
