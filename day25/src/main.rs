#[derive(Debug, Clone)]
struct ParsedInput {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

fn parse_key(grid: &[Vec<char>]) -> [u8; 5] {
    let mut res = [0; 5];
    for (l_idx, line) in grid.iter().skip(1).enumerate().rev().skip(1) {
        for (c_idx, c) in line.iter().enumerate() {
            if c == &'#' && res[c_idx] == 0 {
                res[c_idx] = l_idx as u8 + 1;
            }
        }
    }
    res
}

fn parse(input: &str) -> ParsedInput {
    let mut keys = vec![];
    let mut locks = vec![];
    for block in input.split("\n\n") {
        let mut grid: Vec<Vec<char>> = block.lines().map(|l| l.chars().collect()).collect();
        if grid[0][0] == '#' {
            locks.push(parse_key(&grid));
        } else {
            grid.reverse();
            keys.push(parse_key(&grid));
        }
    }
    ParsedInput { keys, locks }
}

fn overlaps(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    key.iter().zip(lock.iter()).any(|(k, l)| 5 - k < *l)
}

fn part1(parsed_input: &ParsedInput) -> usize {
    parsed_input
        .locks
        .iter()
        .map(|l| parsed_input.keys.iter().filter(|k| !overlaps(k, l)).count())
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
}
