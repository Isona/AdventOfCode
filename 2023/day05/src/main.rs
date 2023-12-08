const INPUT: &str = include_str!("input.txt");

fn main() {
    let (seeds, almanac_map) = parse_input(INPUT);

    let part_1_answer = part_1(&seeds, &almanac_map);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&seeds, &almanac_map);
    println!("Part 2: {part_2_answer}");
}

fn part_1(seeds: &[i64], almanac_map: &Vec<AlmanacMap>) -> i64 {
    let mut closest_location = i64::MAX;

    for seed in seeds {
        let mut current_value = *seed;
        for stage in almanac_map {
            current_value = stage.map_value(current_value);
        }

        closest_location = current_value.min(closest_location);
    }

    closest_location
}

fn part_2(seeds: &[i64], almanac_map: &Vec<AlmanacMap>) -> i64 {
    // recalculate seeds vector
    let mut new_seeds: Vec<i64> = Vec::new();

    for seed_range in seeds.chunks(2) {
        let range = seed_range[0]..seed_range[0] + seed_range[1];
        new_seeds.append(&mut range.collect())
    }

    // Then call part 1 on it and return that value
    part_1(&new_seeds, almanac_map)
}

pub struct AlmanacMap(Vec<SeedTransformer>);

impl AlmanacMap {
    fn new() -> AlmanacMap {
        AlmanacMap(Vec::new())
    }

    fn push(&mut self, input: SeedTransformer) {
        self.0.push(input);
    }

    fn map_value(&self, input_value: i64) -> i64 {
        for transformer in &self.0 {
            if transformer.src_contains(input_value) {
                return transformer.transform_value(input_value);
            }
        }

        input_value
    }
}

pub struct SeedTransformer {
    src_start: i64,
    len: i64, // Remember start+len is the first value outside the range
    transformation: i64,
}

impl SeedTransformer {
    fn src_contains(&self, value: i64) -> bool {
        value >= self.src_start && value < self.src_start + self.len
    }

    fn transform_value(&self, value: i64) -> i64 {
        value + self.transformation
    }
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<AlmanacMap>) {
    let mut lines = input.lines();
    // Seeds: 1 2 3 4

    // Parse seeds from the first line

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    let mut stages: Vec<_> = Vec::new();

    // For the rest of the lines

    // [dest range start] [source range start] [range length]
    let mut stage: AlmanacMap = AlmanacMap::new();

    for line in lines.skip(2) {
        if line.is_empty() {
            continue;
        }

        let split_line: Vec<&str> = line.split(' ').collect();
        if split_line.len() == 2 {
            stages.push(stage);
            stage = AlmanacMap::new();
        } else {
            let dest_start = split_line[0].parse::<i64>().unwrap();
            let src_start = split_line[1].parse::<i64>().unwrap();
            let len = split_line[2].parse::<i64>().unwrap();
            let transformation = dest_start - src_start;

            let transformer = SeedTransformer {
                src_start,
                len,
                transformation,
            };
            stage.push(transformer);
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
