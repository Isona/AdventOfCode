use core::fmt;
use std::fmt::Display;

use aoc_lib::Coordinate;

pub(crate) fn get_num_keypad() -> Vec<Coordinate> {
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

pub(crate) fn get_num_directions(
    start_coord: &Coordinate,
    end_coord: &Coordinate,
) -> Vec<Vec<KeypadDirection>> {
    if start_coord == end_coord {
        return vec![vec![KeypadDirection::A]];
    }

    let mut output = Vec::new();
    let mut path = Vec::new();
    // Left
    for _ in 0..start_coord.x.saturating_sub(end_coord.x) {
        path.push(KeypadDirection::Left)
    }
    //Right
    for _ in 0..end_coord.x.saturating_sub(start_coord.x) {
        path.push(KeypadDirection::Right);
    }

    // Down
    for _ in 0..end_coord.y.saturating_sub(start_coord.y) {
        path.push(KeypadDirection::Down);
    }

    // Up
    for _ in 0..start_coord.y.saturating_sub(end_coord.y) {
        path.push(KeypadDirection::Up)
    }

    path.push(KeypadDirection::A);
    if is_valid_num_path(start_coord, &path) {
        output.push(path.clone());
    }

    if start_coord.x != end_coord.x && start_coord.y != end_coord.y {
        path.pop();
        path.reverse();
        path.push(KeypadDirection::A);
        if is_valid_num_path(start_coord, &path) {
            output.push(path);
        }
    }

    output
}

fn is_valid_num_path(start_coord: &Coordinate, path: &[KeypadDirection]) -> bool {
    let mut current_x = start_coord.x;
    let mut current_y = start_coord.y;
    for direction in path {
        match direction {
            KeypadDirection::Up => current_y -= 1,
            KeypadDirection::Down => current_y += 1,
            KeypadDirection::Left => current_x -= 1,
            KeypadDirection::Right => current_x += 1,
            KeypadDirection::A => {}
        }

        if current_x == 0 && current_y == 3 {
            return false;
        }
    }

    true
}

// Get the shortest paths between directional keypad buttons
// A nice grid showing this here: https://www.reddit.com/r/adventofcode/comments/1hjj8lc/2024_day_21_upper_limit_on_the_number_of_sequences/
pub(crate) fn get_dir_keypad_directions(
    start: KeypadDirection,
    end: KeypadDirection,
) -> Option<Vec<KeypadDirection>> {
    if start == end {
        return Some(vec![KeypadDirection::A]);
    }
    match start {
        // Up
        KeypadDirection::Up => match end {
            KeypadDirection::Right => Some(vec![
                KeypadDirection::Down,
                KeypadDirection::Right,
                KeypadDirection::A,
            ]),
            KeypadDirection::Left => Some(vec![
                KeypadDirection::Down,
                KeypadDirection::Left,
                KeypadDirection::A,
            ]),
            KeypadDirection::A => Some(vec![KeypadDirection::Right, KeypadDirection::A]),
            _ => None,
        },
        // Down
        KeypadDirection::Down => match end {
            KeypadDirection::Left => Some(vec![KeypadDirection::Left, KeypadDirection::A]),
            KeypadDirection::Right => Some(vec![KeypadDirection::Right, KeypadDirection::A]),
            KeypadDirection::A => Some(vec![
                KeypadDirection::Up,
                KeypadDirection::Right,
                KeypadDirection::A,
            ]),
            _ => None,
        },
        // Left
        KeypadDirection::Left => match end {
            KeypadDirection::Up => Some(vec![
                KeypadDirection::Right,
                KeypadDirection::Up,
                KeypadDirection::A,
            ]),
            KeypadDirection::Down => Some(vec![KeypadDirection::Right, KeypadDirection::A]),
            KeypadDirection::A => Some(vec![
                KeypadDirection::Right,
                KeypadDirection::Right,
                KeypadDirection::Up,
                KeypadDirection::A,
            ]),
            _ => None,
        },
        // Right
        KeypadDirection::Right => match end {
            KeypadDirection::Up => Some(vec![
                KeypadDirection::Left,
                KeypadDirection::Up,
                KeypadDirection::A,
            ]),
            KeypadDirection::Down => Some(vec![KeypadDirection::Left, KeypadDirection::A]),
            KeypadDirection::A => Some(vec![KeypadDirection::Up, KeypadDirection::A]),
            _ => None,
        },
        // A
        KeypadDirection::A => match end {
            KeypadDirection::Up => Some(vec![KeypadDirection::Left, KeypadDirection::A]),
            KeypadDirection::Down => Some(vec![
                KeypadDirection::Left,
                KeypadDirection::Down,
                KeypadDirection::A,
            ]),
            KeypadDirection::Left => Some(vec![
                KeypadDirection::Down,
                KeypadDirection::Left,
                KeypadDirection::Left,
                KeypadDirection::A,
            ]),
            KeypadDirection::Right => Some(vec![KeypadDirection::Down, KeypadDirection::A]),
            _ => None,
        },
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(crate) enum KeypadDirection {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_keypad_distance_test() {
        let keypad = get_num_keypad();

        // Go from button 0 to 1, should be Up, Left, A
        let directions = get_num_directions(&keypad[0], &keypad[1]);
        assert_eq!(directions, vec![[
            KeypadDirection::Up,
            KeypadDirection::Left,
            KeypadDirection::A
        ]]);

        // Go from button 2 to button 9, should be Right, Up Up, A
        // Or Up, Up, Right, A
        let directions = get_num_directions(&keypad[2], &keypad[9]);
        assert_eq!(directions, vec![
            [
                KeypadDirection::Right,
                KeypadDirection::Up,
                KeypadDirection::Up,
                KeypadDirection::A
            ],
            [
                KeypadDirection::Up,
                KeypadDirection::Up,
                KeypadDirection::Right,
                KeypadDirection::A
            ]
        ]);

        // Go from button 9 to button A, should be Down, Down, Down A
        let directions = get_num_directions(&keypad[9], &keypad[10]);
        assert_eq!(directions, vec![[
            KeypadDirection::Down,
            KeypadDirection::Down,
            KeypadDirection::Down,
            KeypadDirection::A
        ]]);

        // Go from button A to button A, should be just A
        let directions = get_num_directions(&keypad[10], &keypad[10]);
        assert_eq!(directions, vec![[KeypadDirection::A]]);

        // Go from button 3 to 7, should get Left, Left, Up, Up A
        // Or Up, Up, Left, Left, A
        let directions = get_num_directions(&keypad[3], &keypad[7]);
        assert_eq!(directions, vec![
            [
                KeypadDirection::Left,
                KeypadDirection::Left,
                KeypadDirection::Up,
                KeypadDirection::Up,
                KeypadDirection::A
            ],
            [
                KeypadDirection::Up,
                KeypadDirection::Up,
                KeypadDirection::Left,
                KeypadDirection::Left,
                KeypadDirection::A
            ]
        ]);

        // Go from button A to 2, should get Left, Up, A
        // Or Up, Left, A
        let directions = get_num_directions(&keypad[10], &keypad[2]);
        assert_eq!(directions, vec![
            [
                KeypadDirection::Left,
                KeypadDirection::Up,
                KeypadDirection::A
            ],
            [
                KeypadDirection::Up,
                KeypadDirection::Left,
                KeypadDirection::A
            ]
        ]);
    }
}
