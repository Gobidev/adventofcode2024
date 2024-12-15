use std::{collections::VecDeque, fmt::Display};

use glam::IVec2;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Tile {
    Wall,
    Robot,
    Box,
    BoxL,
    BoxR,
    Empty,
}
use Tile::*;

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wall => write!(f, "#"),
            Robot => write!(f, "@"),
            Box => write!(f, "O"),
            BoxL => write!(f, "["),
            BoxR => write!(f, "]"),
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

impl State {
    fn widen(&mut self) {
        self.map = self
            .map
            .iter()
            .map(|l| {
                l.iter()
                    .flat_map(|t| match t {
                        Empty => [Empty, Empty],
                        Wall => [Wall, Wall],
                        Box => [BoxL, BoxR],
                        Robot => [Robot, Empty],
                        e => [*e, *e],
                    })
                    .collect()
            })
            .collect();
        self.robot_pos *= IVec2::new(1, 2);
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

fn can_be_moved(state: &State, pos: &IVec2, direction: &IVec2, to_move: &mut Vec<IVec2>) -> bool {
    let tile_in_front = state.map[(pos + direction).x as usize][(pos + direction).y as usize];
    if tile_in_front == Wall {
        return false;
    }
    if tile_in_front == Empty {
        return true;
    }

    let to_check_dir = match tile_in_front {
        BoxL => Right.as_vec(),
        BoxR => Left.as_vec(),
        _ => IVec2::new(0, 0),
    };

    if direction == &Up.as_vec() || direction == &Down.as_vec() {
        to_move.push(pos + direction);
        to_move.push(pos + direction + to_check_dir);
        return can_be_moved(state, &(pos + direction), direction, to_move)
            && can_be_moved(state, &(pos + direction + to_check_dir), direction, to_move);
    }
    to_move.push(pos + direction);
    can_be_moved(state, &(pos + direction), direction, to_move)
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

    let mut to_move = Vec::new();
    if can_be_moved(state, &state.robot_pos, &dir, &mut to_move) {
        let mut new_map = state.map.clone();
        for pos in &to_move {
            *get_tile(&mut new_map, pos) = Empty;
        }

        for pos in &to_move {
            *get_tile(&mut new_map, &(pos + dir)) = *get_tile(&mut state.map, pos);
        }
        state.map = new_map;
        *get_tile(&mut state.map, &pos_in_front) = Robot;
        *get_tile(&mut state.map, &state.robot_pos) = Empty;
        state.robot_pos = pos_in_front;
    }
}

fn get_gps_sum(state: &State) -> usize {
    state
        .map
        .iter()
        .enumerate()
        .flat_map(|(l_idx, l)| {
            l.iter().enumerate().map(move |(t_idx, t)| {
                if t == &Box || t == &BoxL {
                    100 * l_idx + t_idx
                } else {
                    0
                }
            })
        })
        .sum()
}

fn part12(state: &mut State) -> usize {
    while !state.directions.is_empty() {
        do_move(state);
    }
    get_gps_sum(state)
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part12(&mut input.clone()));
    input.widen();
    println!("{}", part12(&mut input));
}
