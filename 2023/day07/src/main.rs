use std::collections::{BTreeMap, HashMap};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &BTreeMap<Hand, u64>) -> u64 {
    let mut part_1_total = 0;
    let mut hand_rank = 1;

    for (hand, bid) in input {
        println!("Hand: {:?}, Bid: {bid}", &hand);
        part_1_total += bid * hand_rank;
        hand_rank += 1;
    }

    part_1_total
}

fn part_2(input: &BTreeMap<Hand, u64>) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> BTreeMap<Hand, u64> {
    let mut hands: BTreeMap<Hand, u64> = BTreeMap::new();
    for line in input.lines() {
        let mut split = line.split(' ');

        let hand = Hand::get_hand(split.next().unwrap());
        let bid = split.next().unwrap().parse::<u64>().unwrap();

        hands.insert(hand, bid);
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
    fn get_hand(input: &str) -> Hand {
        let mut occurences = HashMap::new();

        for character in input.chars() {
            let card = Card::get_card(character);
            if let Some(value) = occurences.get(&card) {
                occurences.insert(card, value + 1);
            } else {
                occurences.insert(card, 1);
            }
        }

        let mut occurences: Vec<(Card, usize)> = occurences.into_iter().collect();
        occurences.sort_by_key(|x| x.0);
        occurences.sort_by_key(|x| x.1);
        occurences.reverse();

        let cards = occurences
            .iter()
            .flat_map(|&(card, count)| std::iter::once(card).cycle().take(count))
            .collect();

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
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card {
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
    fn get_card(input: char) -> Card {
        use Card::*;
        match input {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
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
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 6440);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
