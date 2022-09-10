const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &[i32]) -> i32 {
    let mut previous_value = input[0];
    let mut count = 0;

    for value in input {
        if value > &previous_value {
            count = count + 1
        }
        previous_value = *value;
    }

    count
}

fn part_2(input: &[i32]) -> i32 {
    let sliding_windows: Vec<_> = input.windows(3).map(|x| x.iter().sum()).collect();
    part_1(&sliding_windows)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "199
200
208
210
200
207
240
269
260
263";

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
