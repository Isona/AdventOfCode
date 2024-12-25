use std::collections::{HashMap, HashSet};

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

fn part_1(input: &HashMap<String, HashSet<String>>) -> usize {
    let mut part_1_output = 0;

    for base in input.keys() {
        part_1_output += get_orbiters_count(base, input);
    }

    part_1_output
}

fn get_orbiters_count(base: &str, orbit_list: &HashMap<String, HashSet<String>>) -> usize {
    let Some(orbiters) = orbit_list.get(base) else {
        return 0;
    };
    let mut orbit_count = orbiters.len();

    for orbiter in orbiters {
        orbit_count += get_orbiters_count(&orbiter, orbit_list);
    }

    orbit_count
}

fn part_2(orbit_list: &HashMap<String, HashSet<String>>) -> usize {
    let santa_parents = get_parents("SAN", orbit_list);
    let you_parents = get_parents("YOU", orbit_list);

    let mut output = santa_parents
        .iter()
        .filter(|x| !you_parents.contains(x))
        .count();
    output += you_parents
        .iter()
        .filter(|x| !santa_parents.contains(x))
        .count();

    output
}

fn get_parents(goal: &str, orbit_list: &HashMap<String, HashSet<String>>) -> Vec<String> {
    for (base, orbiters) in orbit_list {
        if orbiters.contains(goal) {
            let mut parents = get_parents(base, orbit_list);
            parents.push(base.clone());
            return parents;
        }
    }

    return Vec::new();
}

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut orbits: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(")");
        let base = split.next().unwrap().to_string();
        let orbiter = split.next().unwrap().to_string();
        if let Some(orbit_list) = orbits.get_mut(&base) {
            orbit_list.insert(orbiter);
        } else {
            orbits.insert(base, HashSet::from([orbiter]));
        }
    }

    orbits
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 42);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_2(&input), 379);
    }
}
