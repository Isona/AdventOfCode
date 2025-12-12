use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1(&input, 1000);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &[JunctionBox], connect_count: usize) -> (u64, u64) {
    let mut distance_map = BTreeMap::new();

    for (first, second) in input.iter().tuple_combinations() {
        distance_map.insert(first.get_distance(second), (*first, *second));
    }

    let mut circuits: Vec<BTreeSet<JunctionBox>> = Vec::new();

    for (_, (first, second)) in distance_map.iter().take(connect_count) {
        let mut circuit_one = usize::MAX;
        let mut circuit_two = usize::MAX;

        for (current_circuit_num, circuit) in circuits.iter().enumerate() {
            if circuit_one == usize::MAX && circuit.contains(first) {
                circuit_one = current_circuit_num;
            } else if circuit_two == usize::MAX && circuit.contains(second) {
                circuit_two = current_circuit_num;
            }
        }

        // Neither are in a circuit
        if circuit_one == usize::MAX && circuit_two == usize::MAX {
            circuits.push(BTreeSet::from([*first, *second]));

        // Both are in circuits
        } else if circuit_one != usize::MAX && circuit_two != usize::MAX {
            // If they're already in the same circuit, then we can continue
            if circuit_one == circuit_two {
                continue;
            }

            let min_index = circuit_one.min(circuit_two);
            let max_index = circuit_one.max(circuit_two);

            // Remove the junction with the larger index and insert all its elements into the other set
            let removed_circuit = circuits.remove(max_index);
            circuits[min_index].extend(removed_circuit);

        // First is in a circuit
        } else if circuit_one != usize::MAX {
            circuits[circuit_one].insert(*second);
        // Second is in a circuit
        } else if circuit_two != usize::MAX {
            circuits[circuit_two].insert(*first);
        }
    }

    let part_1 = circuits
        .iter()
        .map(|circuit| circuit.len() as u64)
        .sorted()
        .rev()
        .take(3)
        .product();

    let mut part_2 = 0;
    let mut seen_junction_boxes = HashSet::new();
    for (_, (first, second)) in distance_map.iter() {
        seen_junction_boxes.insert(first);
        seen_junction_boxes.insert(second);
        if seen_junction_boxes.len() == input.len() {
            part_2 = first.x * second.x;
            break;
        }
    }

    (part_1, part_2)
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input.lines().map(JunctionBox::from).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for JunctionBox {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();
        JunctionBox { x, y, z }
    }
}

impl JunctionBox {
    fn get_distance(&self, other: &JunctionBox) -> u64 {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input, 10).0, 40);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input, 10).1, 25272);
    }
}
