use std::{collections::HashMap, iter::zip};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut list1, mut list2) = parse_input(INPUT);

    let part_1_answer = part_1(&mut list1, &mut list2);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&mut list1, &mut list2);
    println!("Part 2: {part_2_answer}");
}

fn part_1(list1: &mut [i64], list2: &mut [i64]) -> i64 {
    list1.sort();
    list2.sort();
    let iter = zip(list1, list2);

    iter.map(|(x, y)| (*x - *y).abs()).sum()
}

fn part_2(list1: &mut [i64], list2: &mut [i64]) -> i64 {
    let mut hash2: HashMap<i64, i64> = HashMap::new();
    for item in list2 {
        *hash2.entry(*item).or_default() += 1;
    }

    let mut total = 0;

    for item in list1 {
        if hash2.contains_key(item) {
            total += *item * hash2.get(item).unwrap();
        }
    }

    total
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(' ');

        list1.push(parts.next().unwrap().parse::<i64>().unwrap());
        list2.push(parts.last().unwrap().parse::<i64>().unwrap());
    }

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (mut list1, mut list2) = parse_input(TESTINPUT);
        assert_eq!(part_1(&mut list1, &mut list2), 11);
    }

    #[test]
    fn part_2_test() {
        let (mut list1, mut list2) = parse_input(TESTINPUT);
        assert_eq!(part_2(&mut list1, &mut list2), 31);
    }
}
