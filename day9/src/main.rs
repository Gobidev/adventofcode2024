use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Space {
    id: Option<usize>,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if let Some(i) = self.id {
                i.to_string()
            } else {
                ".".to_string()
            }
        )
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.id.is_none() {
            return Ordering::Less;
        }
        if other.id.is_none() {
            return Ordering::Greater;
        }
        self.id.unwrap().cmp(&other.id.unwrap())
    }
}

fn parse(input: &str) -> Vec<Space> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .flat_map(|(c_idx, c)| {
            (0..c.to_digit(10).unwrap()).map(move |_| Space {
                id: if c_idx % 2 == 0 {
                    Some(c_idx / 2)
                } else {
                    None
                },
            })
        })
        .collect()
}

fn checksum(filesystem: &[Space]) -> usize {
    filesystem
        .iter()
        .enumerate()
        .map(|(c_idx, c)| if let Some(v) = c.id { c_idx * v } else { 0 })
        .sum()
}

fn part1(filesystem: &mut [Space]) -> usize {
    for space_idx in 0..filesystem.len() {
        if filesystem[space_idx].id.is_none() {
            let last = filesystem
                .iter()
                .enumerate()
                .rev()
                .find(|(_, n)| n.id.is_some())
                .unwrap();
            if last.0 < space_idx {
                break;
            }
            filesystem.swap(space_idx, last.0);
        };
    }
    checksum(filesystem)
}

fn get_chunk_size(filesystem: &[Space], idx: usize) -> usize {
    let mut chunk_size = 0;
    while idx + chunk_size < filesystem.len() && filesystem[idx + chunk_size] == filesystem[idx] {
        chunk_size += 1;
    }
    chunk_size
}

fn part2(filesystem: &mut [Space]) -> usize {
    for move_id in (0..=filesystem.iter().max().unwrap().id.unwrap()).rev() {
        let move_idx = filesystem
            .iter()
            .enumerate()
            .find(|(_, c)| c.id.unwrap_or(move_id + 1) == move_id)
            .unwrap()
            .0;
        let move_chunk_size = get_chunk_size(filesystem, move_idx);
        for free_idx in 0..move_idx {
            if filesystem[free_idx].id.is_some() {
                continue;
            }
            if get_chunk_size(filesystem, free_idx) >= move_chunk_size {
                for i in 0..move_chunk_size {
                    filesystem.swap(move_idx + i, free_idx + i);
                }
                break;
            }
        }
    }
    checksum(filesystem)
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input.clone()));
    println!("{}", part2(&mut input));
}
