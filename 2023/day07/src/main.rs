use std::collections::{BTreeMap, HashMap};

const INPUT: &str = include_str!("input.txt");

//247255339 is too high?

// Took test input from: https://www.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/

fn main() {
    let part_1_input = parse_input_part_1(INPUT);

    let part_1_answer = calculate_winnings(&part_1_input);
    println!("Part 1: {part_1_answer}");

    let part_2_input = parse_input_part_2(INPUT);
    let part_2_answer = calculate_winnings(&part_2_input);
    println!("Part 2: {part_2_answer}");
}

fn calculate_winnings(input: &Vec<(Hand, u64)>) -> u64 {
    let mut part_1_total = 0;
    let mut hand_rank = 1;

    let mut sorted = input.clone();
    sorted.sort_by(|a, b| (a.0.cmp(&b.0)));

    for (hand, bid) in sorted {
        println!("Hand: {:?}, Bid: {bid}, Rank: {hand_rank}", &hand);
        part_1_total += bid * hand_rank;
        hand_rank += 1;
    }

    part_1_total
}

fn parse_input_part_1(input: &str) -> Vec<(Hand, u64)> {
    let mut hands: Vec<(Hand, u64)> = Vec::new();
    for line in input.lines() {
        let mut split = line.split(' ');

        let hand = Hand::get_hand_part_1(split.next().unwrap());
        let bid = split.next().unwrap().parse::<u64>().unwrap();

        hands.push((hand, bid));
    }

    hands
}

fn parse_input_part_2(input: &str) -> Vec<(Hand, u64)> {
    let mut hands: Vec<(Hand, u64)> = Vec::new();
    for line in input.lines() {
        let mut split = line.split(' ');

        let hand = Hand::get_hand_part_2(split.next().unwrap());
        let bid = split.next().unwrap().parse::<u64>().unwrap();

        hands.push((hand, bid));
    }

    hands
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl Hand {
    fn get_hand_part_1(input: &str) -> Hand {
        let mut occurences = HashMap::new();

        let mut cards: Vec<Card> = vec![];
        for character in input.chars() {
            let card = Card::get_card(character, false);
            if let Some(value) = occurences.get(&card) {
                occurences.insert(card, value + 1);
            } else {
                occurences.insert(card, 1);
            }
            cards.push(card);
        }

        let mut occurences: Vec<(Card, usize)> = occurences.into_iter().collect();
        occurences.sort_by_key(|x| x.0);
        occurences.sort_by_key(|x| x.1);
        occurences.reverse();

        match occurences[0].1 {
            5 => Hand::FiveOfAKind(cards),
            4 => Hand::FourOfAKind(cards),
            3 => {
                if occurences[1].1 == 2 {
                    Hand::FullHouse(cards)
                } else {
                    Hand::ThreeOfAKind(cards)
                }
            }
            2 => {
                if occurences[1].1 == 2 {
                    Hand::TwoPair(cards)
                } else {
                    Hand::OnePair(cards)
                }
            }
            _ => Hand::HighCard(cards),
        }
    }

    fn get_hand_part_2(input: &str) -> Hand {
        let mut occurences = HashMap::new();

        let mut cards: Vec<Card> = vec![];
        let mut joker_count = 0;
        for character in input.chars() {
            let card = Card::get_card(character, true);
            if card == Card::Joker {
                joker_count += 1;
            } else if let Some(value) = occurences.get(&card) {
                occurences.insert(card, value + 1);
            } else {
                occurences.insert(card, 1);
            }
            cards.push(card);
        }

        if occurences.is_empty() {
            return Hand::FiveOfAKind(cards);
        }

        let mut occurences: Vec<(Card, usize)> = occurences.into_iter().collect();
        occurences.sort_by_key(|x| x.0);
        occurences.sort_by_key(|x| x.1);
        occurences.reverse();

        match occurences[0].1 + joker_count {
            5 => Hand::FiveOfAKind(cards),
            4 => Hand::FourOfAKind(cards),
            3 => {
                if occurences[1].1 == 2 {
                    Hand::FullHouse(cards)
                } else {
                    Hand::ThreeOfAKind(cards)
                }
            }
            2 => {
                if occurences[1].1 == 2 {
                    Hand::TwoPair(cards)
                } else {
                    Hand::OnePair(cards)
                }
            }
            _ => Hand::HighCard(cards),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn get_card(input: char, joker: bool) -> Card {
        use Card::*;
        match input {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => {
                if joker {
                    Joker
                } else {
                    Jack
                }
            }
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input_part_1(TESTINPUT);
        assert_eq!(calculate_winnings(&input), 6592);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input_part_2(TESTINPUT);
        assert_eq!(calculate_winnings(&input), 6839);
    }
}
