const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let part_1_answer = part_1(&mut input);
    println!("{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("{part_2_answer}");
}

fn part_1(input: &mut [Vec<Position>]) -> usize {
    // Horizontal, left to right

    for row in input.iter_mut() {
        let mut visible_height = 0;

        for position in row.iter_mut() {
            if position.tree_size >= visible_height {
                position.visible = true;
                visible_height = position.tree_size + 1;
            }

            if visible_height == 10 {
                break;
            }
        }
    }

    for row in input.iter_mut() {
        let mut visible_height = 0;

        for position in row.iter_mut().rev() {
            if position.tree_size >= visible_height {
                position.visible = true;
                visible_height = position.tree_size + 1;
            }

            if visible_height == 10 {
                break;
            }
        }
    }

    for column_index in 0..input[0].len() {
        let mut visible_height = 0;
        for row_index in 0..input.len() {
            let position = input
                .get_mut(row_index)
                .unwrap()
                .get_mut(column_index)
                .unwrap();
            if position.tree_size >= visible_height {
                position.visible = true;
                visible_height = position.tree_size + 1;
            }

            if visible_height == 10 {
                break;
            }
        }
    }

    for column_index in 0..input[0].len() {
        let mut visible_height = 0;
        for row_index in (0..input.len()).rev() {
            let position = input
                .get_mut(row_index)
                .unwrap()
                .get_mut(column_index)
                .unwrap();
            if position.tree_size >= visible_height {
                position.visible = true;
                visible_height = position.tree_size + 1;
            }

            if visible_height == 10 {
                break;
            }
        }
    }

    // Horizontal, right to left,

    // Vertical, top to bottom

    // Vertical, bottom to top

    input
        .iter()
        .map(|x| x.iter().filter(|y| y.visible).count())
        .sum()
}

fn part_2(input: &Vec<Vec<Position>>) -> i32 {
    todo!()
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| Position {
                    tree_size: y.to_digit(10).unwrap(),
                    visible: false,
                })
                .collect()
        })
        .collect()
}

struct Position {
    tree_size: u32,
    visible: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let mut input = parse_input(TESTINPUT);
        assert_eq!(part_1(&mut input), 21);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
