use core::fmt;
use std::fmt::Display;

use aoc_lib::Coordinate;
use cached::proc_macro::cached;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = std::time::Instant::now();
    let part_1_answer = part_1(&INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

// 184712 is too high
fn part_1(input: &str) -> u64 {
    let numeric_keypad = get_num_keypad();
    let directional_keypad = get_directional_keypad();

    let mut output = 0;
    for line in input.lines() {
        // Get the individual buttons to push, as coords
        let mut buttons = Vec::from([numeric_keypad[10]]);
        buttons.extend(
            line.chars()
                .map(|x| numeric_keypad[x.to_digit(16).unwrap() as usize]),
        );
        println!("{buttons:?}");

        let second_buttons = get_next_buttons(&directional_keypad, &buttons, true);
        println!("Second Buttons: {second_buttons:?}");

        let third_buttons = get_next_buttons(&directional_keypad, &second_buttons, false);
        println!("Third Buttons: {third_buttons:?}");

        let fourth_buttons = get_next_buttons(&directional_keypad, &third_buttons, false);
        println!("Fourth Buttons: {fourth_buttons:?}");

        let input_number: usize = line[0..line.len() - 1].parse().unwrap();
        println!();
        println!(
            "{line}: {input_number}, first_buttons_len:{}, second_buttons_len: {}, third_buttons_len: {}, fourth_buttons_len: {}",
            buttons.len(),
            second_buttons.len(),
            third_buttons.len(),
            fourth_buttons.len() - 1
        );
        println!();
        output += (input_number * (fourth_buttons.len() - 1)) as u64;
    }

    output
}

fn part_2(input: &str) -> u64 {
    todo!();
}

fn get_next_buttons(
    directional_keypad: &[Coordinate],
    initial_buttons: &[Coordinate],
    is_num_keypad: bool,
) -> Vec<Coordinate> {
    let mut buttons = Vec::from([directional_keypad[KeypadDirection::A as usize]]);
    for (start_button, end_button) in initial_buttons.iter().tuple_windows() {
        buttons.extend(
            get_directions(*start_button, *end_button, is_num_keypad)
                .iter()
                .map(|direction| directional_keypad[*direction as usize]),
        );
    }

    buttons
}

fn get_num_keypad() -> Vec<Coordinate> {
    // Top row
    let coord_7 = Coordinate::new(0, 0);
    let coord_8 = Coordinate::new(1, 0);
    let coord_9 = Coordinate::new(2, 0);

    // Second row
    let coord_4 = Coordinate::new(0, 1);
    let coord_5 = Coordinate::new(1, 1);
    let coord_6 = Coordinate::new(2, 1);

    // Third row

    let coord_1 = Coordinate::new(0, 2);
    let coord_2 = Coordinate::new(1, 2);
    let coord_3 = Coordinate::new(2, 2);

    // Fourth row
    let coord_0 = Coordinate::new(1, 3);
    let coord_a = Coordinate::new(2, 3);

    Vec::from([
        coord_0, coord_1, coord_2, coord_3, coord_4, coord_5, coord_6, coord_7, coord_8, coord_9,
        coord_a,
    ])
}

#[cached]
fn get_directions(
    start_coord: Coordinate,
    end_coord: Coordinate,
    is_num_keypad: bool,
) -> Vec<KeypadDirection> {
    let mut output = Vec::new();

    let prioritise_left = is_num_keypad && (start_coord.y != 3 || end_coord.x != 0);
    if prioritise_left {
        for _ in 0..start_coord.x.saturating_sub(end_coord.x) {
            output.push(KeypadDirection::Left)
        }
    }

    // Prioritise right
    for _ in 0..end_coord.x.saturating_sub(start_coord.x) {
        output.push(KeypadDirection::Right);
    }

    // Prioritise down
    for _ in 0..end_coord.y.saturating_sub(start_coord.y) {
        output.push(KeypadDirection::Down);
    }

    // Up
    for _ in 0..start_coord.y.saturating_sub(end_coord.y) {
        output.push(KeypadDirection::Up)
    }

    // Left
    if !prioritise_left {
        for _ in 0..start_coord.x.saturating_sub(end_coord.x) {
            output.push(KeypadDirection::Left)
        }
    }

    output.push(KeypadDirection::A);
    output
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KeypadDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    A = 4,
}

impl Display for KeypadDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeypadDirection::Up => write!(f, "^"),
            KeypadDirection::Down => write!(f, "v"),
            KeypadDirection::Left => write!(f, "<"),
            KeypadDirection::Right => write!(f, ">"),
            KeypadDirection::A => write!(f, "A"),
        }
    }
}

fn get_directional_keypad() -> Vec<Coordinate> {
    let up_coord = Coordinate::new(1, 0);
    let down_coord = Coordinate::new(1, 1);
    let left_coord = Coordinate::new(0, 1);
    let right_coord = Coordinate::new(2, 1);
    let a_coord = Coordinate::new(2, 0);

    Vec::from([up_coord, down_coord, left_coord, right_coord, a_coord])
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(&TESTINPUT), 7);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(&TESTINPUT), 5);
    }

    #[test]
    fn numeric_keypad_distance_test() {
        let keypad = get_num_keypad();

        // Go from button 0 to 1, should be Up, Left, A
        let directions = get_directions(keypad[0], keypad[1], true);
        assert_eq!(
            directions,
            Vec::from([
                KeypadDirection::Up,
                KeypadDirection::Left,
                KeypadDirection::A
            ])
        );

        // Go from button 2 to button 9, should be Right, Up Up, A
        let directions = get_directions(keypad[2], keypad[9], true);
        assert_eq!(
            directions,
            Vec::from([
                KeypadDirection::Right,
                KeypadDirection::Up,
                KeypadDirection::Up,
                KeypadDirection::A
            ])
        );

        // Go from button 9 to button A, should be Down, Down, Down A
        let directions = get_directions(keypad[9], keypad[10], true);
        assert_eq!(
            directions,
            Vec::from([
                KeypadDirection::Down,
                KeypadDirection::Down,
                KeypadDirection::Down,
                KeypadDirection::A
            ])
        );

        // Go from button A to button A, should be just A
        let directions = get_directions(keypad[10], keypad[10], true);
        assert_eq!(directions, Vec::from([KeypadDirection::A]));

        // Go from button 3 to 7, should get Left, Left, Up, Up A
        let directions = get_directions(keypad[3], keypad[7], true);
        assert_eq!(
            directions,
            Vec::from([
                KeypadDirection::Left,
                KeypadDirection::Left,
                KeypadDirection::Up,
                KeypadDirection::Up,
                KeypadDirection::A
            ])
        );

        // Go from button A to 2, should get Left, Up, A
        let directions = get_directions(keypad[10], keypad[2], true);
        assert_eq!(
            directions,
            Vec::from([
                KeypadDirection::Left,
                KeypadDirection::Up,
                KeypadDirection::A
            ])
        );
    }
}
