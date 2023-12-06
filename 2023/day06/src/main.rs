const INPUT: &str = include_str!("input.txt");

fn main() {
    let (times, records) = parse_input(INPUT);

    let part_1_answer = part_1(&times, &records);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&times, &records);
    println!("Part 2: {part_2_answer}");
}

fn part_1(times: &[u64], records: &[u64]) -> u64 {
    let mut total = 1;

    for (time, record) in times.iter().zip(records) {
        let mut winning_ways = 0;

        // Start at the midpoint to make the loop as short as possible
        for x in (0..=(time / 2)).rev() {
            let y = time - x;
            if x * y > *record {
                winning_ways += 2;
            // The numbers decrease from the midpoint so break
            } else {
                break;
            }
        }

        // if the time is even don't double count time/2 * time/2
        if time % 2 == 0 && (time / 2).pow(2) > *record {
            winning_ways -= 1
        }

        total *= winning_ways;
    }

    total
}

fn part_2(times: &[u64], records: &[u64]) -> u64 {
    // Concatenate all the times together
    let new_time = times
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    // Concatenate all the records together
    let new_record = records
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    part_1(&[new_time], &[new_record])
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();

    let records = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();

    (times, records)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (times, records) = parse_input(TESTINPUT);
        assert_eq!(part_1(&times, &records), 288);
    }

    #[test]
    fn part_2_test() {
        let (times, records) = parse_input(TESTINPUT);
        assert_eq!(part_2(&times, &records), 71503);
    }
}
