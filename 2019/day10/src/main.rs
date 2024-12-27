use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, HashSet, VecDeque};

use aoc_lib::{Coordinate, Grid, Vector};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let (monitoring_station, part_1_answer) = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input, monitoring_station);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &Grid<char>) -> (Coordinate, usize) {
    input
        .find_all(&'#')
        .map(|coord| (coord, get_visible_count(input, coord)))
        .max_by_key(|x| x.1)
        .unwrap()
}

fn get_visible_count(input: &Grid<char>, monitoring_station: Coordinate) -> usize {
    // The total visible asteroids from a point is determined by counting the number of distinct angles to asteroids
    // We get the angles by using a polar coordinate system, so we do (other_meteor - monitoring_system)
    // Then we run atan2(y,x) on the result to get the angle, then add that angle to a hashset
    let mut angles = HashSet::new();
    for other_meteor in input.find_all(&'#') {
        let vector_between_meteors = Vector::get_difference(&monitoring_station, &other_meteor);
        angles.insert(OrderedFloat::from(f64::atan2(
            vector_between_meteors.y as f64,
            vector_between_meteors.x as f64,
        )));
    }

    angles.len()
}
fn part_2(input: &Grid<char>, monitoring_station: Coordinate) -> Coordinate {
    // Create a vec of asteroids with their angles and distances
    let mut asteroid_polars = Vec::new();

    for asteroid_coord in input.find_all(&'#') {
        let vector_between = Vector::get_difference(&monitoring_station, &asteroid_coord);
        // Angle is atan2(x, y) rather than atan2(y,x) as this causes sort.rev to work for starting at North
        let angle =
            OrderedFloat::from(f64::atan2(vector_between.x as f64, vector_between.y as f64));
        let distance = vector_between.x.abs() + vector_between.y.abs();
        let polar = AsteroidPolar {
            coord: asteroid_coord,
            distance,
            angle,
        };

        asteroid_polars.push(polar);
    }
    asteroid_polars.sort_by_key(|x| x.distance);

    // Create a btreemap of angles mapping to  a vecdeque of Coordinates
    let mut asteroid_tree: BTreeMap<OrderedFloat<f64>, VecDeque<Coordinate>> = BTreeMap::new();
    for asteroid in asteroid_polars {
        if let Some(asteroid_queue) = asteroid_tree.get_mut(&asteroid.angle) {
            asteroid_queue.push_back(asteroid.coord);
        } else {
            asteroid_tree.insert(asteroid.angle, VecDeque::from([asteroid.coord]));
        }
    }

    // Until we find the 200th asteroid, check through each of the angles in the btreemap in turn
    // Pop an asteroid from the front of each vecdeque in turn where possible
    let mut destroyed_asteroids = Vec::new();
    while destroyed_asteroids.len() <= 200 {
        for (_, asteroid_queue) in asteroid_tree.iter_mut().rev() {
            if let Some(destroyed_coord) = asteroid_queue.pop_front() {
                destroyed_asteroids.push(destroyed_coord);
            }
        }
    }

    destroyed_asteroids[199]
}

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }
    grid
}

#[derive(Eq, PartialEq, Debug)]
struct AsteroidPolar {
    coord: Coordinate,
    distance: i128,
    angle: OrderedFloat<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input).1, 8);
    }
    #[test]
    fn part_1_test_2() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_1(&input).1, 210);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        let (monitoring_station, _) = part_1(&input);
        println!("Monitoring station: {monitoring_station}");
        println!("Atan 0, -1: {}", f64::atan2(0.0, -1.0));
        assert_eq!(part_2(&input, monitoring_station), Coordinate::new(8, 2));
    }
}
