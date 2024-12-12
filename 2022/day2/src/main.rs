const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &[Round]) -> i32 {
    let mut score = 0;
    for round in input {
        score += round.get_round_score_part_1();
    }

    score
}

fn part_2(input: &[Round]) -> i32 {
    let mut score = 0;
    for round in input {
        score += round.get_round_score_part_2();
    }

    score
}

struct Round {
    opponent_shape: Shape,
    my_shape: Shape,
    desired_result: RoundResult,
}

impl Round {
    fn get_round(input_line: &str) -> Round {
        let mut input = input_line.split(" ");
        let opponent_shape = Shape::get_shape(input.next().unwrap());
        let second_item = input.next().unwrap();

        Round {
            opponent_shape,
            my_shape: Shape::get_shape(second_item),
            desired_result: RoundResult::get_result(second_item),
        }
    }

    fn get_round_score_part_1(&self) -> i32 {
        self.my_shape.get_shape_score() + self.get_round_result().get_result_score()
    }

    fn get_round_score_part_2(&self) -> i32 {
        let score = self
            .opponent_shape
            .get_score_from_desired_result(&self.desired_result);

        score + self.desired_result.get_result_score()
    }

    fn get_round_result(&self) -> RoundResult {
        if self.opponent_shape == self.my_shape {
            RoundResult::Draw
        } else if (self.my_shape == Shape::Rock && self.opponent_shape == Shape::Scissors)
            || (self.my_shape == Shape::Paper && self.opponent_shape == Shape::Rock)
            || (self.my_shape == Shape::Scissors && self.opponent_shape == Shape::Paper)
        {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_shape(input: &str) -> Shape {
        match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            _ => Shape::Scissors,
        }
    }
    fn get_shape_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn get_score_from_desired_result(&self, desired_result: &RoundResult) -> i32 {
        match self.get_shape_score() + desired_result.get_score_vector() {
            1 | 4 => 1,
            0 | 3 => 3,
            _ => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl RoundResult {
    fn get_result(input: &str) -> RoundResult {
        match input {
            "X" => RoundResult::Lose,
            "Z" => RoundResult::Win,
            _ => RoundResult::Draw,
        }
    }
    fn get_result_score(&self) -> i32 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
    fn get_score_vector(&self) -> i32 {
        match self {
            RoundResult::Lose => -1,
            RoundResult::Draw => 0,
            RoundResult::Win => 1,
        }
    }
}

fn parse_input(input: &str) -> Vec<Round> {
    input.lines().map(Round::get_round).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 15);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 12);
    }
}
