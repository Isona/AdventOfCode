const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

#[allow(clippy::cast_possible_truncation)]
fn part_1(input: &[String]) -> (String, String) {
    let mut output_1 = String::new();
    let mut output_2 = String::new();
    for i in 0..input[0].len() {
        let mut occurences = [0; 26];
        for value in input.iter().map(|x| x.chars().nth(i).unwrap()) {
            occurences[value as usize - 97] += 1usize;
        }

        let most_common =
            char::from(occurences.iter().enumerate().max_by_key(|x| x.1).unwrap().0 as u8 + 97);
        output_1.push(most_common);

        let least_common = char::from(
            occurences
                .iter()
                .enumerate()
                .filter(|x| x.1 != &0)
                .min_by_key(|x| x.1)
                .unwrap()
                .0 as u8
                + 97,
        );
        output_2.push(least_common);
    }

    (output_1, output_2)
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).0, "easter");
        assert_eq!(part_1(&input).1, "advent");
    }
}
