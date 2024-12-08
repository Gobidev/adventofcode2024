use std::collections::HashMap;

use bevy_math::IVec2;
use itertools::Itertools;
use num::Integer;

#[derive(Debug, Clone)]
struct ParsedInput {
    antennas: HashMap<char, Vec<IVec2>>,
    dimensions: IVec2,
}

fn parse(input: &str) -> ParsedInput {
    ParsedInput {
        antennas: input
            .lines()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_alphanumeric())
                    .map(move |(c_idx, c)| {
                        (
                            c,
                            IVec2 {
                                x: line_idx as i32,
                                y: c_idx as i32,
                            },
                        )
                    })
            })
            .into_group_map(),
        dimensions: IVec2 {
            x: input.lines().count() as i32,
            y: input.lines().next().unwrap().chars().count() as i32,
        },
    }
}

fn is_in_bounds(dimensions: &IVec2, elem: &IVec2) -> bool {
    elem.x >= 0 && elem.y >= 0 && elem.x < dimensions.x && elem.y < dimensions.y
}

fn part1(parsed_input: &ParsedInput) -> usize {
    parsed_input
        .antennas
        .values()
        .flat_map(|positions| positions.iter().permutations(2).map(|p| 2 * p[1] - *p[0]))
        .filter(|p| is_in_bounds(&parsed_input.dimensions, p))
        .unique()
        .count()
}

fn part2(parsed_input: &ParsedInput) -> usize {
    parsed_input
        .antennas
        .values()
        .flat_map(|positions| {
            positions.iter().permutations(2).flat_map(|p| {
                let diff = p[1] - p[0];
                let step = diff / diff.x.gcd(&diff.y);
                let mut curr = *p[0];
                let mut res = vec![];
                while is_in_bounds(&parsed_input.dimensions, &curr) {
                    res.push(curr);
                    curr += &step;
                }
                res
            })
        })
        .unique()
        .count()
}

fn main() {
    let input = parse(include_str!("../test_input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
