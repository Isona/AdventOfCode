const INPUT: &str = include_str!("input.txt");
mod helper_functions;

use cached::proc_macro::cached;
use helper_functions::{KeypadDirection, get_dir_keypad_directions, get_num_directions};
use itertools::Itertools;

fn main() {
    let start = std::time::Instant::now();
    let part_1_answer = get_answer(INPUT, 2);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = get_answer(INPUT, 25);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn get_answer(input: &str, robot_count: usize) -> u64 {
    let mut output = 0;
    let num_keypad = helper_functions::get_num_keypad();

    for line in input.lines() {
        // Get the buttons to push on the first directional keyboard, start with A
        let mut numeric_coords = vec![num_keypad[10]];
        numeric_coords.extend(
            line.chars()
                .map(|x| num_keypad[x.to_digit(16).unwrap() as usize]),
        );

        // Get the shortest path for each window of size 2 in the path and add them
        let mut path_length = 0;
        for (start_coord, end_coord) in numeric_coords.iter().tuple_windows() {
            let mut min_path: u64 = u64::MAX;
            for mut path in get_num_directions(start_coord, end_coord) {
                // Prepend A - our start position
                path.insert(0, KeypadDirection::A);
                min_path = min_path.min(get_shortest_path(path, robot_count))
            }
            path_length += min_path
        }

        // Calculate the output value, print out progress
        let input_number: u64 = line[0..line.len() - 1].parse().unwrap();
        println!("{line}: {path_length}");
        output += path_length * input_number;
    }

    output
}

#[cached]
fn get_shortest_path(path: Vec<KeypadDirection>, remaining_bots: usize) -> u64 {
    // If this is the last robot, then this is the final path for this section
    if remaining_bots == 0 {
        return (path.len() - 1).try_into().unwrap();
    }
    let mut path_len = 0;
    // For each window, generate the path between them and recurse with 1 fewer robots, sum the returned path lengths
    for (start_dir, end_dir) in path.iter().tuple_windows() {
        let mut sub_path = get_dir_keypad_directions(*start_dir, *end_dir).unwrap();
        sub_path.insert(0, KeypadDirection::A);
        path_len += get_shortest_path(sub_path, remaining_bots - 1);
    }

    path_len
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(get_answer(TESTINPUT, 2), 126384);
    }
}
