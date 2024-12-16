use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::{AHashMap, AHashSet};
use glam::IVec2;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}
use Tile::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}
use Direction::*;

impl Direction {
    fn as_vec(&self) -> IVec2 {
        match self {
            North => IVec2::new(-1, 0),
            South => IVec2::new(1, 0),
            West => IVec2::new(0, -1),
            East => IVec2::new(0, 1),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    fn turn_around(&self) -> Self {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Node {
    score: u32,
    direction: Direction,
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

fn parse(input: &str) -> (Vec<Vec<Tile>>, IVec2, IVec2) {
    let mut start_pos = IVec2::new(0, 0);
    let mut end_pos = IVec2::new(0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(l_idx, l)| {
                l.chars()
                    .enumerate()
                    .map(|(c_idx, c)| match c {
                        '#' => Wall,
                        'S' => {
                            start_pos.x = l_idx as i32;
                            start_pos.y = c_idx as i32;
                            Start
                        }
                        'E' => {
                            end_pos.x = l_idx as i32;
                            end_pos.y = c_idx as i32;
                            End
                        }
                        _ => Empty,
                    })
                    .collect()
            })
            .collect(),
        start_pos,
        end_pos,
    )
}

fn get_scores(grid: &[Vec<Tile>], start_pos: &IVec2) -> AHashMap<(IVec2, Direction), u32> {
    // dijkstra

    let mut to_visit = BinaryHeap::new();
    let mut visited = AHashMap::new();
    to_visit.push(Reverse(Node {
        score: 0,
        direction: East,
        position: *start_pos,
    }));

    while let Some(curr) = to_visit.pop() {
        let curr = curr.0;

        if visited
            .get(&(curr.position, curr.direction))
            .map_or(false, |s| s < &curr.score)
        {
            continue;
        }
        visited.insert((curr.position, curr.direction), curr.score);

        let pos_in_front = curr.position + curr.direction.as_vec();
        if grid[pos_in_front.x as usize][pos_in_front.y as usize] != Wall {
            to_visit.push(Reverse(Node {
                score: curr.score + 1,
                direction: curr.direction,
                position: curr.position + curr.direction.as_vec(),
            }));
        }

        to_visit.push(Reverse(Node {
            score: curr.score + 1000,
            direction: curr.direction.turn_left(),
            position: curr.position,
        }));

        to_visit.push(Reverse(Node {
            score: curr.score + 1000,
            direction: curr.direction.turn_right(),
            position: curr.position,
        }));
    }
    visited
}

fn part1(scores: &AHashMap<(IVec2, Direction), u32>, end_pos: &IVec2) -> u32 {
    *scores
        .iter()
        .filter(|((p, _), _)| p == end_pos)
        .map(|(_, v)| v)
        .min()
        .unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Visit {
    position: IVec2,
    direction: Direction,
    score: u32,
}

fn part2(scores: &AHashMap<(IVec2, Direction), u32>, end_pos: &IVec2) -> usize {
    // bfs from end to start along shortest paths

    let shortest_path_len = part1(scores, end_pos);
    let mut seen: AHashSet<IVec2> = AHashSet::new();
    let mut to_visit: Vec<Visit> = scores
        .iter()
        .filter(|((p, _), val)| val == &&shortest_path_len && p == end_pos)
        .map(|((p, d), s)| Visit {
            position: *p,
            direction: *d,
            score: *s,
        })
        .collect();

    while let Some(curr) = to_visit.pop() {
        seen.insert(curr.position);

        let pos_behind = curr.position + curr.direction.turn_around().as_vec();
        if scores
            .get(&(pos_behind, curr.direction))
            .map_or(false, |v| v == &(curr.score - 1))
        {
            to_visit.push(Visit {
                position: pos_behind,
                direction: curr.direction,
                score: curr.score - 1,
            });
        }

        let new_score = curr.score - 1000;
        if scores
            .get(&(curr.position, curr.direction.turn_left()))
            .map_or(false, |v| v == &new_score)
        {
            to_visit.push(Visit {
                position: curr.position,
                direction: curr.direction.turn_left(),
                score: new_score,
            });
        }
        if scores
            .get(&(curr.position, curr.direction.turn_right()))
            .map_or(false, |v| v == &new_score)
        {
            to_visit.push(Visit {
                position: curr.position,
                direction: curr.direction.turn_right(),
                score: new_score,
            });
        }
    }
    seen.len()
}

fn main() {
    let (grid, start_pos, end_pos) = parse(include_str!("../input.txt"));
    let scores = get_scores(&grid, &start_pos);
    println!("{}", part1(&scores, &end_pos));
    println!("{}", part2(&scores, &end_pos));
}
