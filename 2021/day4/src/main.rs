use aoc_lib::Grid;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (calls, mut boards) = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1_and_2(&calls, &mut boards);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer}, Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1_and_2(calls: &[i32], boards: &mut Vec<BingoBoard>) -> (i32, i32) {
    let mut fastest_win = usize::MAX;
    let mut fastest_score = 0;

    let mut slowest_win = 0;
    let mut slowest_score = 0;

    for board in boards {
        if let Some((winning_turn, score)) = board.get_turn_and_score(calls) {
            if winning_turn < fastest_win {
                fastest_win = winning_turn;
                fastest_score = score;
            }

            if winning_turn > slowest_win {
                slowest_win = winning_turn;
                slowest_score = score;
            }
        }
    }

    (fastest_score, slowest_score)
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

        let mut board = Grid::new();
        for _ in 0..5 {
            let board_line: Vec<(i32, bool)> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| (x.parse().unwrap(), false))
                .collect();
            board.push_row(board_line);
        }

        boards.push(BingoBoard { board });
    }

    (calls, boards)
}

#[derive(Debug)]
struct BingoBoard {
    board: Grid<(i32, bool)>,
}

impl BingoBoard {
    fn get_turn_and_score(&mut self, calls: &[i32]) -> Option<(usize, i32)> {
        for (current_turn, call) in calls.iter().enumerate() {
            if let Some(coordinate) = self.board.find_item(&(*call, false)) {
                self.board.set(coordinate, (*call, true));

                if self
                    .board
                    .get_column(coordinate.x)
                    .all(|coord| self.board.get(coord).1)
                    || self
                        .board
                        .get_row(coordinate.y)
                        .all(|coord| self.board.get(coord).1)
                {
                    let score = self.get_score(calls[current_turn]);
                    return Some((current_turn, score));
                }
            }
        }
        None
    }

    fn get_score(&self, final_call: i32) -> i32 {
        final_call
            * self
                .board
                .indexed_iter()
                .filter(|(_, item)| !item.1)
                .map(|(_, item)| item.0)
                .sum::<i32>()
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
    fn part_1_and_2_test() {
        let (calls, mut boards) = parse_input(TESTINPUT);
        assert_eq!(part_1_and_2(&calls, &mut boards), (4512, 1924));
    }
}
