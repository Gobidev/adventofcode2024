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

    fn reverse(&self) -> Self {
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
    let mut to_visit = BinaryHeap::new();
    let mut visited = AHashMap::new();
    let start_node = Node {
        score: 0,
        direction: East,
        position: *start_pos,
    };
    to_visit.push(Reverse(start_node));

    while let Some(curr_node) = to_visit.pop() {
        if visited
            .get(&(curr_node.0.position, curr_node.0.direction))
            .map_or(false, |s| s < &curr_node.0.score)
        {
            continue;
        }
        visited.insert(
            (curr_node.0.position, curr_node.0.direction),
            curr_node.0.score,
        );

        let pos_in_front = curr_node.0.position + curr_node.0.direction.as_vec();
        if grid[pos_in_front.x as usize][pos_in_front.y as usize] != Wall {
            to_visit.push(Reverse(Node {
                score: curr_node.0.score + 1,
                direction: curr_node.0.direction,
                position: curr_node.0.position + curr_node.0.direction.as_vec(),
            }));
        }

        to_visit.push(Reverse(Node {
            score: curr_node.0.score + 1000,
            direction: curr_node.0.direction.turn_left(),
            position: curr_node.0.position,
        }));

        to_visit.push(Reverse(Node {
            score: curr_node.0.score + 1000,
            direction: curr_node.0.direction.turn_right(),
            position: curr_node.0.position,
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

fn part2(scores: &AHashMap<(IVec2, Direction), u32>, end_pos: &IVec2) -> usize {
    let shortest_path_len = part1(scores, end_pos);
    let mut seen: AHashSet<IVec2> = AHashSet::new();
    let mut to_visit: Vec<_> = scores
        .iter()
        .filter(|((p, _), val)| val == &&shortest_path_len && p == end_pos)
        .map(|(x, y)| (*x, *y))
        .collect();

    while let Some(curr) = to_visit.pop() {
        // this is very ugly

        seen.insert(curr.0 .0);

        let pos_behind = curr.0 .0 + curr.0 .1.reverse().as_vec();
        if scores
            .get(&(pos_behind, curr.0 .1))
            .map_or(false, |v| v == &(curr.1 - 1))
        {
            to_visit.push(((pos_behind, curr.0 .1), (curr.1 - 1)));
        }
        if scores
            .get(&(curr.0 .0, curr.0 .1.turn_left()))
            .map_or(false, |v| v == &(curr.1 - 1000))
        {
            to_visit.push(((curr.0 .0, curr.0 .1.turn_left()), (curr.1 - 1000)));
        }
        if scores
            .get(&(curr.0 .0, curr.0 .1.turn_right()))
            .map_or(false, |v| v == &(curr.1 - 1000))
        {
            to_visit.push(((curr.0 .0, curr.0 .1.turn_right()), (curr.1 - 1000)));
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
