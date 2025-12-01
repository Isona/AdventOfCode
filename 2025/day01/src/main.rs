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

fn part_1(input: &[Rotation]) -> u64 {
    let mut current = 50;
    let mut zero_count = 0;

    for rotation in input {
        current = rotation.rotate(current);
        if current == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part_2(input: &[Rotation]) -> i64 {
    let mut current = 50;
    let mut total_zero_count = 0;
    let mut current_zero_hits;

    for rotation in input {
        (current, current_zero_hits) = rotation.advanced_rotate(current);
        if current == 0 {
            total_zero_count += 1;
        }
        total_zero_count += current_zero_hits;
    }

    total_zero_count
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input.lines().map(Rotation::from).collect()
}

#[derive(Clone, Copy, Debug)]
struct Rotation {
    direction: Direction,
    magnitude: i64,
}

impl Rotation {
    fn rotate(&self, start: i64) -> i64 {
        match self.direction {
            Direction::Left => (start - self.magnitude).rem_euclid(100),
            Direction::Right => (start + self.magnitude).rem_euclid(100),
        }
    }

    // Returns (new_location, zero_hits)
    fn advanced_rotate(&self, start: i64) -> (i64, i64) {
        match self.direction {
            Direction::Left => {
                let new = start - self.magnitude.rem_euclid(100);
                if new < 0 && start != 0 {
                    (new.rem_euclid(100), self.magnitude / 100 + 1)
                } else {
                    (new.rem_euclid(100), self.magnitude / 100)
                }
            }
            Direction::Right => {
                let new = start + self.magnitude.rem_euclid(100);
                if new > 100 {
                    (new.rem_euclid(100), self.magnitude / 100 + 1)
                } else {
                    (new.rem_euclid(100), self.magnitude / 100)
                }
            }
        }
    }
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let magnitude = value[1..].parse().unwrap();
        let direction = match value.chars().next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!(),
        };
        Self {
            direction,
            magnitude,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 6);
    }
}
