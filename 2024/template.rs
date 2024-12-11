const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);
    println!(
        "Part 1: {part_1_answer} in {:.3} ms",
        start.elapsed().as_secs_f32() * 1000.0
    );

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);
    println!(
        "Part 2: {part_2_answer} in {:.3} ms",
        start.elapsed().as_secs_f32() * 1000.0
    );

fn part_1(input: &[u64]) -> u64 {
    todo!();
}

fn part_2(input: &[u64]) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
