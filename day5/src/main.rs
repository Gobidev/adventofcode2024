use ahash::AHashSet;
use itertools::Itertools;

struct ParsedInput {
    rules: AHashSet<(u32, u32)>,
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

fn all_rules_apply(rules: &AHashSet<(u32, u32)>, page: &[u32]) -> bool {
    page.iter()
        .combinations(2)
        .all(|c| rules.get(&(*c[1], *c[0])).is_none())
}

fn part1(parsed_input: &ParsedInput) -> u32 {
    parsed_input
        .pages
        .iter()
        .filter(|page| all_rules_apply(&parsed_input.rules, page))
        .map(|rule| rule[rule.len() / 2])
        .sum()
}

fn part2(parsed_input: &mut ParsedInput) -> u32 {
    parsed_input
        .pages
        .iter_mut()
        .filter(|page| !all_rules_apply(&parsed_input.rules, page))
        .map(|page| {
            page.sort_by(|a, b| match !parsed_input.rules.contains(&(*a, *b)) {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            });
            page[page.len() / 2]
        })
        .sum()
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&mut input));
}
