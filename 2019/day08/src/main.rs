#![feature(iter_array_chunks)]

use aoc_lib::{Coordinate, Grid};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2 in {time_taken:.3} ms");
}

fn part_1(image: &[Grid<u32>]) -> usize {
    let mut fewest_zeroes_index = 0;
    let mut fewest_zeroes_count = usize::MAX;
    for (index, layer) in image.iter().enumerate() {
        let zero_count = layer.find_all(&0).count();
        if zero_count < fewest_zeroes_count {
            fewest_zeroes_index = index;
            fewest_zeroes_count = zero_count;
        }
    }

    let one_count = image[fewest_zeroes_index].find_all(&1).count();
    let two_count = image[fewest_zeroes_index].find_all(&2).count();
    one_count * two_count
}

fn part_2(image: &[Grid<u32>]) {
    let mut output_image = Grid::new();
    for _ in 0..6 {
        output_image.push_row(vec![' '; 25]);
    }

    for coord in (0..150).map(get_image_coord) {
        let pixel_value = image
            .iter()
            .map(|x| x.get(coord))
            .find(|x| **x != 2)
            .unwrap_or(&2);

        let pixel_value = match pixel_value {
            1 => '#',
            _ => ' ',
        };

        output_image.set(coord, pixel_value);
    }

    println!("{output_image}")
}

fn get_image_coord(index: usize) -> Coordinate {
    Coordinate::new(index % 25, index / 25)
}

fn parse_input(input: &str) -> Vec<Grid<u32>> {
    let mut image = Vec::new();
    for layer_chars in input.chars().array_chunks::<150>() {
        let mut layer = Grid::new();
        for row_chars in layer_chars.chunks(25) {
            let row = row_chars.iter().map(|x| x.to_digit(10).unwrap()).collect();
            layer.push_row(row);
        }
        image.push(layer);
    }

    image
}
