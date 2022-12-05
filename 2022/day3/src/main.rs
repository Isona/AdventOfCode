use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(backpacks: &Vec<Backpack>) -> i32 {
    let mut priority_sum = 0;

    for (backpack) in backpacks {
        priority_sum += get_item_value(backpack.get_intersection());
    } 
    
    priority_sum
}

fn part_2(backpacks: &Vec<Backpack>) -> i32 {
    let mut badge_sum = 0;

    let backpack_iter = backpacks.iter();

    for backpack in backpack_iter {
        badge_sum += get_item_value(backpack.get_security_badge(backpack_iter.next().unwrap(), backpack_iter.next().unwrap()))
    }

    badge_sum
}

fn get_item_value(input: char) -> i32 {
    if input.is_lowercase() {
        input as i32 - 96
    }
    else {
        input as i32 - 38
    }
}

struct Backpack {
    all_items: HashSet<char>,
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

impl Backpack {
    fn get_backpack(input: &str) -> Backpack {
        Backpack {
            all_items: input.chars().collect(),
            first_compartment: input[0..input.len()/2].chars().collect(),
            second_compartment: input[input.len()/2..input.len()].chars().collect()
        }
    }

    fn get_intersection(&self) -> char {
        return self.first_compartment.intersection(&self.second_compartment).next().unwrap().clone()
    }

    fn get_security_badge(&self, second_elf: &Backpack, third_elf: &Backpack) -> char {
        let first_interscetion: HashSet<&char> = self.all_items.intersection(&second_elf.all_items).collect();
        first_interscetion.intersection(third_elf.all_items).next().unwrap().clone().to_owned()
    }
}

fn parse_input(input: &str) -> Vec<Backpack> {
    input.lines().map(|x| Backpack::get_backpack(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 157);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
