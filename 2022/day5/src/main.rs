const INPUT: &str = include_str!("input.txt");

fn main() {
    let (crates, instructions) = parse_input(INPUT);

    let part_1_answer = part_1(crates.clone(), &instructions);
    println!("{part_1_answer}");

    let part_2_answer = part_2(crates, &instructions);
    println!("{part_2_answer}");
}

fn part_1(crates: Vec<Vec<char>>, instructions: &Vec<(i32, i32, i32)>) -> String {
    todo!()
}

fn part_2(crates: Vec<Vec<char>>, instructions: &Vec<(i32, i32, i32)>) -> String {
    todo!()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let mut stack_count = 0;
    let mut instructions = vec![];

    // Input lines like: move 1 from 2 to 1
    for line in input.lines() {
        if line.starts_with("move") {
            let split_line = line.split(" ").collect::<Vec<&str>>();
            let count = split_line[1].parse::<i32>().unwrap();
            let source = split_line[3].parse::<i32>().unwrap() - 1;
            let dest = split_line[5].parse::<i32>().unwrap() - 1;
            instructions.push((count, source, dest));
        }
        // Get the number of stacks
        else if line.starts_with(" 1") {
            stack_count = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else if !line.is_empty() {
        }
    }

    let mut crates = vec![vec![]; stack_count];

    (crates, instructions)
    // input.lines().map(|x| x.parse::<i32>().unwrap()).collect()(crates, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (crates, instructions) = parse_input(TESTINPUT);
        assert_eq!(part_1(crates, &instructions), "CMZ");
    }

    #[test]
    fn part_2_test() {
        let (crates, instructions) = parse_input(TESTINPUT);
        assert_eq!(part_2(crates, &instructions), "TODO");
    }
}
