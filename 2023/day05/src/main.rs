use std::{collections::HashMap, ops::Range};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (seeds, almanac_map) = parse_input(INPUT);

    let part_1_answer = part_1(&seeds, &almanac_map);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&seeds, &almanac_map);
    println!("Part 2: {part_2_answer}");
}

fn part_1(seeds: &[u64], almanac_map: &[HashMap<Range<u64>, (u64, u64)>]) -> u64 {
    let mut closest_location = u64::MAX;

    for seed in seeds {
        let mut current_value = *seed;
        for stage in almanac_map {
            for (map_range, (dest_range_start, src_range_start)) in stage {
                if map_range.contains(&current_value) {
                    let diff = current_value - src_range_start;
                    current_value = dest_range_start + diff;
                    break;
                }
            }
        }

        closest_location = current_value.min(closest_location);
    }

    closest_location
}

fn part_2(seeds: &[u64], almanac_map: &[HashMap<Range<u64>, (u64, u64)>]) -> u64 {
    // recalculate seeds vector
    let mut new_seeds: Vec<u64> = Vec::new();

    for seed_range in seeds.chunks(2) {
        let range = seed_range[0]..seed_range[0] + seed_range[1];
        new_seeds.append(&mut range.collect())
    }

    // Then call part 1 on it and return that value
    part_1(&new_seeds, almanac_map)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<HashMap<Range<u64>, (u64, u64)>>) {
    let mut lines = input.lines();
    // Seeds: 1 2 3 4

    // Parse seeds from the first line

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();

    let mut stages: Vec<HashMap<Range<u64>, (u64, u64)>> = Vec::new();

    // For the rest of the lines

    // [dest range start] [source range start] [range length]
    let mut stage: HashMap<Range<u64>, (u64, u64)> = HashMap::new();

    for line in lines.skip(2) {
        if line.is_empty() {
            continue;
        }

        let split_line: Vec<&str> = line.split(" ").collect();
        if split_line.len() == 2 {
            stages.push(stage);
            stage = HashMap::new();
        } else {
            let dest_range_start = split_line[0].parse::<u64>().unwrap();
            let src_range_start = split_line[1].parse::<u64>().unwrap();
            let range_length = split_line[2].parse::<u64>().unwrap();

            let range = src_range_start..src_range_start + range_length;
            stage.insert(range, (dest_range_start, src_range_start));
        }
    }

    stages.push(stage);

    (seeds, stages)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (seeds, almanac_map) = parse_input(TESTINPUT);
        assert_eq!(part_1(&seeds, &almanac_map), 35);
    }

    #[test]
    fn part_2_test() {
        let (seeds, almanac_map) = parse_input(TESTINPUT);
        assert_eq!(part_2(&seeds, &almanac_map), 46);
    }
}

// 1389615163 - too high
// 525792406
