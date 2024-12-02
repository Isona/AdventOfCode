const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input = parse_input(INPUT);

    let part_1_answer = part_1(&input);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = part_2(&mut input);
    println!("Part 2: {part_2_answer}");
}

fn part_1(reports: &Vec<Vec<u64>>) -> usize {
    reports.iter().filter(|report: &&Vec<u64>| is_safe(*report)).count()
}

fn is_safe(report: &[u64]) -> bool {
    if !report.is_sorted() && !report.is_sorted_by(|a, b| a > b) {
        return false
    }
    for window in report.windows(2) {
        let diff = window[0].abs_diff(window[1]);
        if diff > 3 || diff < 1 {
            return false
        }
    }

    true
}

fn part_2(reports: &mut Vec<Vec<u64>>) -> u64 {
    let mut safetotal = 0;

    'outer: for report in reports { 
        if is_safe(report) {
            safetotal += 1;
            continue;
        }

        for index in 0..report.len() {
            let mut subreport = report.clone();
            subreport.remove(index);
            if is_safe(&subreport) {
                safetotal += 1;
                continue 'outer;
            }
        }
    }
    safetotal
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        reports.push(line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect());
    }
    
    reports
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = include_str!("testinput.txt");

    #[test]
    fn part_1_test() {
        let input = parse_input(TESTINPUT);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part_2_test() {
        let mut input = parse_input(TESTINPUT);
        assert_eq!(part_2(&mut input), 6);
    }
}
