use rayon::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn to_pos(self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
    fn turn(&mut self) {
        match self {
            Up => *self = Right,
            Down => *self = Left,
            Left => *self = Up,
            Right => *self = Down,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PositionType {
    Empty,
    Visited(Vec<Direction>),
    Obstructed,
}
use PositionType::*;

impl Display for PositionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Empty => write!(f, "."),
            Visited(_) => write!(f, "X"),
            Obstructed => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    map: Vec<Vec<PositionType>>,
    guard: Guard,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|l| l.iter().map(|c| c.to_string()).collect::<String>() + "\n")
                .collect::<String>()
        )
    }
}

fn parse(input: &str) -> State {
    let mut guard_pos = (0, 0);
    State {
        map: input
            .lines()
            .enumerate()
            .map(|(line_idx, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c_idx, c)| match c {
                        '#' => Obstructed,
                        '^' => {
                            guard_pos = (line_idx, c_idx);
                            Empty
                        }
                        _ => Empty,
                    })
                    .collect()
            })
            .collect(),
        guard: Guard {
            position: guard_pos,
            direction: Up,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveResult {
    EndedOutside,
    EndedLoop,
    Running,
}
use MoveResult::*;

fn move_guard(state: &mut State) -> MoveResult {
    if let Visited(ref mut directions) =
        state.map[state.guard.position.0][state.guard.position.1]
    {
        if directions.contains(&state.guard.direction) {
            return EndedLoop;
        }
        directions.push(state.guard.direction);
    } else {
        state.map[state.guard.position.0][state.guard.position.1] =
            Visited(vec![]);
    }
    let direction_pos = state.guard.direction.to_pos();
    let pos_in_front = (
        ((state.guard.position.0 as isize) + direction_pos.0) as usize,
        ((state.guard.position.1 as isize) + direction_pos.1) as usize,
    );
    let Some(line) = state.map.get(pos_in_front.0) else {
        return EndedOutside;
    };
    let Some(pos_type) = line.get(pos_in_front.1) else {
        return EndedOutside;
    };
    match pos_type {
        Obstructed => {
            state.guard.direction.turn();
        }
        _ => {
            state.guard.position = pos_in_front;
        }
    }
    Running
}

fn part1(state: &mut State) -> usize {
    while move_guard(state) == Running {}
    state
        .map
        .iter()
        .map(|l| l.iter().filter(|c| matches!(c, Visited(_))).count())
        .sum()
}

fn part2(state: &State) -> usize {
    let mut original_path = state.clone();
    while move_guard(&mut original_path) == Running {}
    let original_path_positions: Vec<(usize, usize)> = original_path
        .map
        .iter()
        .enumerate()
        .flat_map(|(l_idx, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| matches!(c, Visited(_)))
                .map(move |(idx, _)| (l_idx, idx))
        })
        .collect();
    original_path_positions
        .par_iter()
        .filter(|pos| {
            let mut new_start_state = state.clone();
            new_start_state.map[pos.0][pos.1] = Obstructed;
            loop {
                match move_guard(&mut new_start_state) {
                    EndedOutside => {
                        return false;
                    }
                    EndedLoop => {
                        return true;
                    }
                    Running => (),
                }
            }
        })
        .count()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input.clone()));
    println!("{}", part2(&input));
}
