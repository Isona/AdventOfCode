use itertools::Itertools;

const INPUT: (i32, i32) = (137683, 596253);

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

fn part_1(range: &(i32, i32)) -> usize {
    (range.0..range.1).filter(meets_criteria_part_1).count()
}

fn meets_criteria_part_1(input_number: &i32) -> bool {
    let mut adjacent = false;
    for (digit_1, digit_2) in input_number.to_string().chars().tuple_windows() {
        // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
        if digit_2.to_digit(10).unwrap() < digit_1.to_digit(10).unwrap() {
            return false;
        }
        // Two adjacent digits are the same
        if digit_1 == digit_2 {
            adjacent = true;
        }
    }

    adjacent
}

fn part_2(range: &(i32, i32)) -> usize {
    (range.0..range.1).filter(meets_criteria_part_2).count()
}

fn meets_criteria_part_2(input_number: &i32) -> bool {
    for (digit_1, digit_2) in input_number.to_string().chars().tuple_windows() {
        // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
        if digit_2.to_digit(10).unwrap() < digit_1.to_digit(10).unwrap() {
            return false;
        }
    }

    let mut last_digit = 10;
    let mut last_digit_count = 0;
    for digit in input_number.to_string().chars() {
        // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
        let real_digit = digit.to_digit(10).unwrap();
        if real_digit != last_digit {
            if last_digit_count == 2 {
                return true;
            }
            last_digit = real_digit;
            last_digit_count = 1;
        } else {
            last_digit_count += 1;
        }
    }

    last_digit_count == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert!(meets_criteria_part_1(&111111));
        assert!(!meets_criteria_part_1(&223450));
        assert!(!meets_criteria_part_1(&123789));
    }
    #[test]
    fn part_2_test() {
        assert!(meets_criteria_part_2(&112233));
        assert!(!meets_criteria_part_2(&123444));
        assert!(meets_criteria_part_2(&111122));
    }
}
