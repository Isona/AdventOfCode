use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

use intcode::IntCodePC;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> u64 {
    // Take all the items we can take and move to the security check
    // https://cyberchef.org/#recipe=To_Decimal('Comma',false)&input=ZWFzdAp0YWtlIGxvb20KZWFzdAp0YWtlIGZpeGVkIHBvaW50Cm5vcnRoCnRha2Ugc3Bvb2wgb2YgY2F0Ngpub3J0aAp0YWtlIHdlYXRoZXIgbWFjaGluZQpzb3V0aAp3ZXN0CnRha2Ugc2hlbGwKZWFzdApzb3V0aAp3ZXN0CnNvdXRoCndlc3QKbm9ydGgKdGFrZSBjYW5keSBjYW5lCnNvdXRoCmVhc3QKdGFrZSBvcm5hbWVudApub3J0aAp3ZXN0Cm5vcnRoCnRha2Ugd3JlYXRoCm5vcnRoCmVhc3QK
    let input = [
        101, 97, 115, 116, 10, 116, 97, 107, 101, 32, 108, 111, 111, 109, 10, 101, 97, 115, 116,
        10, 116, 97, 107, 101, 32, 102, 105, 120, 101, 100, 32, 112, 111, 105, 110, 116, 10, 110,
        111, 114, 116, 104, 10, 116, 97, 107, 101, 32, 115, 112, 111, 111, 108, 32, 111, 102, 32,
        99, 97, 116, 54, 10, 110, 111, 114, 116, 104, 10, 116, 97, 107, 101, 32, 119, 101, 97, 116,
        104, 101, 114, 32, 109, 97, 99, 104, 105, 110, 101, 10, 115, 111, 117, 116, 104, 10, 119,
        101, 115, 116, 10, 116, 97, 107, 101, 32, 115, 104, 101, 108, 108, 10, 101, 97, 115, 116,
        10, 115, 111, 117, 116, 104, 10, 119, 101, 115, 116, 10, 115, 111, 117, 116, 104, 10, 119,
        101, 115, 116, 10, 110, 111, 114, 116, 104, 10, 116, 97, 107, 101, 32, 99, 97, 110, 100,
        121, 32, 99, 97, 110, 101, 10, 115, 111, 117, 116, 104, 10, 101, 97, 115, 116, 10, 116, 97,
        107, 101, 32, 111, 114, 110, 97, 109, 101, 110, 116, 10, 110, 111, 114, 116, 104, 10, 119,
        101, 115, 116, 10, 110, 111, 114, 116, 104, 10, 116, 97, 107, 101, 32, 119, 114, 101, 97,
        116, 104, 10, 110, 111, 114, 116, 104, 10, 101, 97, 115, 116, 10,
    ]
    .into();
    let (_, _) = pc.run_with_input(input);

    let items = [
        vec![111, 114, 110, 97, 109, 101, 110, 116], // ornament
        vec![108, 111, 111, 109],                    // loom
        vec![115, 112, 111, 111, 108, 32, 111, 102, 32, 99, 97, 116, 54], // spool of cat6
        vec![119, 114, 101, 97, 116, 104],           // wreath
        vec![102, 105, 120, 101, 100, 32, 112, 111, 105, 110, 116], // fixed point
        vec![115, 104, 101, 108, 108],               // shell
        vec![99, 97, 110, 100, 121, 32, 99, 97, 110, 101], // candy cane
        vec![
            119, 101, 97, 116, 104, 101, 114, 32, 109, 97, 99, 104, 105, 110, 101,
        ],
    ];
    let drop = vec![100, 114, 111, 112, 32];
    let take = vec![116, 97, 107, 101, 32];
    let south = VecDeque::from([115, 111, 117, 116, 104, 10]);

    let regex = Regex::new(r"typing (?<n>\d+)").unwrap();

    let take_items: Vec<VecDeque<i128>> = items
        .iter()
        .map(|x| {
            let mut command = VecDeque::from(take.clone());
            command.extend(x.iter());
            command.push_back(10);
            command
        })
        .collect();

    let drop_items: Vec<VecDeque<i128>> = items
        .iter()
        .map(|x| {
            let mut command = VecDeque::from(drop.clone());
            command.extend(x.iter());
            command.push_back(10);
            command
        })
        .collect();

    let input = drop_items.iter().flatten().cloned().collect();
    let (_, _) = pc.run_with_input(input);

    for indexes in (0..8).powerset() {
        let take_command = take_items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                if indexes.contains(&index) {
                    Some(item)
                } else {
                    None
                }
            })
            .flatten()
            .cloned()
            .collect();
        pc.run_with_input(take_command);

        let (_, output) = pc.run_with_input(south.clone());
        let output_string: String = output
            .iter()
            .map(|x| char::from_u32(*x as u32).unwrap())
            .collect();
        if !output_string.contains(r#"you are ejected back to the checkpoint."#) {
            let caps = regex.captures(&output_string).unwrap();
            return caps["n"].parse().unwrap();
        }

        let drop_command = drop_items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                if indexes.contains(&index) {
                    Some(item)
                } else {
                    None
                }
            })
            .flatten()
            .cloned()
            .collect();

        pc.run_with_input(drop_command);
    }

    panic!()
}
