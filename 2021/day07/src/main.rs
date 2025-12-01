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
    let mut min_cost = usize::MAX;
    for i in input {
        let cost = input.iter().map(|other| i.abs_diff(*other)).sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn part_2(input: &[usize]) -> usize {
    let mut min_cost = usize::MAX;

    for i in *input.iter().min().unwrap()..=*input.iter().max().unwrap() {
        let cost = input
            .iter()
            .map(|other| get_triangle_difference(i, *other))
            .sum();

        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn get_triangle_difference(one: usize, two: usize) -> usize {
    let diff = one.abs_diff(two);
    (diff * (diff + 1)) / 2
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
        assert_eq!(part_1(&input), 37);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 168);
    }
}
