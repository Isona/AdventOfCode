use intcode::{IntCodePC, IntCodeProgramState};
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);
    pc.set(1, 12);
    pc.set(2, 2);

    let start = std::time::Instant::now();
    assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
    let part_1_answer = pc.get_data(0).unwrap();

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_2(pc: &mut IntCodePC) -> i128 {
    // Calculate it faster by working out what difference x and y have on the output

    // Find output when x and y are both 0
    // This is the base value
    pc.reset_all();
    pc.set(1, 0);
    pc.set(2, 0);
    pc.run_program();
    let base_value = pc.get_data(0).unwrap();

    // Find output when x = 1, y = 0
    pc.reset_all();
    pc.set(1, 1);
    pc.set(2, 0);
    pc.run_program();
    let x1y0 = pc.get_data(0).unwrap();

    // Base_value has x * x_vector added to it
    let x_vector = x1y0 - base_value;

    pc.reset_all();
    pc.set(1, 0);
    pc.set(2, 1);
    pc.run_program();
    let x0y1 = pc.get_data(0).unwrap();
    // Base_value has y*y_vector added to it
    let y_vector = x0y1 - base_value;

    // Subtract the base value from our goal output
    let goal = 19690720 - base_value;
    let x = goal / x_vector;
    let y = (goal % x_vector) / y_vector;
    x * 100 + y
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let mut pc = IntCodePC::new(TESTINPUT);
        assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
        assert_eq!(pc.get_data(0), Some(3500));
    }
}
