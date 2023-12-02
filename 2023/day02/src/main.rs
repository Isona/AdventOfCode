const INPUT: &str = include_str!("input.txt");

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = parse_input(INPUT);

    let (part_1_ans, part_2_ans) = part_1_and_2(&input);
    println!("Part 1: {part_1_ans}");
    println!("Part 2: {part_2_ans}");
}

fn part_1_and_2(input: &Vec<Vec<&str>>) -> (i32, u32) {
    let mut game_id = 1;
    let mut part_1 = 0;
    let mut part_2 = 0;

    for game in input {
        let mut game_possible = true;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for play in game.chunks(2).skip(1) {
            let cubes = play[0].parse::<u32>().unwrap();

            match play[1] {
                "red" => {
                    if cubes > min_red {
                        min_red = cubes;
                    }

                    if cubes > MAX_RED {
                        game_possible = false;
                    }
                }
                "green" => {
                    if cubes > min_green {
                        min_green = cubes;
                    }
                    if cubes > MAX_GREEN {
                        game_possible = false;
                    }
                }
                "blue" => {
                    if cubes > min_blue {
                        min_blue = cubes;
                    }

                    if cubes > MAX_BLUE {
                        game_possible = false;
                    }
                }
                _ => {
                    panic!()
                }
            };
        }

        if game_possible {
            part_1 += game_id;
        }
        game_id += 1;

        let power = min_red * min_green * min_blue;
        part_2 += power;
    }
    (part_1, part_2)
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|x| {
            x.split(&[' ', ':', ';', ','])
                .filter(|y| !y.is_empty())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_and_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1_and_2(&input), (8, 2286));
    }
}
