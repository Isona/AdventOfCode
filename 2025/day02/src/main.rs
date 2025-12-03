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

fn part_1(input: &[IDRange]) -> u64 {
    input.iter().map(|x| x.invalid_sum()).sum()
}

fn part_2(input: &[IDRange]) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Vec<IDRange> {
    let mut id_ranges = vec![];
    let ranges_split = input.split(',');
    for range in ranges_split {
        let mut current_split = range.split('-');
        let start = current_split.next().unwrap().parse().unwrap();
        let end = current_split.next().unwrap().parse().unwrap();
        id_ranges.push(IDRange { start, end });
    }

    id_ranges
}

#[derive(Debug, Clone, Copy)]
struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    fn invalid_sum(&self) -> u64 {
        let mut invalid_sum = 0;
        let range = self.start..=self.end;

        let start_invalid_index = get_invalid_index(self.start);
        let end_invalid_index = get_invalid_index(self.end);

        for invalid_index in start_invalid_index..=end_invalid_index {
            let invalid = get_invalid_by_index(invalid_index);
            if range.contains(&invalid) {
                invalid_sum += invalid;
            }
        }
        invalid_sum
    }

    fn invalid_sum_part_2(&self) -> u64 {
        let mut invalid_sum = 0;

        // If the start and end ranges have the same digit count
        if self.start.ilog10() == self.end.ilog10() {
        }
        // If they're different
        else {
        }
        invalid_sum
    }
}

fn get_invalid_by_index(input: u64) -> u64 {
    input * 10_u64.pow(input.ilog10() + 1) + input
}

fn get_invalid_index(input: u64) -> u64 {
    let input_log10 = input.ilog10();
    if input_log10.is_multiple_of(2) {
        10_u64.pow(input_log10 / 2)
    } else {
        input / 10_u64.pow(input_log10.div_ceil(2))
    }
}

fn get_factors(input: u64) -> impl Iterator<Item = u64> {
    (1u64..=input / 2).filter(move |i| input.is_multiple_of(*i))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        dbg!(&input);
        assert_eq!(part_1(&input), 1227775554);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 4174379265);
    }

    #[test]
    fn test_factors() {
        assert_eq!(get_factors(10).collect::<Vec<u64>>(), vec![1, 2, 5]);
        assert_eq!(get_factors(7).collect::<Vec<u64>>(), vec![1]);
        assert_eq!(get_factors(9).collect::<Vec<u64>>(), vec![1, 3]);
        assert_eq!(
            get_factors(24).collect::<Vec<u64>>(),
            vec![1, 2, 3, 4, 6, 8, 12]
        );
    }
}
