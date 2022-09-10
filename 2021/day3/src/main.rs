const INPUT: &str = include_str!("input.txt");

fn main() {
    let part_1_answer = part_1(INPUT);
    println!("{part_1_answer}");

    let part_2_answer = part_2(INPUT);
    println!("{part_2_answer}");
}

fn part_1(input: &str) -> u32 {
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    let sample_count = input.lines().count();
    let sample_length = input.lines().next().unwrap().len();

    for i in 0..sample_length {
        let one_count = input
            .lines()
            .filter(|x| x.chars().nth(i) == Some('1'))
            .count();

        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        gamma_rate |= (one_count > sample_count / 2) as u32;
        epsilon_rate |= (one_count <= sample_count / 2) as u32;
    }

    epsilon_rate * gamma_rate
}

fn part_2(input: &str) -> u32 {
    let mut oxygen_rating: u32 = 0;
    let mut co2_rating: u32 = 0;

    let sample_length = input.lines().next().unwrap().len();

    let mut oxygen_input: Vec<_> = input.clone().lines().collect();

    for i in 0..sample_length {
        if oxygen_input.len() == 1 {
            dbg!(oxygen_input[0]);
            oxygen_rating <<= 1;
            oxygen_rating |= (oxygen_input[0].chars().nth(i) == Some('1')) as u32;
            continue;
        }

        let sample_count = oxygen_input.len();
        let one_count = oxygen_input
            .iter()
            .filter(|x| x.chars().nth(i) == Some('1'))
            .count();

        oxygen_rating <<= 1;
        oxygen_rating |= (one_count >= sample_count - one_count) as u32;
        let most_popular = if one_count >= sample_count - one_count {
            '1'
        } else {
            '0'
        };
        oxygen_input.retain(|x| x.chars().nth(i) == Some(most_popular));
    }

    let mut co2_input: Vec<_> = input.clone().lines().collect();

    for i in 0..sample_length {
        if co2_input.len() == 1 {
            co2_rating <<= 1;
            co2_rating |= (co2_input[0].chars().nth(i) == Some('1')) as u32;
            continue;
        }

        let sample_count = co2_input.len();
        let one_count = co2_input
            .iter()
            .filter(|x| x.chars().nth(i) == Some('1'))
            .count();

        co2_rating <<= 1;
        co2_rating |= !(one_count >= sample_count - one_count) as u32;
        let least_popular = if one_count >= sample_count - one_count {
            '0'
        } else {
            '1'
        };
        co2_input.retain(|x| x.chars().nth(i) == Some(least_popular));
    }

    dbg!(oxygen_rating);
    dbg!(co2_rating);

    oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TESTINPUT), 198);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TESTINPUT), 230);
    }
}
