const INPUT: &str = include_str!("input.txt");

fn main() {
    let part_1_answer = part_1(INPUT);
    println!("{part_1_answer}");

    let part_2_answer = part_2(INPUT);
    println!("{part_2_answer}");
}

fn part_1(input: &str) -> i32 {
    let mut current_position = 0;
    let mut trees_hit = 0;

    for line in input.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        if line_chars[current_position] == '#' {
            trees_hit += 1;
        }
        current_position = (current_position + 3) % line.len();
    }

    trees_hit
}

fn part_2(input: &str) -> i64 {
    let mut trees_hit_r1d1 = 0;
    let mut trees_hit_r3d1 = 0;
    let mut trees_hit_r5d1 = 0;
    let mut trees_hit_r7d1 = 0;
    let mut trees_hit_r1d2 = 0;

    for (current_y, line) in input.lines().enumerate() {
        let line_chars: Vec<char> = line.chars().collect();
        let line_len = line_chars.len();

        if line_chars[(current_y) % line_len] == '#' {
            trees_hit_r1d1 += 1;
        }

        if line_chars[(current_y * 3) % line_len] == '#' {
            trees_hit_r3d1 += 1;
        }

        if line_chars[(current_y * 5) % line_len] == '#' {
            trees_hit_r5d1 += 1;
        }

        if line_chars[(current_y * 7) % line_len] == '#' {
            trees_hit_r7d1 += 1;
        }

        if current_y % 2 == 0 && line_chars[(current_y / 2) % line_len] == '#' {
            trees_hit_r1d2 += 1;
        }
    }

    trees_hit_r1d1 * trees_hit_r3d1 * trees_hit_r5d1 * trees_hit_r7d1 * trees_hit_r1d2
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TESTINPUT), 7);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TESTINPUT), 336);
    }
}
