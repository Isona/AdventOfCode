const INPUT: &str = include_str!("input.txt");

fn main() {
    let (rules, reports) = parse_input(INPUT);

    let (part_1_answer, part_2_answer) = part_1(&rules, &reports);
    println!("Part 1: {part_1_answer}");
    println!("Part 2: {part_2_answer}");
}

fn part_1(rules: &Vec<Rule>, reports: &Vec<Vec<u64>>) -> (u64, u64) {
    let mut part_1_total = 0;
    let mut part_2_total = 0;
    for report in reports {
        let mut obeyed_rules = true;
        for rule in rules {
            if !rule.obeys_rule(report) {
                obeyed_rules = false;
                break;
            }
        }
        if obeyed_rules {
            part_1_total += get_middle_item(report);
        } else {
            part_2_total += get_middle_item(&reorder_report(rules, report));
        }
    }

    (part_1_total, part_2_total)
}

fn reorder_report(rules: &Vec<Rule>, report: &Vec<u64>) -> Vec<u64> {
    let mut reordered_report = report.clone();

    let mut ordered = false;
    while !ordered {
        ordered = true;
        for rule in rules {
            if !rule.obeys_rule(&reordered_report) {
                let index_1 = reordered_report
                    .iter()
                    .position(|x| x == &rule.lower)
                    .unwrap();
                let index_2 = reordered_report
                    .iter()
                    .position(|x| x == &rule.upper)
                    .unwrap();
                reordered_report.swap(index_1, index_2);
                ordered = false;
            }
        }
    }

    reordered_report
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u64>>) {
    let mut rules: Vec<Rule> = Vec::new();
    let mut reports: Vec<Vec<u64>> = Vec::new();

    let mut getting_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            getting_rules = false;
            continue;
        }

        if getting_rules {
            rules.push(Rule::new(line));
        } else {
            reports.push(line.split(',').map(|x| x.parse::<u64>().unwrap()).collect())
        }
    }

    (rules, reports)
}

fn get_middle_item(report: &[u64]) -> u64 {
    let middle_index = report.len() / 2;
    report[middle_index]
}

struct Rule {
    lower: u64,
    upper: u64,
}

impl Rule {
    fn new(input: &str) -> Self {
        let mut split = input.split('|');

        Rule {
            lower: split.next().unwrap().parse::<u64>().unwrap(),
            upper: split.next().unwrap().parse::<u64>().unwrap(),
        }
    }

    fn obeys_rule(&self, list: &[u64]) -> bool {
        if let Some(first_index) = list.iter().position(|x| x == &self.lower) {
            if let Some(second_index) = list.iter().position(|x| x == &self.upper) {
                if second_index < first_index {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let (rules, reports) = parse_input(TESTINPUT);
        let (part_1_result, _) = part_1(&rules, &reports);
        assert_eq!(part_1_result, 143);
    }

    #[test]
    fn part_2_test() {
        let (rules, reports) = parse_input(TESTINPUT);
        let (_, part_2_result) = part_1(&rules, &reports);
        assert_eq!(part_2_result, 123);
    }
}
