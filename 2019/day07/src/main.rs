use std::collections::VecDeque;

use intcode::{IntCodePC, IntCodeProgramState};
use itertools::Itertools;

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

fn part_1(input: &str) -> i128 {
    let mut pc = IntCodePC::new(input);
    (0..5)
        .permutations(5)
        .map(|signals| get_thruster_signal(&mut pc, signals.into()))
        .max()
        .unwrap()
}

fn get_thruster_signal(pc: &mut IntCodePC, signals: VecDeque<i128>) -> i128 {
    let mut next_input = VecDeque::from([0]);
    for signal in signals {
        pc.reset_all();
        next_input.push_front(signal);
        pc.set_input(next_input);
        pc.run_program();
        next_input = pc.get_output();
    }

    next_input.pop_front().unwrap()
}

fn part_2(input: &str) -> i128 {
    let pc = IntCodePC::new(input);
    (5..10)
        .permutations(5)
        .map(|signals| get_feedback_thruster_signal(&pc, signals.into()))
        .max()
        .unwrap()
}

fn get_feedback_thruster_signal(pc: &IntCodePC, signals: VecDeque<i128>) -> i128 {
    // Set up the thrusters
    let mut thrusters = vec![pc.clone(); 5];

    // Add the signal input to each thruster in turn
    signals
        .iter()
        .enumerate()
        .for_each(|(index, input)| thrusters.get_mut(index).unwrap().set_input([*input].into()));
    // Set the first thruster to have input [signal,0]
    thrusters
        .get_mut(0)
        .unwrap()
        .set_input([signals[0], 0].into());

    // Run each thruster until it
    for thruster in thrusters.iter_mut() {
        assert_eq!(thruster.run_program(), IntCodeProgramState::NeedsInput);
    }

    // While the last thruster hasn't terminated
    while thrusters.last_mut().unwrap().run_program() != IntCodeProgramState::Halted {
        // Loop through each iterator
        for i in 0..thrusters.len() {
            // Get the input from the previous thruster
            let previous_index: usize = (i + 4) % 5;
            let previous_thruster = thrusters.get_mut(previous_index).unwrap();
            let new_input = previous_thruster.take_output();

            // Set the input on the current thruster
            let current_thruster = thrusters.get_mut(i).unwrap();
            current_thruster.set_input(new_input);

            // Run the current thruster
            current_thruster.run_program();
        }
    }

    thrusters
        .last_mut()
        .unwrap()
        .get_output()
        .pop_front()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let output = part_1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(output, 43210);
    }

    #[test]
    fn part_2_test() {
        let output = part_2(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(output, 139629729);

        let output = part_2(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(output, 18216);
    }
}
