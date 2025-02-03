use std::collections::BTreeMap;
use std::fmt::Write;

use md5::{Digest, Md5};

const INPUT: &str = "uqwqemis";

fn main() {
    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1(INPUT);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &str) -> (String, String) {
    let mut output_1 = String::new();
    let mut output_2 = BTreeMap::new();
    let mut counter = 0;

    while output_2.len() < 8 || output_1.len() < 8 {
        let mut hasher = Md5::new();
        hasher.update(format!("{input}{counter}"));
        let result = hasher.finalize();
        if result[0..2] == [0, 0] && result[2] < 0x10 {
            if output_1.len() < 8 {
                output_1.push_str(&format!("{:x}", result[2]));
            }
            if result[2] < 8 {
                output_2.entry(result[2]).or_insert_with(|| result[3] >> 4);
            }
        }
        counter += 1;
    }
    let output_2 = output_2.values().fold(String::new(), |mut output, value| {
        let _ = write!(output, "{value:x}");
        output
    });

    (output_1, output_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "abc";

    #[test]
    fn part_1_test() {
        let (output_1, output_2) = part_1(TESTINPUT);
        assert_eq!(output_1, "18f47a30");
        assert_eq!(output_2, "05ace8e3");
    }
}
