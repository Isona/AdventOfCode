use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &[Scratchcard]) -> u32 {
    // Loop over the scratchcards and add the scores
    let mut part1_output = 0;
    for card in input {
        part1_output += card.get_card_score();
    }

    part1_output
}

fn part_2(input: &[Scratchcard]) -> usize {
    // Number of cards for output
    let mut card_count = 0;

    // Hashmap to store how many copies of each card we have
    // Initialise with 1 of each card index in the input vector
    let won_cards = (1..=input.len()).map(|x| (x, 1));
    let mut won_cards: HashMap<usize, usize> = HashMap::from_iter(won_cards);

    // Loop over each card
    for current_card_number in 1..=input.len() {
        // Calculate the number of copies we have and add to the output value
        let card_copies = *won_cards.get(&current_card_number).unwrap();
        card_count += card_copies;

        // Get the current card and how many matches
        let current_card = &input[current_card_number - 1];
        let won_card_count = current_card.get_match_count();

        // Iterate over won card numbers, adding the appropriate number of copies to the hashmap
        for won_card_number in current_card_number + 1..=current_card_number + won_card_count {
            if let Some(won_card_copies) = won_cards.get(&won_card_number) {
                won_cards.insert(won_card_number, won_card_copies + card_copies);
            }
        }
    }

    card_count
}

#[derive(Default, Debug)]
struct Scratchcard {
    winning_numbers: HashSet<u32>,
    card_numbers: Vec<u32>,
}

impl Scratchcard {
    pub fn get_scratchcard(input: &str) -> Scratchcard {
        let mut scratchcard = Scratchcard::default();

        // Split into 3 around the card definition, the winning numbers and card numbers
        let card_parts: Vec<&str> = input.split(&[':', '|']).collect();

        // Get the winning numbers from the second part
        scratchcard.winning_numbers = card_parts[1]
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        // Get the card numbers from the third part
        scratchcard.card_numbers = card_parts[2]
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        scratchcard
    }

    // Get the score for the card for part 1
    pub fn get_card_score(&self) -> u32 {
        let matches: u32 = self.get_match_count().try_into().unwrap();

        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }

    // Get the number of card numbers matches
    pub fn get_match_count(&self) -> usize {
        let mut match_count = 0;

        for card_number in &self.card_numbers {
            if self.winning_numbers.contains(card_number) {
                match_count += 1;
            }
        }

        match_count
    }
}

fn parse_input(input: &str) -> Vec<Scratchcard> {
    input.lines().map(Scratchcard::get_scratchcard).collect()
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
