use std::fmt::Display;

use glam::IVec2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
use Direction::*;

impl Direction {
    fn to_vec(self) -> IVec2 {
        match self {
            Up => IVec2::new(-1, 0),
            Left => IVec2::new(0, -1),
            Down => IVec2::new(1, 0),
            Right => IVec2::new(0, 1),
        }
    }

    fn all() -> Vec<Direction> {
        vec![Up, Left, Down, Right]
    }

    fn orthogonal(&self) -> Vec<Self> {
        match self {
            Up | Down => vec![Left, Right],
            Left | Right => vec![Up, Down],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum TileType {
    Free,
    Wall,
    Start,
    End,
}
use TileType::*;

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Free => write!(f, "."),
            Wall => write!(f, "#"),
            Start => write!(f, "S"),
            End => write!(f, "E"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Tile {
    tiletype: TileType,
    distance: usize,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.distance < usize::MAX {
            write!(f, ",")
        } else {
            write!(f, "{}", self.tiletype)
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, IVec2) {
    let mut start_pos = IVec2::new(0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(l_idx, l)| {
                l.chars()
                    .enumerate()
                    .map(|(c_idx, c)| Tile {
                        tiletype: match c {
                            '#' => Wall,
                            'S' => {
                                start_pos.x = l_idx as i32;
                                start_pos.y = c_idx as i32;
                                Start
                            }
                            'E' => End,
                            _ => Free,
                        },
                        distance: usize::MAX,
                    })
                    .collect()
            })
            .collect(),
        start_pos,
    )
}

fn get_distances(map: &mut [Vec<Tile>], pos: &IVec2, distance: &usize, path: &mut Vec<IVec2>) {
    let curr_tile = &mut map[pos.x as usize][pos.y as usize];
    curr_tile.distance = *distance;
    if curr_tile.tiletype == End {
        return;
    }
    for direction in Direction::all() {
        let new_tile_pos = pos + direction.to_vec();
        let new_tile = map[new_tile_pos.x as usize][new_tile_pos.y as usize];
        if new_tile.tiletype != Wall && new_tile.distance > distance + 1 {
            path.push(new_tile_pos);
            get_distances(map, &new_tile_pos, &(distance + 1), path);
        }
    }
}

fn part12(map: &[Vec<Tile>], path: &[IVec2], max_distance: i32) -> usize {
    let mut res = 0;
    for tile in path {
        let mut cheated_targets = vec![];
        for direction in Direction::all() {
            for i in 2..=max_distance {
                for j in 0..=(max_distance - i) {
                    for other_direction in direction.orthogonal() {
                        let new_tile_pos =
                            tile + i * direction.to_vec() + j * other_direction.to_vec();
                        let Some(Some(t)) = map
                            .get(new_tile_pos.x as usize)
                            .map(|l| l.get(new_tile_pos.y as usize))
                        else {
                            continue;
                        };
                        if t.tiletype == Wall || cheated_targets.contains(&new_tile_pos) {
                            continue;
                        }
                        if t.distance
                            >= map[tile.x as usize][tile.y as usize].distance
                                + 100
                                + i as usize
                                + j as usize
                        {
                            cheated_targets.push(new_tile_pos);
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let (mut map, start_pos) = parse(include_str!("../input.txt"));
    let mut path = vec![start_pos];
    get_distances(&mut map, &start_pos, &0, &mut path);
    println!("{}", part12(&map, &path, 2));
    println!("{}", part12(&map, &path, 20));
}
