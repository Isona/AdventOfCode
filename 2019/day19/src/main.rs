
use aoc_lib::{Coordinate, Grid};
use intcode::IntCodePC;
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut pc = IntCodePC::new(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut pc);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(pc: &mut IntCodePC) -> usize {
    let mut grid: Grid<i128> = Grid::new();

    for y in 0..50 {
        grid.push_row(vec![0;50]);
        for x  in 0..50 {
            let coord = Coordinate::new(x.try_into().unwrap(),y.try_into().unwrap());
            pc.reset_all();
            let value = *pc.run_with_input([x, y].into()).1.front().unwrap();
            grid.set(coord, value);
        }
    }

    println!("{grid}");
    
    grid.find_all(&1).count()
}

fn part_2(pc: &mut IntCodePC) -> u64 {
    'yloop: for y in 0..100 {
        let mut x_found = false;
        let mut width = 0;
        for x in 0..150 {
            pc.reset_all();
            let value = *pc.run_with_input([x, y].into()).1.front().unwrap();
            if value == 1{
                width += 1;
                if !x_found {
                    x_found = true;
                }
            }
            else if x_found {
                println!("{y}: {x}, width: {width}");
                continue 'yloop;
            }
        }
        println!("{y}: -");
    }

    todo!();
}