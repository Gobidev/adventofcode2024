use std::{cmp::Reverse, collections::BinaryHeap};

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
enum Tile {
    Corrupted,
    Safe,
}
use Tile::*;

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

fn build_grid(dimensions: &IVec2, bytes_amount: usize, positions: &[IVec2]) -> Vec<Vec<Tile>> {
    let mut grid = vec![];
    for px in 0..=dimensions.x {
        let mut line = vec![];
        for py in 0..=dimensions.y {
            if positions[..bytes_amount].contains(&IVec2::new(px, py)) {
                line.push(Corrupted);
            } else {
                line.push(Safe);
            }
        }
        grid.push(line);
    }
    grid
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

fn find_path(grid: &[Vec<Tile>], start_pos: &IVec2, end_pos: &IVec2) -> u32 {
    let mut distances = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut to_visit = BinaryHeap::new();

    distances[start_pos.x as usize][start_pos.y as usize] = 0;
    to_visit.push(Reverse(Node {
        score: 0,
        position: *start_pos,
    }));

    while let Some(Reverse(curr)) = to_visit.pop() {
        if curr.position == *end_pos {
            return curr.score;
        }

        if curr.score > distances[curr.position.x as usize][curr.position.y as usize] {
            continue;
        }

        for direction in Direction::all() {
            let new_pos = curr.position + direction.to_vec();
            let Some(Some(t)) = grid
                .get(new_pos.x as usize)
                .map(|l| l.get(new_pos.y as usize))
            else {
                continue;
            };
            if *t != Corrupted {
                let new_score = curr.score + 1;
                if new_score < distances[new_pos.x as usize][new_pos.y as usize] {
                    distances[new_pos.x as usize][new_pos.y as usize] = new_score;
                    to_visit.push(Reverse(Node {
                        score: new_score,
                        position: new_pos,
                    }));
                }
            }
        }
    }

    u32::MAX
}

fn part1(positions: &[IVec2], dimensions: &IVec2) -> u32 {
    let grid = build_grid(dimensions, 1024, positions);
    find_path(&grid, &IVec2::new(0, 0), dimensions)
}

fn part2(positions: &[IVec2], dimensions: &IVec2) -> String {
    let start_pos = IVec2::new(0, 0);
    // todo: use binary search
    for i in 1024.. {
        let grid = build_grid(dimensions, i, positions);
        if find_path(&grid, &start_pos, dimensions) == u32::MAX {
            return positions
                .get(i - 1)
                .map(|p| format!("{},{}", p.y, p.x))
                .unwrap();
        }
    }
    unreachable!()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    let size = IVec2::new(70, 70);
    println!("{}", part1(&input, &size));
    println!("{}", part2(&input, &size));
}
