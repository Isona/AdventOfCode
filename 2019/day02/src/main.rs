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
    for x in 0..100 {
        for y in 0..100 {
            pc.reset_all();
            pc.set(1, x);
            pc.set(2, y);
            assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
            if pc.get_data(0) == Some(19690720) {
                return x * 100 + y;
            }
        }
    }

    panic!()
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
