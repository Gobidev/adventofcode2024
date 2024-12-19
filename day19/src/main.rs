#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}
use ahash::AHashMap;
use Color::*;

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => White,
            'u' => Blue,
            'b' => Black,
            'r' => Red,
            'g' => Green,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ParsedInput {
    available_patterns: Vec<Vec<Color>>,
    designs: Vec<Vec<Color>>,
}

fn parse(input: &str) -> ParsedInput {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    ParsedInput {
        available_patterns: patterns
            .trim_end()
            .split(", ")
            .map(|p| p.chars().map(|c| c.into()).collect())
            .collect(),
        designs: designs
            .lines()
            .map(|d| d.chars().map(|c| c.into()).collect())
            .collect(),
    }
}

fn count_possible(
    design: &[Color],
    available_patterns: &[Vec<Color>],
    cache: &mut AHashMap<Vec<Color>, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(res) = cache.get(design) {
        return *res;
    }
    let mut count = 0;
    for pattern in available_patterns {
        if pattern.len() > design.len() {
            continue;
        }
        let mut matches = true;
        for i in 0..pattern.len() {
            if design[i] != pattern[i] {
                matches = false;
                break;
            }
        }
        if matches {
            count += count_possible(&design[pattern.len()..], available_patterns, cache);
        }
    }
    cache.insert(design.to_vec(), count);
    count
}

fn part1(parsed_input: &ParsedInput) -> usize {
    let mut cache = AHashMap::new();
    parsed_input
        .designs
        .iter()
        .filter(|d| count_possible(d, &parsed_input.available_patterns, &mut cache) > 0)
        .count()
}

fn part2(parsed_input: &ParsedInput) -> usize {
    let mut cache = AHashMap::new();
    parsed_input
        .designs
        .iter()
        .map(|d| count_possible(d, &parsed_input.available_patterns, &mut cache))
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
