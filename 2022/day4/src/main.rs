const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &Vec<(AssignedSections, AssignedSections)>) -> i32 {
    let mut overlapping_pairs = 0;
    for (sections1, sections2) in input {
        if sections1.is_subset(sections2) {
            overlapping_pairs += 1;
        }
    }

    overlapping_pairs
}

fn part_2(input: &Vec<(AssignedSections, AssignedSections)>) -> i32 {
    let mut overlapping_pairs = 0;
    for (sections1, sections2) in input {
        if sections1.any_overlap(sections2) {
            overlapping_pairs += 1;
        }
    }

    overlapping_pairs
}

fn parse_input(input: &str) -> Vec<(AssignedSections, AssignedSections)> {
    let mut assignment_pairs = vec![];
    for line in input.lines() {
        let mut assignments = line.split(",");
        let mut assignment1 = assignments.next().unwrap().split("-");
        let mut assignment2 = assignments.next().unwrap().split("-");

        assignment_pairs.push((
            AssignedSections {
                start: assignment1.next().unwrap().parse().unwrap(),
                end: assignment1.next().unwrap().parse().unwrap(),
            },
            AssignedSections {
                start: assignment2.next().unwrap().parse().unwrap(),
                end: assignment2.next().unwrap().parse().unwrap(),
            },
        ))
    }
    assignment_pairs
}

struct AssignedSections {
    start: u32,
    end: u32,
}

impl AssignedSections {
    fn is_subset(&self, other: &AssignedSections) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }

    fn any_overlap(&self, other: &AssignedSections) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
            || (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 4);
    }
}
