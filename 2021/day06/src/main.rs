use std::mem;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &[usize]) -> usize {
    simulate(input, 80)
}

fn part_2(input: &[usize]) -> usize {
    simulate(input, 256)
}

fn simulate(input: &[usize], days: usize) -> usize {
    let mut current_state = vec![0; 9];
    let mut new_state = vec![0; 9];
    for i in input {
        current_state[*i] += 1;
    }

    for _ in 0..days {
        new_state[..8].copy_from_slice(&current_state[1..9]);
        new_state[8] = current_state[0];
        new_state[6] += current_state[0];

        mem::swap(&mut current_state, &mut new_state);
    }

    current_state.iter().sum()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 5934);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 26984457539);
    }
}
