use std::i32::MAX;

use ndarray::Array2;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (calls, mut boards) = parse_input(INPUT);

    let part_1_answer = part_1(&calls, &mut boards);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&calls, &boards);
    println!("{part_2_answer}");
}

fn part_1(calls: &[i32], boards: &mut Vec<BingoBoard>) -> i32 {
    let mut fastest_win = MAX;
    let mut score = 0;
    for board in boards {
        if let Some(winning_turn) = board.get_winning_turn(&calls) {
            dbg!(&winning_turn);
            if winning_turn < fastest_win {
                fastest_win = winning_turn;
                score = board.get_score(&calls, winning_turn);
            }
        }
    }

    score
}

fn part_2(calls: &[i32], boards: &Vec<BingoBoard>) -> i32 {
    0
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut input = input.lines().peekable();

    let calls: Vec<i32> = input
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    while input.peek().is_some() {
        // Discard empty line
        input.next();
        let mut raw_board: Vec<(i32, bool)> = Vec::new();
        for _ in 0..5 {
            let mut board_line: Vec<(i32, bool)> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| (x.parse().unwrap(), false))
                .collect();
            raw_board.append(&mut board_line);
        }
        let board: Array2<(i32, bool)> = Array2::from_shape_vec((5, 5), raw_board).unwrap();
        boards.push(BingoBoard { board });
    }
    //input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
    return (calls, Vec::new());
}

#[derive(Debug)]
struct BingoBoard {
    board: Array2<(i32, bool)>,
}

impl BingoBoard {
    fn get_winning_turn(&mut self, calls: &[i32]) -> Option<i32> {
        for i in 0..calls.len() {
            for item in self.board.iter_mut() {
                if calls[i] == item.0 {
                    item.1 = true;
                    break;
                }
            }
            if self.has_won() {
                return Some(i.try_into().unwrap());
            }
        }

        None
    }

    fn has_won(&self) -> bool {
        for row in self.board.rows() {
            if !row.iter().any(|x| x.1 == false) {
                return true;
            }
        }

        for column in self.board.columns() {
            if !column.iter().any(|x| x.1 == false) {
                return true;
            }
        }

        return false;
    }

    fn get_score(&self, calls: &[i32], winning_turn: i32) -> i32 {
        let mut sum = 0;
        for item in self.board.iter() {
            if !item.1 {
                sum += item.0;
            }
        }

        sum * calls[winning_turn as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn part_1_test() {
        let (calls, mut boards) = parse_input(TESTINPUT);
        assert_eq!(part_1(&calls, &mut boards), 4512);
    }

    #[test]
    fn part_2_test() {
        //let input = parse_input(TESTINPUT);
        //assert_eq!(part_2(&input), 5);
    }
}
