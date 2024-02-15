const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for series in input {
        let next_val = calculate_next(series);
        sum += next_val;
    }

    sum
}

fn part_2(input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for series in input {
        let next_val = calculate_last(series);
        sum += next_val;
    }

    sum
}

fn calculate_next(series: &Vec<i64>) -> i64 {
    let mut differences = vec![];
    let mut all_zero = true;
    for pair in series.windows(2) {
        let difference = pair[1] - pair[0];

        if difference != 0 {
            all_zero = false;
        }

        differences.push(difference);
    }

    if all_zero {
        *series.last().unwrap()
    } else {
        let difference = calculate_next(&differences);
        let last_in_series = series.last().unwrap();
        last_in_series + difference
    }
}

fn calculate_last(series: &Vec<i64>) -> i64 {
    let mut differences = vec![];
    let mut all_zero = true;
    for pair in series.windows(2) {
        let difference = pair[1] - pair[0];

        if difference != 0 {
            all_zero = false;
        }

        differences.push(difference);
    }

    if all_zero {
        *series.first().unwrap()
    } else {
        let difference = calculate_last(&differences);
        let last_in_series = series.first().unwrap();
        last_in_series - difference
    }
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|x| x.split(' ').map(|y| y.parse::<i64>().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 114);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 2);
    }
}
