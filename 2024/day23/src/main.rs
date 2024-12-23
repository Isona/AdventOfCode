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
        let start_node_neighbours: HashSet<&str> = graph.neighbors(start_node).collect();
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
    let nodes = graph.nodes().map(|x| x.to_string()).collect();
    let max_graph = bron_kerbosch(graph, HashSet::new(), nodes, HashSet::new());
    let mut max_graph = max_graph.iter().collect_vec();
    max_graph.sort();
    max_graph.iter().join(",")
}

fn bron_kerbosch(
    graph: &UnGraphMap<&str, i32>,
    r: HashSet<String>,
    p: HashSet<String>,
    mut x: HashSet<String>,
) -> HashSet<String> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    let mut max_set = HashSet::new();
    let mut child_p = p.clone();

    for vertex in p {
        let vertex_neighbours: HashSet<String> =
            graph.neighbors(&vertex).map(|x| x.to_string()).collect();
        let vertex_hashset = HashSet::from([vertex.clone()]);
        let max_clique = bron_kerbosch(
            graph,
            r.union(&vertex_hashset).cloned().collect(),
            child_p.intersection(&vertex_neighbours).cloned().collect(),
            x.intersection(&vertex_neighbours).cloned().collect(),
        );

        if max_clique.len() > max_set.len() {
            max_set = max_clique;
        }
        child_p.remove(&vertex);
        x.insert(vertex);
    }

    max_set
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
