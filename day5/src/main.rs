struct ParsedInput {
    rules: Vec<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

fn parse(input: &str) -> ParsedInput {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    ParsedInput {
        rules: rules
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|(n1, n2)| (n1.parse().unwrap(), n2.parse().unwrap()))
            .collect(),
        pages: pages
            .lines()
            .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
            .collect(),
    }
}

fn all_rules_apply(rules: &[(u32, u32)], page: &[u32]) -> bool {
    rules.iter().all(|rule| {
        let Some(first) = page.iter().enumerate().find(|(_, e)| e == &&rule.0) else {
            return true;
        };
        let Some(second) = page.iter().enumerate().find(|(_, e)| e == &&rule.1) else {
            return true;
        };
        first.0 < second.0
    })
}

fn part1(parsed_input: &ParsedInput) -> u32 {
    parsed_input
        .pages
        .iter()
        .filter(|page| all_rules_apply(&parsed_input.rules, page))
        .map(|rule| rule[rule.len() / 2])
        .sum()
}

fn part2(parsed_input: &ParsedInput) -> u32 {
    parsed_input
        .pages
        .iter()
        .filter(|page| !all_rules_apply(&parsed_input.rules, page))
        .fold(vec![], |mut acc, curr| {
            let mut corrected_page = curr.clone();
            corrected_page.sort_unstable_by(|a, b| {
                match all_rules_apply(&parsed_input.rules, &[*a, *b]) {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                }
            });
            acc.push(corrected_page);
            acc
        })
        .iter()
        .map(|rule| rule[rule.len() / 2])
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
