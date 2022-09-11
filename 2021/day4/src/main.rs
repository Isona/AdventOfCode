use ndarray::Array2;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (calls, boards) = parse_input(INPUT);

    let part_1_answer = part_1(&calls, &boards);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&calls, &boards);
    println!("{part_2_answer}");
}

fn part_1(calls: &[i32], boards: &Vec<BingoBoard>) -> i32 {
    0
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
    dbg!(&calls);
    while input.peek().is_some() {
        // Discard empty line
        input.next();
        let mut raw_board: Vec<i32> = Vec::new();
        for _ in 0..5 {
            let mut board_line: Vec<i32> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            raw_board.append(&mut board_line);
        }
        let board: Array2<i32> = Array2::from_shape_vec((5, 5), raw_board).unwrap();
        dbg!(&board);
        boards.push(BingoBoard { board });
    }
    //input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
    return (calls, Vec::new());
}

#[derive(Debug)]
struct BingoBoard {
    board: Array2<i32>,
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
        let input = parse_input(TESTINPUT);
        panic!();
        //assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn part_2_test() {
        //let input = parse_input(TESTINPUT);
        //assert_eq!(part_2(&input), 5);
    }
}
