use std::{collections::VecDeque, fmt::Display};

use glam::IVec2;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Robot,
    Box,
    Empty,
}
use Tile::*;

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wall => write!(f, "#"),
            Robot => write!(f, "@"),
            Box => write!(f, "O"),
            Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn as_vec(&self) -> IVec2 {
        match self {
            Up => IVec2::new(-1, 0),
            Down => IVec2::new(1, 0),
            Left => IVec2::new(0, -1),
            Right => IVec2::new(0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    map: Vec<Vec<Tile>>,
    directions: VecDeque<Direction>,
    robot_pos: IVec2,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|l| l.iter().map(|t| t.to_string()).collect::<String>() + "\n")
                .collect::<String>()
        )
    }
}

fn parse(input: &str) -> State {
    let (map, directions) = input.split_once("\n\n").unwrap();
    let mut robot_pos = IVec2::new(0, 0);
    State {
        map: map
            .lines()
            .enumerate()
            .map(|(l_idx, l)| {
                l.chars()
                    .enumerate()
                    .map(|(c_idx, c)| match c {
                        '#' => Wall,
                        '@' => {
                            robot_pos.x = l_idx as i32;
                            robot_pos.y = c_idx as i32;
                            Robot
                        }
                        'O' => Box,
                        _ => Empty,
                    })
                    .collect()
            })
            .collect(),
        directions: directions
            .replace("\n", "")
            .chars()
            .map(|c| match c {
                '<' => Left,
                '^' => Up,
                '>' => Right,
                'v' => Down,
                _ => panic!(),
            })
            .collect(),
        robot_pos,
    }
}

#[inline]
fn get_tile<'a>(map: &'a mut [Vec<Tile>], idx: &IVec2) -> &'a mut Tile {
    &mut map[idx.x as usize][idx.y as usize]
}

fn do_move(state: &mut State) {
    let dir = state.directions.pop_front().unwrap().as_vec();
    let pos_in_front = state.robot_pos + dir;

    if get_tile(&mut state.map, &pos_in_front) == &Wall {
        return;
    }

    if get_tile(&mut state.map, &pos_in_front) == &Empty {
        *get_tile(&mut state.map, &pos_in_front) = Robot;
        *get_tile(&mut state.map, &state.robot_pos) = Empty;
        state.robot_pos = pos_in_front;
        return;
    }

    let mut curr = pos_in_front;
    while get_tile(&mut state.map, &curr) != &Wall && get_tile(&mut state.map, &curr) != &Empty {
        curr += dir;
    }
    if get_tile(&mut state.map, &curr) == &Wall {
        return;
    }
    *get_tile(&mut state.map, &curr) = Box;
    *get_tile(&mut state.map, &pos_in_front) = Robot;
    *get_tile(&mut state.map, &state.robot_pos) = Empty;
    state.robot_pos = pos_in_front;
}

fn part1(state: &mut State) -> usize {
    while !state.directions.is_empty() {
        do_move(state);
    }
    state
        .map
        .iter()
        .enumerate()
        .flat_map(|(l_idx, l)| {
            l.iter().enumerate().map(
                move |(t_idx, t)| {
                    if t == &Box {
                        100 * l_idx + t_idx
                    } else {
                        0
                    }
                },
            )
        })
        .sum()
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input));
}
