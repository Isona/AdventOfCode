use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &Vec<Scratchcard>) -> u32 {
    let mut part1_output = 0;
    for card in input {
        part1_output += card.get_card_score();
    }

    part1_output
}

fn part_2(input: &Vec<Scratchcard>) -> i32 {
    let mut card_count = 0;

    let mut won_cards = VecDeque::from_iter(1..=input.len());

    while let Some(current_card_number) = won_cards.pop_front() {
        // Continue if the card isn't not in the vector
        if current_card_number > input.len() {
            continue;
        }

        let current_card = &input[current_card_number - 1];
        let won_card_count = current_card.get_match_count();

        won_cards.append(&mut VecDeque::from_iter(
            current_card_number + 1..=current_card_number + won_card_count,
        ));

        card_count += 1;
    }

    card_count
}

#[derive(Default, Debug)]
struct Scratchcard {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: Vec<u32>,
}

impl Scratchcard {
    pub fn get_scratchcard(input: &str) -> Scratchcard {
        let mut scratchcard = Scratchcard::default();
        // Split around ": "
        let card_parts: Vec<&str> = input.split(&[':', '|']).collect();

        scratchcard.card_number = card_parts[0]
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        scratchcard.winning_numbers = card_parts[1]
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        scratchcard.card_numbers = card_parts[2]
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        scratchcard
    }

    pub fn get_card_score(&self) -> u32 {
        let mut score = 0;

        for card_number in &self.card_numbers {
            if self.winning_numbers.contains(&card_number) {
                score = Self::calculate_score(score);
            }
        }

        score
    }

    pub fn get_match_count(&self) -> usize {
        let mut match_count = 0;

        for card_number in &self.card_numbers {
            if self.winning_numbers.contains(&card_number) {
                match_count += 1;
            }
        }

        match_count
    }

    fn calculate_score(score: u32) -> u32 {
        if score == 0 {
            1
        } else {
            score * 2
        }
    }
}

fn parse_input(input: &str) -> Vec<Scratchcard> {
    input
        .lines()
        .map(|x| Scratchcard::get_scratchcard(x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 30);
    }
}
