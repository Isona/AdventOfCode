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
    // Provide instruction then newline (10)
    // At the end provide "WALK" followed by a newline

    // https://cyberchef.org/#recipe=To_Decimal('Comma',false)&input=Tk9UIEMgSgpOT1QgQSBUCk9SIFQgSgpBTkQgRCBKCldBTEsK
    // NOT C J
    // NOT A T
    // OR T J
    // AND D J
    // WALK
    let commands = VecDeque::from([
        78, 79, 84, 32, 67, 32, 74, 10, 78, 79, 84, 32, 65, 32, 84, 10, 79, 82, 32, 84, 32, 74, 10,
        65, 78, 68, 32, 68, 32, 74, 10, 87, 65, 76, 75, 10,
    ]);

    let (_, output) = pc.run_with_input(commands);
    let output_value = *output.back().unwrap();
    assert!(output_value > 200);
    output_value
}

fn part_2(pc: &mut IntCodePC) -> i128 {
    pc.reset_all();

    // If (E OR H) AND D AND !(A AND B AND C)
    // https://cyberchef.org/#recipe=To_Decimal('Comma',false)&input=T1IgRCBKCkFORCBFIFQKQU5EIEkgVApPUiBIIFQKQU5EIFQgSgpOT1QgQSBUCk9SIFQgSgpOT1QgVCBUCkFORCBCIFQKQU5EIEMgVApOT1QgVCBUCkFORCBUIEoKUlVOCg
    // AND E T
    // AND I T
    // OR H J
    // AND D J
    // NOT A T
    // OR T J
    // NOT T T
    // AND B T
    // AND C T
    // NOT T T
    // AND T J
    // RUN

    let commands = VecDeque::from([
        65, 78, 68, 32, 69, 32, 84, 10, 65, 78, 68, 32, 73, 32, 84, 10, 79, 82, 32, 72, 32, 74, 10,
        65, 78, 68, 32, 68, 32, 74, 10, 78, 79, 84, 32, 65, 32, 84, 10, 79, 82, 32, 84, 32, 74, 10,
        78, 79, 84, 32, 84, 32, 84, 10, 65, 78, 68, 32, 66, 32, 84, 10, 65, 78, 68, 32, 67, 32, 84,
        10, 78, 79, 84, 32, 84, 32, 84, 10, 65, 78, 68, 32, 84, 32, 74, 10, 82, 85, 78, 10,
    ]);

    let (_, output) = pc.run_with_input(commands);
    let output_value = *output.back().unwrap();
    assert!(output_value > 200);
    output_value
}
