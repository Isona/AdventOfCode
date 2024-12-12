use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (directions, nodes) = parse_input(INPUT);

    let part_1_answer = part_1(&directions, &nodes);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&directions, &nodes);
    println!("Part 2: {part_2_answer}");
}

fn part_1(directions: &str, nodes: &HashMap<String, (String, String)>) -> usize {
    let mut current_node = "AAA".to_string();
    let mut steps = 0;
    let directions: Vec<char> = directions.chars().collect();

    while current_node != *"ZZZ" {
        match directions[steps % directions.len()] {
            'L' => current_node = nodes.get(&current_node).unwrap().0.clone(),
            'R' => current_node = nodes.get(&current_node).unwrap().1.clone(),
            _ => panic!(),
        }

        steps += 1
    }

    steps
}

fn part_2(directions: &str, nodes: &HashMap<String, (String, String)>) -> u128 {
    let a_nodes: Vec<String> = nodes
        .keys()
        .filter_map(|x| {
            if x.ends_with('A') {
                Some(x.clone())
            } else {
                None
            }
        })
        .collect();

    let directions: Vec<char> = directions.chars().collect();

    println!("{:?}", a_nodes);

    // Get the loop length for each node
    let mut cycle_lengths: Vec<usize> = vec![];
    for a_node in a_nodes {
        let mut current_node = a_node;
        let mut steps = 0;
        while !current_node.ends_with('Z') {
            match directions[steps % directions.len()] {
                'L' => current_node = nodes.get(&current_node).unwrap().0.clone(),
                'R' => current_node = nodes.get(&current_node).unwrap().1.clone(),
                _ => panic!(),
            }

            steps += 1;
        }
        cycle_lengths.push(steps);
    }

    println!("{:?}", cycle_lengths);

    let mut lcm: u128 = 0;
    for cycle in cycle_lengths {
        if lcm == 0 {
            lcm = cycle.try_into().unwrap();
        } else {
            lcm = num::integer::lcm(lcm, cycle.try_into().unwrap());
        }
    }

    lcm
}

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().to_string();

    lines.next().unwrap();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let mut split = line
            .split([' ', '=', '(', ')', ','])
            .filter(|x| !x.is_empty());
        let node_name = split.next().unwrap().to_string();
        let left = split.next().unwrap().to_string();
        let right = split.next().unwrap().to_string();

        nodes.insert(node_name, (left, right));
    }

    (directions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (directions, nodes) = parse_input(TESTINPUT);
        assert_eq!(part_1(&directions, &nodes), 2);
    }

    #[test]
    fn part_2_test() {
        let (directions, nodes) = parse_input(TESTINPUT);
        assert_eq!(part_2(&directions, &nodes), 6);
    }
}
