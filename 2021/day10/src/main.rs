const INPUT: &str = include_str!("input.txt");
const LEFT_BRACKETS: [char; 4] = ['(', '[', '{', '<'];

fn main() {
    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1_and_2(INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} calculated at the same time");

    // let start = std::time::Instant::now();
    // let part_2_answer = part_2(INPUT);

    // let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    // println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1_and_2(input: &str) -> (u64, u64) {
    let mut corruption_sum = 0;
    let mut completion_values = Vec::new();
    'outer_loop: for line in input.lines() {
        let mut unmatched_bracket_stack: Vec<char> = Vec::new();
        for bracket in line.chars() {
            if LEFT_BRACKETS.contains(&bracket) {
                unmatched_bracket_stack.push(bracket);
            } else {
                match bracket {
                    ')' => {
                        if unmatched_bracket_stack.pop().unwrap() != '(' {
                            corruption_sum += 3;
                            continue 'outer_loop;
                        }
                    }
                    ']' => {
                        if unmatched_bracket_stack.pop().unwrap() != '[' {
                            corruption_sum += 57;
                            continue 'outer_loop;
                        }
                    }
                    '}' => {
                        if unmatched_bracket_stack.pop().unwrap() != '{' {
                            corruption_sum += 1197;
                            continue 'outer_loop;
                        }
                    }
                    '>' => {
                        if unmatched_bracket_stack.pop().unwrap() != '<' {
                            corruption_sum += 25137;
                            continue 'outer_loop;
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        let mut current_completion_score = 0;
        for left_bracket in unmatched_bracket_stack.iter().rev() {
            current_completion_score *= 5;
            current_completion_score += match left_bracket {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            }
        }
        completion_values.push(current_completion_score);
    }

    completion_values.sort();
    let final_completion_score = completion_values.get(completion_values.len() / 2).unwrap();
    (corruption_sum, *final_completion_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1_and_2(TESTINPUT).0, 26397);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_1_and_2(TESTINPUT).1, 288957);
    }
}
