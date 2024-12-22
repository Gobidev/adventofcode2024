use std::{collections::HashMap, fmt::Display, iter};

use glam::IVec2;
use itertools::Itertools;
use phf::phf_map;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Pad {
    Keypad,
    Numpad,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_vec(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(-1, 0),
            Direction::Down => IVec2::new(1, 0),
            Direction::Left => IVec2::new(0, -1),
            Direction::Right => IVec2::new(0, 1),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

static NUMPAD_POSITIONS: phf::Map<char, IVec2> = phf_map! {
    '7' => IVec2::new(0, 0),
    '8' => IVec2::new(0, 1),
    '9' => IVec2::new(0, 2),
    '4' => IVec2::new(1, 0),
    '5' => IVec2::new(1, 1),
    '6' => IVec2::new(1, 2),
    '1' => IVec2::new(2, 0),
    '2' => IVec2::new(2, 1),
    '3' => IVec2::new(2, 2),
    '0' => IVec2::new(3, 1),
    'A' => IVec2::new(3, 2),
};

static KEYPAD_POSITIONS: phf::Map<char, IVec2> = phf_map! {
    '^' => IVec2::new(0, 1),
    'A' => IVec2::new(0, 2),
    '<' => IVec2::new(1, 0),
    'v' => IVec2::new(1, 1),
    '>' => IVec2::new(1, 2),
};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn directions(pos1: &IVec2, pos2: &IVec2) -> Vec<Vec<Direction>> {
    let diff = pos2 - pos1;
    let mut path_directions = vec![];
    if diff.x > 0 {
        path_directions.extend((0..diff.x).map(|_| Direction::Down));
    } else {
        path_directions.extend((0..diff.x.abs()).map(|_| Direction::Up));
    }
    if diff.y > 0 {
        path_directions.extend((0..diff.y).map(|_| Direction::Right));
    } else {
        path_directions.extend((0..diff.y.abs()).map(|_| Direction::Left));
    }
    let reverse = path_directions.iter().rev().map(|d| d.to_owned()).collect();
    if reverse == path_directions {
        vec![path_directions]
    } else {
        vec![path_directions, reverse]
    }
}

fn is_allowed_path(pos1: &IVec2, directions: &[Direction], banned_pos: &IVec2) -> bool {
    let mut pos = *pos1;
    for direction in directions {
        pos += direction.to_vec();
        if pos == *banned_pos {
            return false;
        }
    }
    true
}

fn get_paths(pos1: &IVec2, pos2: &IVec2, banned_pos: &IVec2) -> Vec<Vec<Direction>> {
    directions(pos1, pos2)
        .iter()
        .filter(|d| is_allowed_path(pos1, d, banned_pos))
        .map(|d| d.to_owned())
        .collect()
}

fn min_length(code: &str, pads: &[Pad], cache: &mut HashMap<(String, usize), usize>) -> usize {
    // credits: RubixDev

    if pads.is_empty() {
        return code.len();
    }
    if let Some(val) = cache.get(&(code.to_string(), pads.len())) {
        return *val;
    }
    let result = std::iter::once('A')
        .chain(code.chars())
        .tuple_windows()
        .map(|(start, end)| {
            match pads[0] {
                Pad::Numpad => get_paths(
                    NUMPAD_POSITIONS.get(&start).unwrap(),
                    NUMPAD_POSITIONS.get(&end).unwrap(),
                    &IVec2::new(3, 0),
                ),
                Pad::Keypad => get_paths(
                    KEYPAD_POSITIONS.get(&start).unwrap(),
                    KEYPAD_POSITIONS.get(&end).unwrap(),
                    &IVec2::new(0, 0),
                ),
            }
            .into_iter()
            .map(|dirs| dirs.iter().map(|d| d.to_string()).collect::<String>() + "A")
        })
        .multi_cartesian_product()
        .map(|combination| {
            combination
                .iter()
                .map(|c| min_length(c, &pads[1..], cache))
                .sum::<usize>()
        })
        .min()
        .unwrap();
    cache.insert((code.to_string(), pads.len()), result);
    result
}

fn part12(input: &[String], keypad_robot_count: usize) -> usize {
    let mut cache = HashMap::new();
    let pads: Vec<Pad> = iter::once(Pad::Numpad)
        .chain((0..keypad_robot_count).map(|_| Pad::Keypad))
        .collect();
    input
        .iter()
        .map(|s| s[..s.len() - 1].parse::<usize>().unwrap() * min_length(s, &pads, &mut cache))
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part12(&input, 2));
    println!("{}", part12(&input, 25));
}
