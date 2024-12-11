const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);
    println!(
        "Part 1: {part_1_answer} in {:.3} ms",
        start.elapsed().as_secs_f32() * 1000.0
    );

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);
    println!(
        "Part 2: {part_2_answer} in {:.3} ms",
        start.elapsed().as_secs_f32() * 1000.0
    );
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
            //print!("{current_front_block_num}");
            total += (current_front_block_num * index) as u64;
        } else {
            while current_index_in_end_block >= *current_end_block_length {
                current_end_block_num -= 1;
                current_end_block_length = rev_file_blocks.next().unwrap();
                current_index_in_end_block = 0;
            }
            //print!("{current_end_block_num}");
            total += current_end_block_num * index as u64;
            current_index_in_end_block += 1;
        }

        current_index_in_block += 1;
    }

    //println!();
    total
}

fn part_2(input: &[u32]) -> u64 {
    let mut blocks = get_blocks(input);
    let mut back_index = blocks.len() - 1;

    if !blocks[back_index].is_file {
        back_index -= 1;
    }

    while blocks.get(back_index).is_some() {
        let mut front_index = 1;
        while front_index < back_index {
            if blocks[back_index].length <= blocks[front_index].length {
                blocks[back_index].start_index = blocks[front_index].start_index;
                blocks[front_index].start_index += blocks[back_index].length;
                blocks[front_index].length -= blocks[back_index].length;
                break;
            }
            front_index += 2;
        }
        if let Some(new_value) = back_index.checked_sub(2) {
            back_index = new_value;
        } else {
            break;
        }
    }

    blocks.iter().map(Block::get_checksum).sum()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn get_blocks(input_vec: &[u32]) -> Vec<Block> {
    let mut block_vec = Vec::new();
    let mut is_file = true;
    let mut start_index: usize = 0;
    let mut block_number = 0;
    for block in input_vec {
        block_vec.push(Block {
            is_file,
            start_index,
            length: *block as usize,
            block_number,
        });
        if is_file {
            block_number += 1
        }
        is_file = !is_file;
        start_index += *block as usize;
    }

    block_vec
}

#[derive(Debug)]
struct Block {
    is_file: bool,
    start_index: usize,
    length: usize,
    block_number: usize,
}

impl Block {
    fn get_checksum(&self) -> u64 {
        if !self.is_file {
            return 0;
        }

        let mut checksum = 0;
        for i in 0..self.length {
            checksum += (self.start_index + i) * self.block_number;
        }
        checksum.try_into().unwrap()
    }
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
        let input = parse_input(input_str);
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
        let input = parse_input(input_str);
        assert_eq!(part_2(&input), 132)
    }

    #[test]
    fn test_block_checksum() {
        let block = Block {
            is_file: true,
            start_index: 10,
            length: 5,
            block_number: 2,
        };
        assert_eq!(block.get_checksum(), 120);
    }
}
