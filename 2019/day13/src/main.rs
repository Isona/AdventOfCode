use std::fmt::Display;

use aoc_lib::{Coordinate, Grid};
use intcode::IntCodePC;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    pc.reset_all();
    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> usize {
    pc.run_program();
    let mut output = pc.take_output();

    assert_eq!(output.len() % 3, 0);

    let mut grid = Grid::new();
    for _ in 0..25 {
        grid.push_row([TileType::Empty; 50].to_vec());
    }

    for output_triple in output.make_contiguous().chunks(3) {
        let coordinate = Coordinate::new(
            output_triple[0].try_into().unwrap(),
            output_triple[1].try_into().unwrap(),
        );
        let value = TileType::from(output_triple[2]);
        grid.set(coordinate, value);
    }

    // The paddle
    assert_eq!(pc.get_data(1629), Some(3));
    // The wall to the left
    assert_eq!(pc.get_data(1607), Some(1));
    // Check there's a wall before it (the wall at the right on the row above)
    assert_eq!(pc.get_data(1606), Some(1));
    // Wall to the right +21
    assert_eq!(pc.get_data(1650), Some(1));
    // Check that there's a wall next (the wall at the left on the row below)
    assert_eq!(pc.get_data(1651), Some(1));
    println!("{grid}");
    grid.find_all(&TileType::Block).count()
}

// Paddle - 22 is the start of the row (but is a wall)
// Paddle is at index 1629
fn part_2(pc: &mut IntCodePC) -> i128 {
    pc.set(0, 2);
    // That's a paddlin'
    for i in 1608..=1649 {
        pc.set(i, 3);
    }
    pc.set_input([0; 20000].into());
    pc.run_program();
    let mut output = pc.take_output();

    assert_eq!(output.len() % 3, 0);

    let mut grid = Grid::new();
    for _ in 0..25 {
        grid.push_row([TileType::Empty; 50].to_vec());
    }

    let mut score = 0;
    for output_triple in output.make_contiguous().chunks(3) {
        if output_triple[0] == -1 && output_triple[1] == 0 {
            score = output_triple[2];
            continue;
        }
        let coordinate = Coordinate::new(
            output_triple[0].try_into().unwrap(),
            output_triple[1].try_into().unwrap(),
        );
        let value = TileType::from(output_triple[2]);
        grid.set(coordinate, value);
    }

    println!("{grid}");
    score
}

#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
enum TileType {
    #[default]
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl From<i128> for TileType {
    fn from(value: i128) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!(),
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Empty => write!(f, " "),
            TileType::Wall => write!(f, "#"),
            TileType::Block => write!(f, "B"),
            TileType::Paddle => write!(f, "_"),
            TileType::Ball => write!(f, "O"),
        }
    }
}
