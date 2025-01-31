use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (part_1_answer, part_2_answer) = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &[Room]) -> (usize, usize) {
    let mut summed_ids = 0;
    let mut part_2_found = false;
    let mut part_2_id = 0;

    for room in input {
        if room.is_valid() {
            summed_ids += room.sector_id;
            if !part_2_found && room.decrypt_name() == "northpole-object-storage" {
                part_2_found = true;
                part_2_id = room.sector_id;
            }
        }
    }

    (summed_ids, part_2_id)
}

fn parse_input(input: &str) -> Vec<Room> {
    input.lines().map(Room::new).collect()
}

struct Room {
    encrypted_name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    // totally-real-room-200[decoy]
    #[allow(clippy::needless_raw_string_hashes)]
    fn new(input: &str) -> Self {
        let regex =
            Regex::new(r#"(?<encrypted_name>.*)-(?<sector_id>\d+)\[(?<checksum>.*)\]"#).unwrap();
        let caps = regex.captures(input).unwrap();
        Self {
            encrypted_name: caps["encrypted_name"].to_string(),
            sector_id: caps["sector_id"].parse().unwrap(),
            checksum: caps["checksum"].to_string(),
        }
    }

    fn is_valid(&self) -> bool {
        let mut occurences = [(0, 0); 26];
        for i in 0..26 {
            let current_char = char::from(i + 97);
            occurences[i as usize] = (
                i as usize,
                self.encrypted_name.matches(current_char).count(),
            );
        }
        occurences.sort_by(|x, y| y.1.cmp(&x.1));

        let checksum_usizes: Vec<usize> = self
            .checksum
            .chars()
            .take(5)
            .map(|x| x as usize - 97)
            .collect();
        let correct_checksum: Vec<usize> = occurences[0..5].iter().map(|x| x.0).collect();
        checksum_usizes == correct_checksum
    }

    #[allow(clippy::cast_possible_truncation)]
    fn decrypt_name(&self) -> String {
        let shift = self.sector_id.rem_euclid(26) as u8;

        self.encrypted_name
            .chars()
            .map(|x| {
                if x.is_alphabetic() {
                    char::from((x as u8 - 97 + shift).rem_euclid(26) + 97)
                } else {
                    x
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).0, 1514);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input("qzmt-zixmtkozy-ivhz-343[12345]");
        assert_eq!(input[0].decrypt_name(), "very-encrypted-name");
    }
}
