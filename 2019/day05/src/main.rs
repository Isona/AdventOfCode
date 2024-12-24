use std::collections::VecDeque;

use intcode::IntCodePC;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);
    pc.set_input(VecDeque::from([1]));
    let start = std::time::Instant::now();
    pc.run_program();
    let part_1_answer = pc.get_output().pop_back().unwrap();

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    //let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    //println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_2(input: &[u64]) -> u64 {
    todo!();
}
