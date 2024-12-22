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

fn part_1(input: &[u64]) -> u64 {
    let mut fuel_requirement = 0;
    for value in input {
        fuel_requirement += get_required_mass(*value);
    }

    fuel_requirement
}

#[inline]
fn get_required_mass(input: u64) -> u64 {
    (input / 3).saturating_sub(2)
}

fn part_2(input: &[u64]) -> u64 {
    let mut output = 0;
    for value in input {
        output += get_total_fuel(*value);
    }

    output
}

fn get_total_fuel(input: u64) -> u64 {
    let mut total_fuel_requirement = 0;

    let mut fuel_requirement = get_required_mass(input);
    total_fuel_requirement += fuel_requirement;

    while fuel_requirement != 0 {
        fuel_requirement = get_required_mass(fuel_requirement);
        total_fuel_requirement += fuel_requirement;
    }

    total_fuel_requirement
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(get_required_mass(100756), 33583);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(&[100756]), 50346);
    }
}
