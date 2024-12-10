use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(10)).collect())
        .collect()
}

fn get_score(pos: &(usize, usize), prev: &isize, map: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    let Some(Some(val)) = map.get(pos.0).map(|x| x.get(pos.1)) else {
        return res;
    };
    if *val != (prev + 1) as u32 {
        return res;
    }
    if val == &9 && prev == &8 {
        res.insert(*pos);
        return res;
    }
    res.extend(get_score(
        &(pos.0.overflowing_sub(1).0, pos.1),
        &(*val as isize),
        map,
    ));
    res.extend(get_score(&(pos.0 + 1, pos.1), &(*val as isize), map));
    res.extend(get_score(
        &(pos.0, pos.1.overflowing_sub(1).0),
        &(*val as isize),
        map,
    ));
    res.extend(get_score(&(pos.0, pos.1 + 1), &(*val as isize), map));
    res
}

fn get_rating(pos: &(usize, usize), prev: &isize, map: &[Vec<u32>]) -> usize {
    let Some(Some(val)) = map.get(pos.0).map(|x| x.get(pos.1)) else {
        return 0;
    };
    if *val != (prev + 1) as u32 {
        return 0;
    }
    if val == &9 && prev == &8 {
        return 1;
    }
    get_rating(&(pos.0.overflowing_sub(1).0, pos.1), &(*val as isize), map)
        + get_rating(&(pos.0 + 1, pos.1), &(*val as isize), map)
        + get_rating(&(pos.0, pos.1.overflowing_sub(1).0), &(*val as isize), map)
        + get_rating(&(pos.0, pos.1 + 1), &(*val as isize), map)
}

fn part12(input: &[Vec<u32>], part1: bool) -> usize {
    let trailheads: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(l_idx, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| c == &&0)
                .map(move |(c_idx, _)| (l_idx, c_idx))
        })
        .collect();
    if part1 {
        trailheads
            .iter()
            .flat_map(|t| get_score(t, &-1, input))
            .count()
    } else {
        trailheads.iter().map(|t| get_rating(t, &-1, input)).sum()
    }
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part12(&input, true));
    println!("{}", part12(&input, false));
}
