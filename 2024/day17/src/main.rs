const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = input.part_1();

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer}");
    println!("Part 1 took {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_2(input: &Computer) -> u64 {
    todo!();
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
        initial_a: a,
        a,
        b,
        c,
        program_counter: 0,
        instructions,
    }
}

#[derive(Clone, Debug)]
struct Computer {
    initial_a: i64,
    a: i64,
    b: i64,
    c: i64,
    program_counter: usize,
    instructions: Vec<i64>,
}

impl Computer {
    fn run_program(&mut self) -> Vec<i64> {
        // Reset self:
        self.reset();

        // Run the program
        let mut output = Vec::new();

        while self.program_counter < self.instructions.len() {
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
                        continue;
                    }
                }
                // bxc
                4 => self.b ^= self.c,
                // out
                5 => output.push(combo_operand % 8),
                // bdv
                6 => self.b = self.a / 2i64.pow(combo_operand.try_into().unwrap()),
                // cdv
                7 => self.c = self.a / 2i64.pow(combo_operand.try_into().unwrap()),
                _ => panic!(),
            }

            self.program_counter += 2;
        }

        dbg!(&output);
        output
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

    fn reset(&mut self) {
        self.a = self.initial_a;
        self.b = 0;
        self.c = 0;
    }

    fn part_1(&mut self) -> String {
        self.run_program()
            .iter()
            .map(i64::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    fn part_2(&mut self) -> String {
        todo!();
    }
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
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
