use itertools::{Itertools, chain, repeat_n};
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(input.clone(), 100);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: Vec<i32>, phases: usize) -> String {
    let mut output = input;
    for _ in 0..phases {
        output = do_phase(&output);
    }

    //println!("{output:?}");
    output[0..8].iter().join("")
}

fn do_phase(input: &[i32]) -> Vec<i32> {
    let mut output = vec![0; input.len()];
    output
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, output_location)| {
            let iteration = index + 1;
            let first_zeros = repeat_n(0, iteration);
            let ones = repeat_n(1, iteration);
            let second_zeros = repeat_n(0, iteration);
            let negative_ones = repeat_n(-1, iteration);
            let pattern = chain!(first_zeros, ones, second_zeros, negative_ones)
                .cycle()
                .skip(1);

            *output_location = input
                .iter()
                .zip(pattern)
                .map(|(input, pattern)| input * pattern)
                .sum::<i32>()
                .abs()
                % 10;
        });

    output
}

fn part_2(input: &[i32]) -> String {
    let input = input.iter().map(|x| *x as u64).collect_vec();
    let split_point: usize = input[0..7]
        .iter()
        .fold(0, |acc, item| acc * 10 + item)
        .try_into()
        .unwrap();
    println!("{split_point}");
    assert!(split_point > input.len() / 2);
    let total_signal_length = input.len() * 10000;
    let signal_repetitions: usize = (total_signal_length - split_point).div_ceil(input.len());
    let skip_len = split_point % input.len();
    let mut input = input.repeat(signal_repetitions).split_off(skip_len);
    println!("{:?}", &input.len());
    input.reverse();

    for _ in 0..100 {
        for index in 1..input.len() {
            input[index] = (input[index] + input[index - 1]) % 10;
        }
    }

    (0..8)
        .map(|_| input.pop().map(|x| x % 10).unwrap())
        .join("")
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = parse_input("12345678");
        assert_eq!(part_1(input.clone(), 1), "48226158".to_string());
        assert_eq!(part_1(input, 4), "01029498".to_string());

        let input = parse_input("80871224585914546619083218645595");
        assert_eq!(part_1(input, 100), "24176176");
    }

    #[test]
    fn part_2_test() {
        let input = parse_input("03036732577212944063491565474664");
        assert_eq!(part_1(input.repeat(10000), 100), "84462026");
    }
}
