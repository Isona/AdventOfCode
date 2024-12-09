const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(input: &[u32]) -> u64 {
    let mut total = 0;
    let total_fileblocks: u32 = input.iter().step_by(2).sum();

    let mut current_end_block_num = (input.len() / 2) as u64;
    let mut current_front_block_num = 0;

    let mut block_iter = input.iter();
    let mut rev_file_blocks = input.iter().step_by(2).rev();
    let mut is_file_block = true;

    let mut current_index_in_block = 0;
    let mut current_block_length = block_iter.next().unwrap();

    let mut current_index_in_end_block = 0;
    let mut current_end_block_length = rev_file_blocks.next().unwrap();

    // Loop through until we reach the end of the compressed file blocks
    for index in 0..total_fileblocks {
        // Check this first, some blocks are len 0
        // Keep checking until we get past any 0 len blocks
        while current_index_in_block >= *current_block_length {
            // Negate is_file_block
            is_file_block = !is_file_block;
            // If this is a new file block increment the number
            if is_file_block {
                current_front_block_num += 1;
            }
            // Get the next block length
            current_block_length = block_iter.next().unwrap();
            // Reset the current index in block
            current_index_in_block = 0;
            // Continue to do this check again for the next block
        }

        if is_file_block {
            print!("{current_front_block_num}");
            total += (current_front_block_num * index) as u64;
        } else {
            while current_index_in_end_block >= *current_end_block_length {
                current_end_block_num -= 1;
                current_end_block_length = rev_file_blocks.next().unwrap();
                current_index_in_end_block = 0;
            }
            print!("{current_end_block_num}");
            total += current_end_block_num * index as u64;
            current_index_in_end_block += 1;
        }

        current_index_in_block += 1;
    }

    println!();
    total
}

fn part_2(input: &[u32]) -> u64 {
    todo!()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 1928);
    }

    #[test]
    fn part_1_test_2() {
        let input_str = "12345";
        let input = parse_input(&input_str);
        assert_eq!(part_1(&input), 60)
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 2858);
    }

    #[test]
    fn part_2_test_2() {
        let input_str = "12345";
        let input = parse_input(&input_str);
        assert_eq!(part_2(&input), 72)
    }
}
