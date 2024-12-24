use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Shl,
};

use regex::Regex;
const INPUT: &str = include_str!("input.txt");

fn main() {
    let (start_wires, calculations) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&start_wires, &calculations);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&start_wires, &calculations);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(start_wires: &HashMap<String, u8>, calculations: &Vec<Calculation>) -> u64 {
    let mut current_wires = start_wires.clone();
    let mut remaining_calculations: VecDeque<&Calculation> = calculations.iter().collect();
    while let Some(calc) = remaining_calculations.pop_front() {
        if let Some((result_wire, result)) = calc.calc_result(&current_wires) {
            current_wires.insert(result_wire, result);
        } else {
            remaining_calculations.push_back(calc);
        }
    }

    let mut z_wires: Vec<_> = current_wires
        .iter()
        .filter(|(wire, _)| wire.starts_with("z"))
        .collect();

    z_wires.sort();
    println!("{z_wires:?}");
    let z_wires = z_wires
        .iter()
        .rev()
        .map(|x| x.1)
        .fold(0, |acc, x| acc.shl(1) + *x as u64);
    println!("{z_wires:?}");
    z_wires
}

fn part_2(start_wires: &HashMap<String, u8>, calculations: &Vec<Calculation>) -> u64 {
    let x_y = calculations
        .iter()
        .filter(|x| x.op1.starts_with("x") || x.op2.starts_with("x"));

    // Get the (xn XOR yn) and (xn AND yn) wire names
    let mut x_and_y = vec![String::default(); start_wires.len() / 2];
    let mut x_xor_y = vec![String::default(); start_wires.len() / 2];
    for i in x_y {
        let index: usize = i.op1[1..].parse().unwrap();
        match i.operation {
            BooleanOp::And => x_and_y[index] = i.out.clone(),
            BooleanOp::Xor => x_xor_y[index] = i.out.clone(),
            BooleanOp::Or => panic!(),
        }
    }

    // let mut carry = vec![String::default(); start_wires.len() / 2];
    // carry[0] = x_and_y[0].clone();

    // for i in 1..45() {

    // }

    // We know that a Z calculation is wrong if it doesn't come from an XOR
    let z_errors: Vec<String> = calculations
        .iter()
        .filter(|calc| calc.out.starts_with("z") && calc.operation != BooleanOp::Xor)
        .map(|x| x.out.clone())
        .collect();

    println!("{z_errors:?}");

    let and_outputs: HashSet<String> = calculations
        .iter()
        .filter(|x| x.operation == BooleanOp::And)
        .map(|x| x.out.clone())
        .collect();

    let mut and_output_errors = Vec::new();
    for and_output in and_outputs {
        if calculations
            .iter()
            .any(|x| x.has_input(&and_output) && x.operation != BooleanOp::Or)
        {
            if and_output != x_and_y[0] {
                and_output_errors.push(and_output);
            }
        }
    }
    println!("{and_output_errors:?}");

    let mut or_output_errors = Vec::new();
    let or_outputs: HashSet<String> = calculations
        .iter()
        .filter(|x| x.operation == BooleanOp::Or)
        .map(|x| x.out.clone())
        .collect();
    'outer: for or_output in or_outputs {
        let calculations_using_output: Vec<&Calculation> = calculations
            .iter()
            .filter(|x| x.has_input(&or_output))
            .collect();

        if calculations_using_output.len() != 2 {
            or_output_errors.push(or_output);
            continue;
        }

        let mut found_xor = false;
        let mut found_and = false;

        for calc in calculations_using_output {
            match calc.operation {
                BooleanOp::And => found_and = true,
                BooleanOp::Xor => found_xor = true,
                BooleanOp::Or => {
                    or_output_errors.push(or_output.clone());
                    continue 'outer;
                }
            }
        }
        if !(found_and && found_xor) {
            or_output_errors.push(or_output.clone());
        }
    }

    println!("{or_output_errors:?}");

    let mut xor_output_errors = Vec::new();
    let xor_outputs: HashSet<String> = calculations
        .iter()
        .filter(|x| x.operation == BooleanOp::Xor)
        .map(|x| x.out.clone())
        .collect();
    'outer: for xor_output in xor_outputs {
        let calculations_using_output: Vec<&Calculation> = calculations
            .iter()
            .filter(|x| x.has_input(&xor_output))
            .collect();

        if calculations_using_output.len() != 2 {
            if !xor_output.starts_with("z") {
                xor_output_errors.push(xor_output);
            }
            continue;
        }

        let mut found_xor = false;
        let mut found_and = false;

        for calc in calculations_using_output {
            match calc.operation {
                BooleanOp::And => found_and = true,
                BooleanOp::Xor => found_xor = true,
                BooleanOp::Or => {
                    xor_output_errors.push(xor_output.clone());
                    continue 'outer;
                }
            }
        }
        if !(found_and && found_xor) {
            xor_output_errors.push(xor_output.clone());
        }
    }
    println!("XOR Errors: {xor_output_errors:?}");

    todo!();
}

fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Calculation>) {
    let mut wires = HashMap::new();
    let mut calculations = Vec::new();
    let mut state = ParseState::Wires;

    // frj XOR qhw -> z04
    let regex_pattern = r"(?<op1>.{3}) (?<operation>.{2,3}) (?<op2>.{3}) -> (?<output>.{3})";
    let regex = Regex::new(&regex_pattern).unwrap();
    for line in input.lines() {
        match state {
            ParseState::Wires => {
                if line.is_empty() {
                    state = ParseState::Operations;
                    continue;
                }
                let mut split = line.split(": ");
                wires.insert(
                    split.next().unwrap().to_string(),
                    split.next().unwrap().parse().unwrap(),
                );
            }
            ParseState::Operations => {
                let caps = regex.captures(line).unwrap();
                let calc = Calculation {
                    op1: caps["op1"].to_string(),
                    op2: caps["op2"].to_string(),
                    operation: BooleanOp::from(&caps["operation"]),
                    out: caps["output"].to_string(),
                };
                calculations.push(calc);
            }
        }
    }

    (wires, calculations)
}

enum ParseState {
    Wires,
    Operations,
}

#[derive(Debug, Clone)]
struct Calculation {
    op1: String,
    op2: String,
    operation: BooleanOp,
    out: String,
}

impl Calculation {
    fn calc_result(&self, available_wires: &HashMap<String, u8>) -> Option<(String, u8)> {
        if let Some(op1) = available_wires.get(&self.op1) {
            if let Some(op2) = available_wires.get(&self.op2) {
                return Some((self.out.clone(), self.operation.do_op(op1, op2)));
            }
        }
        None
    }

    fn has_input(&self, wire: &str) -> bool {
        wire == self.op1 || wire == self.op2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BooleanOp {
    And,
    Or,
    Xor,
}

impl BooleanOp {
    fn do_op(&self, op1: &u8, op2: &u8) -> u8 {
        match self {
            BooleanOp::And => op1 & op2,
            BooleanOp::Or => op1 | op2,
            BooleanOp::Xor => op1 ^ op2,
        }
    }
}

impl From<&str> for BooleanOp {
    fn from(value: &str) -> Self {
        match value {
            "OR" => Self::Or,
            "AND" => Self::And,
            "XOR" => Self::Xor,
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
        let (start_wires, calculations) = parse_input(TESTINPUT);
        assert_eq!(part_1(&start_wires, &calculations), 2024);
    }

    #[test]
    fn part_2_test() {
        let (start_wires, calculations) = parse_input(TESTINPUT);
        assert_eq!(part_2(&start_wires, &calculations), 5);
    }
}
