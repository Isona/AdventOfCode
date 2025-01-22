const INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input, 2019, 10007);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input, 2020, 119315717514047, 101741582076661);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &[Shuffle], goal_card: i128, number_of_cards: i128) -> i128 {
    let (coefficient, constant) = get_coeff_and_constant(input, number_of_cards);
    (goal_card * coefficient + constant).rem_euclid(number_of_cards)
}

// 20066843390330 is too low
fn part_2(
    input: &[Shuffle],
    goal_index: i128,
    number_of_cards: i128,
    number_of_shuffles: i128,
) -> i128 {
    let (coefficient, constant) = get_coeff_and_constant(input, number_of_cards);
    println!("Coefficient: {coefficient}, Constant: {constant}");

    // coefficient * x ≡ goal_index % number_of_cards
    //let (coeff_bezout, card_bezout) = extended_euclid(coefficient, number_of_cards);

    let coeff_after_iterations = mod_pow(coefficient, number_of_shuffles, number_of_cards);
    let constant_after_iterations = constant * (1 - coeff_after_iterations) / (1 - coefficient);

    //let goal = (goal_index - coefficient).rem_euclid(number_of_cards);
    let goal = (goal_index - constant_after_iterations).rem_euclid(number_of_cards);

    //println!("{}", goal);

    let (coeff_bezout, _card_bezout) = extended_euclid(coeff_after_iterations, number_of_cards);

    (goal * coeff_bezout).rem_euclid(number_of_cards)

    //(goal * card_bezout).rem_euclid(number_of_cards)
}

fn get_coeff_and_constant(input: &[Shuffle], number_of_cards: i128) -> (i128, i128) {
    let mut coefficient = 1;
    let mut constant = 0;
    for shuffle in input {
        match shuffle {
            // Shuffle::DealNew => current_index = (-current_index - 1).rem_euclid(number_of_cards),
            // Shuffle::Cut(cut_size) => current_index -= cut_size,
            // Shuffle::Deal(deal_size) => current_index *= deal_size,
            Shuffle::DealNew => {
                coefficient = -coefficient;
                constant = -constant - 1;
            }
            Shuffle::Cut(cut_size) => constant -= cut_size,
            Shuffle::Deal(deal_size) => {
                coefficient = (coefficient * deal_size).rem_euclid(number_of_cards);
                constant = (constant * deal_size).rem_euclid(number_of_cards);
            }
        }
    }
    (coefficient, constant)
}

fn extended_euclid(a: i128, b: i128) -> (i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        let temp_r = old_r - quotient * r;
        old_r = r;
        r = temp_r;

        let temp_s = old_s - quotient * s;
        old_s = s;
        s = temp_s;

        let temp_t = old_t - quotient * t;
        old_t = t;
        t = temp_t;
    }

    (old_s, old_t)
}

fn mod_pow(mut b: i128, mut e: i128, m: i128) -> i128 {
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    b = b.rem_euclid(m);
    while e > 0 {
        if e.rem_euclid(2) == 1 {
            r = (r * b).rem_euclid(m)
        }
        b = (b * b).rem_euclid(m);
        e /= 2;
    }
    r
}

fn parse_input(input: &str) -> Vec<Shuffle> {
    input.lines().map(Shuffle::from).collect()
}

// Deal into new stack
// Cut N cards
// Deal with increment N
#[derive(Debug, Eq, PartialEq)]
enum Shuffle {
    DealNew,
    Cut(i128),
    Deal(i128),
}

impl From<&str> for Shuffle {
    fn from(value: &str) -> Self {
        if value == "deal into new stack" {
            Shuffle::DealNew
        } else if value.starts_with("deal with increment") {
            let end_value = value.split_whitespace().last().unwrap().parse().unwrap();
            Shuffle::Deal(end_value)
        } else if value.starts_with("cut") {
            let end_value = value.split_whitespace().last().unwrap().parse().unwrap();
            Shuffle::Cut(end_value)
        } else {
            panic!()
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
        assert_eq!(part_1(&input, 7, 10), 6);
        assert_eq!(part_1(&input, 5, 10), 2);
        assert_eq!(part_1(&input, 3, 10), 8);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input, 7, 10, 1), 0);
        assert_eq!(part_2(&input, 3, 10, 1), 8);

        let initial_index = 9;
        let iterations = 151;
        let mut current_index = initial_index;
        for _ in 0..iterations {
            current_index = part_1(&input, current_index, 10);
        }

        assert_eq!(part_2(&input, current_index, 10, iterations), initial_index)
    }

    #[test]
    fn input_test() {
        let input = parse_input(INPUT);
        let number_of_cards = 119315717514047;

        let initial_index = 5;
        let mut current_index = initial_index;
        let iterations = 5;
        for _ in 0..iterations {
            current_index = part_1(&input, current_index, number_of_cards);
        }

        assert_eq!(
            part_2(&input, current_index, number_of_cards, iterations),
            initial_index
        )
    }
}
