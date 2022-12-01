const INPUT: &str = include_str!("input.txt");

fn main() {
    let part_1_answer = part_1(INPUT);
    println!("{part_1_answer}");

    let part_2_answer = part_2(INPUT);
    println!("{part_2_answer}");
}

fn part_1(input: &str) -> i32 {
    let mut max_elf_sum = 0;
    let mut elf_sum = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            max_elf_sum = elf_sum.max(max_elf_sum);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }

    // Don't miss the last elf
    max_elf_sum = elf_sum.max(max_elf_sum);

    max_elf_sum
}

fn part_2(input: &str) -> i32 {
    let mut elf_sum = 0;
    let mut elf_sums: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            elf_sums.push(elf_sum);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }

    // Don't miss the last elf
    elf_sums.push(elf_sum);

    elf_sums.sort_by(|a, b| b.cmp(a));
    elf_sums[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TESTINPUT), 24000);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TESTINPUT), 45000);
    }
}
