fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[u32]) -> bool {
    let pairs: Vec<(u32, u32)> = report
        .windows(2)
        .map(|w| (*w.first().unwrap(), *w.get(1).unwrap()))
        .collect();
    pairs.iter().all(|(x, y)| x < y && x + 3 >= *y)
        || pairs.iter().all(|(x, y)| x > y && *x <= y + 3)
}

fn part1(parsed_input: &[Vec<u32>]) -> u32 {
    parsed_input.iter().filter(|report| is_safe(report)).count() as u32
}

fn part2(parsed_input: &[Vec<u32>]) -> u32 {
    parsed_input
        .iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|idx| {
                    let mut modified_report = (*report).clone();
                    modified_report.remove(idx);
                    is_safe(&modified_report)
                })
        })
        .count() as u32
}

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input = parse(input);
    println!("{}", part1(&parsed_input));
    println!("{}", part2(&parsed_input));
}
