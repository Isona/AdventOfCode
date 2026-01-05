use std::collections::{HashMap, HashSet, VecDeque};

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

fn part_1(input: &HashMap<String, Vec<String>>) -> u128 {
    bfs("you".to_string(), "out".to_string(), 1, input)
    // let mut visited = HashSet::new();
    // visited.insert("you".to_string());
    // let mut paths_to = HashMap::new();
    // paths_to.insert("you".to_string(), 1);
    // let mut queue = VecDeque::new();
    // queue.push_back("you".to_string());
    // while let Some(current_node) = queue.pop_front() {
    //     let Some(neighbours) = input.get(&current_node) else {
    //         continue;
    //     };
    //     let paths_to_current = *paths_to.get(&current_node).unwrap();

    //     for neighbour in neighbours {
    //         if !visited.contains(neighbour) {
    //             queue.push_back(neighbour.to_string());
    //             visited.insert(neighbour.clone());
    //         }

    //         paths_to
    //             .entry(neighbour.clone())
    //             .and_modify(|x| *x += paths_to_current)
    //             .or_insert(paths_to_current);
    //     }
    // }

    // *paths_to.get("out").unwrap()
}

// 3706171596504 is too low
fn part_2(graph: &HashMap<String, Vec<String>>) -> u128 {
    // Now going from svr to out
    // Want all paths visiting both dac and fft - but unsure which order

    // First find if dac can be reached from fft or if it's the other way
    let paths_from_dac_to_fft = bfs("dac".to_string(), "fft".to_string(), 1, graph);
    dbg!(paths_from_dac_to_fft);

    let (first_leg_goal, second_leg_goal, second_leg_total) = if paths_from_dac_to_fft != 0 {
        ("dac".to_string(), "fft".to_string(), paths_from_dac_to_fft)
    } else {
        (
            "fft".to_string(),
            "dac".to_string(),
            bfs("fft".to_string(), "dac".to_string(), 1, graph),
        )
    };

    let first_leg_total = bfs("svr".to_string(), first_leg_goal.clone(), 1, graph);
    let second_leg_total = bfs(
        first_leg_goal,
        second_leg_goal.clone(),
        first_leg_total,
        graph,
    );

    bfs(second_leg_goal, "out".to_string(), second_leg_total, graph)
}

fn bfs(
    start_node: String,
    end_node: String,
    paths_to_start: u128,
    graph: &HashMap<String, Vec<String>>,
) -> u128 {
    let mut visited = HashSet::new();
    visited.insert(start_node.to_string());
    let mut paths_to = HashMap::new();
    paths_to.insert(start_node.to_string(), paths_to_start);
    let mut queue = VecDeque::new();
    queue.push_back(start_node.to_string());

    while let Some(current_node) = queue.pop_front() {
        // if current_node == end_node {
        //     break;
        // }
        //let neighbours = graph.get(&current_node).unwrap();
        let Some(neighbours) = graph.get(&current_node) else {
            continue;
        };
        let paths_to_current = *paths_to.get(&current_node).unwrap();

        for neighbour in neighbours {
            if !visited.contains(neighbour) {
                queue.push_back(neighbour.to_string());
                visited.insert(neighbour.clone());
            }

            paths_to
                .entry(neighbour.clone())
                .and_modify(|x| *x += paths_to_current)
                .or_insert(paths_to_current);
        }
    }

    *paths_to.get(&end_node).unwrap_or(&0)
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let mut main_node = split.next().unwrap().to_string();
        main_node.pop();

        let connections = split.map(|x| x.to_string()).collect();

        nodes.insert(main_node, connections);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");
    const TESTINPUT2: &str = include_str!("testinput2.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(TESTINPUT2);
        assert_eq!(part_2(&input), 216);
    }

    // from https://www.reddit.com/r/adventofcode/comments/1poub2o/2025_day_11_part_2_stuck_on_part_2/
    #[test]
    fn bfs_test() {
        let input = "aaa: bbb ccc
ccc: ddd
ddd: eee
eee: bbb
bbb: fff";
        let graph = parse_input(input);
        assert_eq!(bfs("aaa".to_string(), "fff".to_string(), 1, &graph), 2);
    }
}
