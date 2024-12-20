use glam::IVec2;
use rayon::prelude::*;

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

    fn turn(&self) -> Self {
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Tile {
    tiletype: bool,
    distance: usize,
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
                    .map(|(c_idx, c)| Tile {
                        tiletype: match c {
                            '#' => true,
                            'S' => {
                                start_pos.x = l_idx as i32;
                                start_pos.y = c_idx as i32;
                                false
                            }
                            'E' => {
                                end_pos.x = l_idx as i32;
                                end_pos.y = c_idx as i32;
                                false
                            }
                            _ => false,
                        },
                        distance: usize::MAX,
                    })
                    .collect()
            })
            .collect(),
        start_pos,
        end_pos,
    )
}

fn get_distances(
    map: &mut [Vec<Tile>],
    pos: &IVec2,
    distance: &usize,
    path: &mut Vec<IVec2>,
    end_pos: &IVec2,
) {
    let curr_tile = &mut map[pos.x as usize][pos.y as usize];
    curr_tile.distance = *distance;
    if pos == end_pos {
        return;
    }
    for direction in Direction::all() {
        let new_tile_pos = pos + direction.to_vec();
        let new_tile = map[new_tile_pos.x as usize][new_tile_pos.y as usize];
        if !new_tile.tiletype && new_tile.distance > distance + 1 {
            path.push(new_tile_pos);
            get_distances(map, &new_tile_pos, &(distance + 1), path, end_pos);
            break;
        }
    }
}

fn part12(map: &[Vec<Tile>], path: &[IVec2], max_distance: i32) -> usize {
    let combinations: Vec<(i32, i32)> = (1..=max_distance)
        .flat_map(|i| (0..=(max_distance - i)).map(move |j| (i, j)))
        .collect();
    path.par_iter()
        .map(|tile| {
            Direction::all()
                .iter()
                .map(|direction| {
                    combinations
                        .iter()
                        .filter(|(i, j)| {
                            let new_tile_pos =
                                tile + i * direction.to_vec() + j * direction.turn().to_vec();
                            map.get(new_tile_pos.x as usize)
                                .and_then(|l| l.get(new_tile_pos.y as usize))
                                .map_or(false, |t| {
                                    t.distance
                                        >= map[tile.x as usize][tile.y as usize].distance
                                            + 100
                                            + *i as usize
                                            + *j as usize
                                        && !t.tiletype
                                })
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let (mut map, start_pos, end_pos) = parse(include_str!("../input.txt"));
    let mut path = vec![start_pos];
    get_distances(&mut map, &start_pos, &0, &mut path, &end_pos);
    println!("{}", part12(&map, &path, 2));
    println!("{}", part12(&map, &path, 20));
}
