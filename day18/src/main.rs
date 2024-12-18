use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::AHashSet;
use glam::IVec2;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn to_vec(&self) -> IVec2 {
        match self {
            Up => IVec2::new(-1, 0),
            Down => IVec2::new(1, 0),
            Left => IVec2::new(0, -1),
            Right => IVec2::new(0, 1),
        }
    }
    fn all() -> Vec<Self> {
        vec![Up, Down, Left, Right]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    score: u32,
    position: IVec2,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Distance = (u32, Vec<IVec2>);

fn parse(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|l| {
            let (y, x) = l.split_once(',').unwrap();
            IVec2 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

fn get_score<'a>(scores: &'a mut [Vec<Distance>], pos: &IVec2) -> &'a mut Distance {
    &mut scores[pos.x as usize][pos.y as usize]
}

fn find_path(corrupt_pos: &AHashSet<IVec2>, dimensions: &IVec2) -> Distance {
    let mut scores =
        vec![vec![(u32::MAX, vec![]); dimensions.y as usize + 1]; dimensions.x as usize + 1];
    let mut to_visit = BinaryHeap::new();

    let start_pos = IVec2::new(0, 0);

    get_score(&mut scores, &start_pos).0 = 0;
    to_visit.push(Reverse(Node {
        score: 0,
        position: start_pos,
    }));

    while let Some(Reverse(curr)) = to_visit.pop() {
        if curr.position == *dimensions {
            return (curr.score, get_score(&mut scores, &curr.position).1.clone());
        }

        if curr.score > get_score(&mut scores, &curr.position).0 {
            continue;
        }

        for direction in Direction::all() {
            let new_pos = curr.position + direction.to_vec();
            if new_pos.cmplt(start_pos).any()
                || new_pos.cmpgt(*dimensions).any()
                || corrupt_pos.contains(&new_pos)
            {
                continue;
            }

            let new_score = curr.score + 1;
            if new_score < get_score(&mut scores, &new_pos).0 {
                get_score(&mut scores, &new_pos).0 = new_score;

                let mut prev_positions = get_score(&mut scores, &curr.position).1.clone();
                prev_positions.push(curr.position);
                get_score(&mut scores, &new_pos).1.extend(prev_positions);

                to_visit.push(Reverse(Node {
                    score: new_score,
                    position: new_pos,
                }));
            }
        }
    }

    (u32::MAX, vec![])
}

fn part1(positions: &[IVec2], dimensions: &IVec2) -> u32 {
    find_path(
        &AHashSet::from_iter(positions[..1024].iter().map(|i| i.to_owned())),
        dimensions,
    )
    .0
}

fn part2(positions: &[IVec2], dimensions: &IVec2) -> String {
    let mut prev_path = AHashSet::new();
    for i in 1024..positions.len() {
        if !prev_path.is_empty() && !prev_path.contains(&positions[i - 1]) {
            continue;
        }
        let (len, path) = find_path(
            &AHashSet::from_iter(positions[..i].iter().map(|i| i.to_owned())),
            dimensions,
        );
        prev_path = AHashSet::from_iter(path);
        if len == u32::MAX {
            return positions
                .get(i - 1)
                .map(|p| format!("{},{}", p.y, p.x))
                .unwrap();
        }
    }
    panic!("not found")
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    let size = IVec2::new(70, 70);
    println!("{}", part1(&input, &size));
    println!("{}", part2(&input, &size));
}
