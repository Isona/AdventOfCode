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

fn part_1(pc: &mut IntCodePC) -> i128 {
    let mut one_count = 0;

    'yloop: for y in 0..50 {
        let mut x_found = false;
        for x in 0..50 {
            pc.reset_all();

            let value = *pc.run_with_input([x, y].into()).1.front().unwrap();
            if value == 1 {
                one_count += 1;
                if !x_found {
                    x_found = true;
                }
            } else if x_found {
                continue 'yloop;
            }
        }
    }

    one_count
}

// We want to find the corner of the right angle triangle
// Formed by (first_x(y+100), y+100), (first_x(y+100), y), (last_x, y)
// The goal is (first_x(y+100), y).
// We do this by finding the first and last x.
// When we find a first_x we check if the value at y-99 is equal to first_x + 99
// Note: it's 99 not 100 because fencepost counting
fn part_2(pc: &mut IntCodePC) -> i128 {
    let mut width;
    // Vecs for storing first and last x - indexed by y-100
    let mut first_x_vec: Vec<i128> = Vec::new();
    let mut last_x_vec: Vec<i128> = Vec::new();
    let mut current_y = 100;

    // Find the first and last x for y = 100
    {
        let mut x_found = false;
        let mut current_x = 0;
        'outer: loop {
            pc.reset_all();
            let value = *pc
                .run_with_input([current_x, current_y].into())
                .1
                .front()
                .unwrap();
            if value == 1 {
                if !x_found {
                    first_x_vec.push(current_x);
                    x_found = true;
                }
            } else if x_found {
                width = current_x - first_x_vec[0];
                last_x_vec.push(current_x - 1);
                break 'outer;
            }

            current_x += 1;
        }
    }

    'yloop: loop {
        current_y += 1;

        // Find the first x, we start our search at the previous first_x - 1
        // We do -1 to ensure we see at least 1 empty space first so we know it's definitely the first x
        let mut empty_found = false;
        let mut first_x = first_x_vec[(current_y - 1) as usize - 100] - 1;

        loop {
            pc.reset_all();
            let value = *pc
                .run_with_input([first_x, current_y].into())
                .1
                .front()
                .unwrap();
            if value == 0 && !empty_found {
                empty_found = true;
            } else if value == 1 && empty_found {
                first_x_vec.push(first_x);
                if current_y > 200 && first_x + 99 == last_x_vec[current_y as usize - 199] {
                    return first_x * 10000 + current_y - 99;
                }
                break;
            } else if value == 1 && !empty_found {
                panic!()
            }
            first_x += 1;
        }

        // Find the last x, start our search at first_x + width
        // Minus 2 because of how width varies
        let mut x_found = false;
        let mut last_x = first_x_vec[current_y as usize - 100] + width - 2;
        loop {
            pc.reset_all();
            let value = *pc
                .run_with_input([last_x, current_y].into())
                .1
                .front()
                .unwrap();

            if value == 1 && !x_found {
                x_found = true;
            } else if value == 0 && x_found {
                width = last_x - first_x_vec[current_y as usize - 100];
                last_x_vec.push(last_x - 1);
                continue 'yloop;
            } else if value == 0 && !x_found {
                panic!()
            }
            last_x += 1;
        }
    }
}
