use aoc_lib::{Coordinate, Direction, Grid};
use petgraph::{algo::dijkstra, prelude::DiGraphMap};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    let start = std::time::Instant::now();
    let part_1_answer = part_1(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 1: {part_1_answer} in {time_taken:.3} ms");

    let start = std::time::Instant::now();
    let part_2_answer = part_2(&input);

    let time_taken = start.elapsed().as_secs_f32() * 1000.0;
    println!("Part 2: {part_2_answer} in {time_taken:.3} ms");
}

fn part_1(input: &Grid<CellType>) -> i32 {
    let mut graph = DiGraphMap::new();

    let start = input.find_item(&CellType::Start).unwrap();
    add_coord_edges(input, &mut graph, &start);

    let end = input.find_item(&CellType::End).unwrap();
    add_coord_edges(input, &mut graph, &end);

    for coordinate in input.find_all(&CellType::Empty) {
        add_coord_edges(input, &mut graph, &coordinate);
    }

    let augmented_start = AugmentedCoord::new(&start, &Direction::East);
    let dijkstra = dijkstra(&graph, augmented_start, None, |x| *x.2);

    Direction::get_cardinals()
        .map(|direction| {
            dijkstra
                .get(&AugmentedCoord::new(&end, &direction))
                .unwrap()
        })
        .copied()
        .min()
        .unwrap()
}

fn add_coord_edges(
    input: &Grid<CellType>,
    graph: &mut DiGraphMap<AugmentedCoord, i32>,
    coordinate: &Coordinate,
) {
    for direction in Direction::get_cardinals() {
        let augmented = AugmentedCoord::new(&coordinate, &direction);
        let augmented_left = AugmentedCoord::new(&coordinate, &direction.turn_left());
        graph.add_edge(augmented, augmented_left, 1000);
        let augmented_right = AugmentedCoord::new(&coordinate, &direction.turn_right());
        graph.add_edge(augmented, augmented_right, 1000);

        if let Some(neighbour) = input.get_neighbour(*coordinate, direction) {
            if neighbour.value == &CellType::Wall {
                continue;
            }
            let augmented_neighbour = AugmentedCoord::new(&neighbour.location, &direction);
            graph.add_edge(augmented, augmented_neighbour, 1);
        }
    }
}

fn part_2(input: &Grid<CellType>) -> u64 {
    todo!();
}

fn parse_input(input: &str) -> Grid<CellType> {
    let mut grid = Grid::new();

    for line in input.lines() {
        grid.push_row(line.chars().map(CellType::from).collect());
    }

    grid
}

#[derive(Default, PartialEq, Eq)]
enum CellType {
    Wall,
    #[default]
    Empty,
    Start,
    End,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct AugmentedCoord {
    coordinate: Coordinate,
    direction: Direction,
}

impl AugmentedCoord {
    fn new(coordinate: &Coordinate, direction: &Direction) -> Self {
        AugmentedCoord {
            coordinate: *coordinate,
            direction: *direction,
        }
    }
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 7036);
    }

    #[test]
    fn part_1_test_2() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_1(&input), 11048);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), 5);
    }
}
