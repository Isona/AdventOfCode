use std::ops::RangeInclusive;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut ranges, ingredients) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&ranges, &ingredients);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut ranges);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(ranges: &[RangeInclusive<u64>], ingredients: &[u64]) -> u64 {
    let mut fresh_count = 0;
    for ingredient in ingredients {
        if ranges.iter().any(|x| x.contains(ingredient)) {
            fresh_count += 1;
        }
    }
    fresh_count
}

fn part_2(ranges: &mut [RangeInclusive<u64>]) -> u64 {
    ranges.sort_by_key(|x| *x.start());
    let mut fresh_ingredients = 0;

    let mut current_range = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if current_range.contains(range.start()) && !current_range.contains(range.end()) {
            current_range = *current_range.start()..=*range.end();
        } else if !current_range.contains(range.start()) {
            fresh_ingredients += get_range_len(&current_range);
            current_range = range.clone();
        }
    }

    fresh_ingredients += get_range_len(&current_range);
    fresh_ingredients
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut parsing_ranges = true;
    for line in input.lines() {
        if parsing_ranges {
            if line.is_empty() {
                parsing_ranges = false;
            } else {
                let mut range_split = line.split('-');
                let start = range_split.next().unwrap().parse().unwrap();
                let end = range_split.next().unwrap().parse().unwrap();
                ranges.push(start..=end);
            }
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    (ranges, ingredients)
}

fn get_range_len(range: &RangeInclusive<u64>) -> u64 {
    range.end() - range.start() + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (ranges, ingredients) = parse_input(TESTINPUT);
        assert_eq!(part_1(&ranges, &ingredients), 3);
    }

    #[test]
    fn part_2_test() {
        let (mut ranges, _) = parse_input(TESTINPUT);
        assert_eq!(part_2(&mut ranges), 14);
    }
}
