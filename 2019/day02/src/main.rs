use intcode::IntCodePC;
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);
    pc.set(1, 12);
    pc.set(2, 2);

    let start = std::time::Instant::now();
    let part_1_answer = pc.run_program();

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let pc = IntCodePC::new(INPUT);
    let part_2_answer = part_2(&pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_2(initial_pc: &IntCodePC) -> i128 {
    for x in 0..100 {
        for y in 0..100 {
            let mut pc = initial_pc.clone();
            pc.set(1, x);
            pc.set(2, y);
            if pc.run_program() == 19690720 {
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
        assert_eq!(pc.run_program(), 3500);
    }
}
