use std::collections::HashSet;

use itertools::Itertools;
use petgraph::prelude::UnGraphMap;

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

fn part_1(graph: &UnGraphMap<&str, i32>) -> usize {
    let mut cycles: HashSet<Vec<&str>> = HashSet::new();

    // Find cycles!
    // Get all the nodes in the graph starting with `t``
    for start_node in graph.nodes().filter(|pc_name| pc_name.starts_with('t')) {
        // Iterate over its neighbours
        let start_node_neighbours = graph.neighbors(start_node).collect_vec();
        for second_node in graph.neighbors(start_node) {
            let second_node_neighbours = graph.neighbors(second_node);
            // Find possible third nodes by getting the intersection of the neighbours of nodes 1 and 2
            let third_nodes =
                second_node_neighbours.filter(|node| start_node_neighbours.contains(node));

            // Add each node to the cycles hashset, but sort them first
            for third_node in third_nodes {
                let mut cycle = vec![start_node, second_node, third_node];
                cycle.sort();
                cycles.insert(cycle);
            }
        }
    }

    cycles.len()
}

fn part_2(graph: &UnGraphMap<&str, i32>) -> String {
    let mut max_graph = Vec::new();

    for node in graph.nodes() {
        let neighbours = graph.neighbors(node).collect_vec();
        let candidate_graph = find_fully_connected(graph, vec![node], neighbours);
        if candidate_graph.len() > max_graph.len() {
            max_graph = candidate_graph;
        }
    }

    max_graph.sort();
    max_graph.iter().join(",")
}

fn find_fully_connected<'a>(
    graph: &'a UnGraphMap<&'a str, i32>,
    nodes: Vec<&'a str>,
    neighbours_intersection: Vec<&'a str>,
) -> Vec<&'a str> {
    if neighbours_intersection.is_empty() {
        return nodes.to_vec();
    }

    let mut largest_graph = Vec::new();
    for candidate_node in &neighbours_intersection {
        let candidate_intersection = graph
            .neighbors(candidate_node)
            .filter(|node| neighbours_intersection.contains(node))
            .collect_vec();
        let mut candidate_nodes = nodes.clone();
        candidate_nodes.push(candidate_node);

        let candidate_graph = find_fully_connected(graph, candidate_nodes, candidate_intersection);
        if candidate_graph.len() > largest_graph.len() {
            largest_graph = candidate_graph;
        }
    }

    largest_graph
}

fn parse_input(input: &str) -> UnGraphMap<&str, i32> {
    let mut graph = UnGraphMap::new();
    for line in input.lines() {
        let mut split = line.split('-');
        graph.add_edge(split.next().unwrap(), split.next().unwrap(), 1);
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_2(&input), "co,de,ka,ta");
    }
}
