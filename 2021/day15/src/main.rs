const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &[i32]) -> i32 {
    1
}

fn part_2(input: &[i32]) -> i32 {
    1
}

// Parse grid of single digit ints into a Vec of <DangerLevel,Value,Visited=False>
// Going to cheat and assume a square cavern

struct CavernMap {
    Positions: Vec<Position>,
    UnvisitedPositions: Vec<(i32, i32)>,
    CavernLength: i32,
}

struct Position {
    DangerLevel: i32,
    ShortestPath: i32,
}

impl CavernMap {
    // Basically, do dijkstra's algorithm
    fn calculate_danger_levels() {}

    fn get_bottom_right_danger_level(x: i32, y: i32) -> i32 {
        1
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 40);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
