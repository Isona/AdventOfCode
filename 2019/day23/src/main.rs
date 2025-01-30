use std::collections::VecDeque;

use intcode::IntCodePC;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> i128 {
    // Set up the network
    let mut network = Vec::new();
    for i in 0..50 {
        let mut network_pc = pc.clone();
        network_pc.set_input([i].into());
        network_pc.run_program();
        network.push(network_pc);
    }

    // Make vecs holding the input
    let mut inputs: Vec<VecDeque<i128>> = Vec::from([const { VecDeque::new() }; 50]);
    let mut next_inputs: Vec<VecDeque<i128>> = Vec::from([const { VecDeque::new() }; 50]);
    loop {
        // Loop over the PCs, running them to get their output
        for (index, pc) in network.iter_mut().enumerate() {
            // Set the input correctly
            if inputs[index].is_empty() {
                pc.set_input([-1].into());
            } else {
                pc.set_input(std::mem::take(&mut inputs[index]));
            }

            pc.run_program();

            // Set the output in next_inputs so that we can use it in the next cycle
            let mut output = pc.take_output();
            while !output.is_empty() {
                let destination = output.pop_front().unwrap();
                // If the destination is 255 then this is our output
                if destination == 255 {
                    return output[1];
                } else {
                    // Add to the destinations list of inputs
                    next_inputs[destination as usize].append(
                        &mut [output.pop_front().unwrap(), output.pop_front().unwrap()].into(),
                    );
                }
            }
        }
        // Swap inputs and next inputs to avoid reallocating
        std::mem::swap(&mut inputs, &mut next_inputs);
    }
}

fn part_2(pc: &mut IntCodePC) -> i128 {
    // Set up the network
    let mut network = Vec::new();
    for i in 0..50 {
        let mut network_pc = pc.clone();
        network_pc.set_input([i].into());
        network_pc.run_program();
        network.push(network_pc);
    }

    let mut inputs: Vec<VecDeque<i128>> = Vec::from([const { VecDeque::new() }; 50]);
    let mut next_inputs: Vec<VecDeque<i128>> = Vec::from([const { VecDeque::new() }; 50]);

    // Set up nat_input
    let mut nat_input = VecDeque::from([0, 0]);
    let mut last_y_value = i128::MAX;

    loop {
        // Use these to check if the network is quiet
        let mut all_input_empty = true;
        let mut none_sent = true;

        // Loop over the PCs, running them to get their output
        for (index, pc) in network.iter_mut().enumerate() {
            // Set the input correctly
            if inputs[index].is_empty() {
                pc.set_input([-1].into());
            } else {
                all_input_empty = false;
                pc.set_input(std::mem::take(&mut inputs[index]));
            }

            pc.run_program();

            let mut output = pc.take_output();
            if !output.is_empty() {
                none_sent = false;
            }

            // Set the output in next_inputs so that we can use it in the next cycle
            while !output.is_empty() {
                let destination = output.pop_front().unwrap();
                if destination == 255 {
                    nat_input[0] = output.pop_front().unwrap();
                    nat_input[1] = output.pop_front().unwrap();
                } else {
                    next_inputs[destination as usize].append(
                        &mut [output.pop_front().unwrap(), output.pop_front().unwrap()].into(),
                    );
                }
            }
        }

        // If there were no inputs and no outputs
        // Check if the y value is the same as last time
        // Then set the input for pc[0] to the nat values
        if all_input_empty & none_sent {
            if last_y_value == nat_input[1] {
                return last_y_value;
            } else {
                last_y_value = nat_input[1];
            }
            next_inputs[0].append(&mut nat_input);
            nat_input.push_back(0);
            nat_input.push_back(0);
        }

        // Swap inputs and next inputs to avoid reallocating
        std::mem::swap(&mut inputs, &mut next_inputs);
    }
}
