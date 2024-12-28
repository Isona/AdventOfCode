use regex::Regex;
use rustc_hash::FxHashMap as HashMap;
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = do_n_iterations(&input, 1000);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&mut input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_2(moons: &mut [Moon]) -> i64 {
    let mut x_states: HashMap<Vec<i64>, i64> = HashMap::default();
    let mut x_repeat = false;
    let mut x_repeat_time = 0;

    let mut y_states: HashMap<Vec<i64>, i64> = HashMap::default();
    let mut y_repeat = false;
    let mut y_repeat_time = 0;

    let mut z_states: HashMap<Vec<i64>, i64> = HashMap::default();
    let mut z_repeat = false;
    let mut z_repeat_time = 0;

    let mut current_cycle = 0;

    while !x_repeat || !y_repeat || !z_repeat {
        if !x_repeat {
            let x_state = get_x_state(moons);
            if x_states.contains_key(&x_state) {
                x_repeat = true;
                x_repeat_time = current_cycle;
            } else {
                x_states.insert(x_state, current_cycle);
            }
        }

        if !y_repeat {
            let y_state = get_y_state(moons);
            if y_states.contains_key(&y_state) {
                y_repeat = true;
                y_repeat_time = current_cycle;
            } else {
                y_states.insert(y_state, current_cycle);
            }
        }

        if !z_repeat {
            let z_state = get_z_state(moons);
            if z_states.contains_key(&z_state) {
                z_repeat = true;
                z_repeat_time = current_cycle;
            } else {
                z_states.insert(z_state, current_cycle);
            }
        }

        do_iteration(moons);
        current_cycle += 1;
    }

    println!("{x_repeat_time}, {y_repeat_time}, {z_repeat_time}");
    num::integer::lcm(
        num::integer::lcm(x_repeat_time, y_repeat_time),
        z_repeat_time,
    )
}

fn do_n_iterations(input: &[Moon], iter_count: usize) -> i64 {
    let mut moons: Vec<Moon> = input.iter().cloned().collect();
    for _ in 0..iter_count {
        do_iteration(&mut moons);
    }

    moons.iter().map(|moon| moon.get_score()).sum()
}

fn do_iteration(input: &mut [Moon]) {
    for current_moon_index in 0..input.len() {
        for other_moon_index in (0..input.len()).filter(|index| index != &current_moon_index) {
            let other_moon = input.get(other_moon_index).unwrap().clone();
            let current_moon = input.get_mut(current_moon_index).unwrap();
            current_moon.update_velocity(&other_moon);
        }
    }

    for moon in input.iter_mut() {
        moon.update_position();
    }
}

fn get_x_state(moons: &[Moon]) -> Vec<i64> {
    let mut x_state = Vec::new();
    for moon in moons {
        x_state.push(moon.pos_x);
        x_state.push(moon.vel_x);
    }

    x_state
}

fn get_y_state(moons: &[Moon]) -> Vec<i64> {
    let mut y_state = Vec::new();
    for moon in moons {
        y_state.push(moon.pos_y);
        y_state.push(moon.vel_y);
    }

    y_state
}

fn get_z_state(moons: &[Moon]) -> Vec<i64> {
    let mut z_state = Vec::new();
    for moon in moons {
        z_state.push(moon.pos_z);
        z_state.push(moon.vel_z);
    }

    z_state
}

fn parse_input(input: &str) -> Vec<Moon> {
    let mut moons = Vec::new();
    let regex = Regex::new(r"x=(?<x>-?\d+), y=(?<y>-?\d+), z=(?<z>-?\d+)").unwrap();
    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        moons.push(Moon::new(
            caps["x"].parse().unwrap(),
            caps["y"].parse().unwrap(),
            caps["z"].parse().unwrap(),
        ));
    }

    moons
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Moon {
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
}

impl Moon {
    fn new(pos_x: i64, pos_y: i64, pos_z: i64) -> Self {
        Moon {
            pos_x,
            pos_y,
            pos_z,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        }
    }

    fn update_velocity(&mut self, other: &Self) {
        if self.pos_x < other.pos_x {
            self.vel_x += 1;
        } else if self.pos_x > other.pos_x {
            self.vel_x -= 1;
        }
        if self.pos_y < other.pos_y {
            self.vel_y += 1;
        } else if self.pos_y > other.pos_y {
            self.vel_y -= 1;
        }
        if self.pos_z < other.pos_z {
            self.vel_z += 1;
        } else if self.pos_z > other.pos_z {
            self.vel_z -= 1;
        }
    }

    fn update_position(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.pos_z += self.vel_z;
    }

    fn get_score(&self) -> i64 {
        let potential_energy = self.pos_x.abs() + self.pos_y.abs() + self.pos_z.abs();
        let kinetic_energy = self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs();

        potential_energy * kinetic_energy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn parse_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(input.len(), 4);
        assert_eq!(input[0], Moon::new(-1, 0, 2));
    }

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(do_n_iterations(&input, 10), 179);
    }

    #[test]
    fn part_2_test() {
        let mut input = parse_input(TESTINPUT);
        assert_eq!(part_2(&mut input), 2772);
    }
}
