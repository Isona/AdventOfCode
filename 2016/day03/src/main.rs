#![feature(iter_array_chunks)]
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

fn part_1(input: &[Vec<u64>]) -> u64 {
    let mut count = 0;
    for triangle in input {
        let max = triangle.iter().max().unwrap();
        if triangle.iter().sum::<u64>() - max > *max {
            count += 1;
        }
    }
    count
}

fn part_2(input: &[Vec<u64>]) -> u64 {
    let mut count = 0;
    for index in 0..3 {
        for [a, b, c] in input.iter().map(|x| x[index]).array_chunks() {
            let max = a.max(b).max(c);
            if a + b + c - max > max {
                count += 1;
            }
        }
    }

    count
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}
