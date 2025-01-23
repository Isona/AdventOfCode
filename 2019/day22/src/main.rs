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
// 82066653842347 is too high
fn part_2(
    input: &[Shuffle],
    goal_index: i128,
    number_of_cards: i128,
    number_of_shuffles: i128,
) -> i128 {
    let (coefficient, constant) = get_inverse_coeff_and_constant(input, number_of_cards);
    let coeff_after_iterations = mod_pow(coefficient, number_of_shuffles, number_of_cards);

    println!("Coefficient: {coefficient}, coefficient after iterations: {coeff_after_iterations}");

    let constant_after_iterations = {
        if coefficient == 1 {
            constant * number_of_shuffles
        } else {
            constant * (1 - coeff_after_iterations) / (1 - coefficient)
        }
    };
    println!("Constant after iterations: {constant_after_iterations}");

    ((goal_index * coeff_after_iterations).rem_euclid(number_of_cards) + constant_after_iterations)
        .rem_euclid(number_of_cards)
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

fn get_inverse_coeff_and_constant(input: &[Shuffle], number_of_cards: i128) -> (i128, i128) {
    let mut coefficient = 1;
    let mut constant = 0;

    for shuffle in input.iter().rev() {
        match shuffle {
            Shuffle::DealNew => {
                coefficient = -coefficient;
                constant = -constant - 1;
            }
            Shuffle::Cut(cut_size) => constant += cut_size,
            Shuffle::Deal(deal_size) => {
                let inverse = inverse(*deal_size, number_of_cards);
                coefficient = (coefficient * inverse).rem_euclid(number_of_cards);
                constant = (constant * inverse).rem_euclid(number_of_cards);
            }
        }
    }

    (
        coefficient.rem_euclid(number_of_cards),
        constant.rem_euclid(number_of_cards),
    )
}

fn inverse(a: i128, n: i128) -> i128 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        let temp_t = t - quotient * newt;
        t = newt;
        newt = temp_t;

        let temp_r = r - quotient * newr;
        r = newr;
        newr = temp_r;
    }

    if r > 1 {
        panic!()
    }
    if t < 0 {
        t += n;
    }

    t
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
    fn deal_new_inverse_test() {
        let input = parse_input("deal into new stack");
        assert_eq!(part_2(&input, 1, 10, 1), 8);
        assert_eq!(part_2(&input, 1, 10, 2), 1);
    }

    #[test]
    fn cut_inverse_test() {
        let input = parse_input("cut 3");
        assert_eq!(part_2(&input, 2, 10, 1), 5);
        assert_eq!(part_2(&input, 2, 10, 2), 8);
    }

    #[test]
    fn deal_inverse_test() {
        let input = parse_input("deal with increment 3");
        assert_eq!(part_2(&input, 1, 10, 1), 7);
        assert_eq!(part_2(&input, 3, 10, 1), 1);

        assert_eq!(part_2(&input, 1, 10, 2), 9);
        assert_eq!(part_2(&input, 3, 10, 2), 7);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input, 7, 10, 1), 0);
        assert_eq!(part_2(&input, 3, 10, 1), 8);

        let initial_index = 9;
        let iterations = 1251223;
        let mut current_index = initial_index;
        for _ in 0..iterations {
            current_index = part_1(&input, current_index, 10);
        }

        assert_eq!(part_2(&input, current_index, 10, iterations), initial_index)
    }

    #[test]
    fn input_test() {
        let input = parse_input(INPUT);

        assert_eq!(part_2(&input, 5755, 10007, 1), 2019);

        let number_of_cards = 10007;

        let initial_index = 2020;
        let mut current_index = initial_index;
        let iterations = 2;
        for _ in 0..iterations {
            current_index = part_1(&input, current_index, number_of_cards);
        }

        println!("Current index: {current_index}");
        assert_eq!(
            part_2(&input, current_index, number_of_cards, iterations),
            initial_index
        )
    }

    #[test]
    fn mod_pow_test() {
        assert_eq!(mod_pow(15, 18, 23), 12);
        assert_eq!(mod_pow(2616, 19861, 252), 180);
    }
}
