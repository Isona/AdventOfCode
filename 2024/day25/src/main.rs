#![feature(iter_array_chunks)]
const INPUT: &str = include_str!("input.txt");

fn main() {
    let (keys, locks) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&keys, &locks);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
}

fn part_1(keys: &[Vec<usize>], locks: &[Vec<usize>]) -> u64 {
    let mut part_1_output = 0;

    for key in keys {
        for lock in locks {
            if key
                .iter()
                .zip(lock.iter())
                .all(|(key_tumbler, lock_tumbler)| key_tumbler + lock_tumbler < 6)
            {
                part_1_output += 1
            }
        }
    }

    part_1_output
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut scratch_string = String::default();
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            let new_schematic = get_schematic(&scratch_string);

            if scratch_string.starts_with("#####") {
                locks.push(new_schematic);
            } else {
                keys.push(new_schematic);
            }
            scratch_string.clear();
        } else {
            scratch_string.push_str(line.trim());
        }
    }

    let new_schematic = get_schematic(&scratch_string);

    if scratch_string.starts_with("#####") {
        locks.push(new_schematic);
    } else {
        keys.push(new_schematic);
    }

    (keys, locks)
}

fn get_schematic(scratch_string: &str) -> Vec<usize> {
    let mut new_schematic = Vec::new();
    for tumbler_number in 0..5 {
        new_schematic.push(
            scratch_string
                .chars()
                .skip(tumbler_number)
                .step_by(5)
                .filter(|x| x == &'#')
                .count()
                - 1,
        )
    }

    new_schematic
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (keys, locks) = parse_input(TESTINPUT);
        assert_eq!(part_1(&keys, &locks), 3);
    }
}
