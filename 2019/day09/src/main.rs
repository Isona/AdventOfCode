use intcode::{IntCodePC, IntCodeProgramState};
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);
    pc.set_input([1].into());

    let start = std::time::Instant::now();
    assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
    let part_1_answer = pc.get_output()[0];

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    pc.reset_all();
    pc.set_input([2].into());
    let start = std::time::Instant::now();
    assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
    let part_2_answer = pc.get_output()[0];

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}
