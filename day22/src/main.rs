use ahash::{AHashMap, AHashSet};

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn evolve_number(curr: &mut usize) {
    *curr ^= *curr << 6;
    *curr &= 0xffffff;
    *curr ^= *curr >> 5;
    *curr &= 0xffffff;
    *curr ^= *curr << 11;
    *curr &= 0xffffff;
}

fn part1(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .map(|secret| {
            let mut res = *secret;
            (0..2000).for_each(|_| evolve_number(&mut res));
            res
        })
        .sum()
}

fn part2(numbers: &[usize]) -> isize {
    // kinda slow
    let mut sequence_bananas: AHashMap<(isize, isize, isize, isize), Vec<isize>> = AHashMap::new();
    for number in numbers {
        let mut costs_and_changes: Vec<(isize, isize)> = vec![];
        let mut curr_magic = *number;
        for _ in 0..2000 {
            let cost = curr_magic % 10;
            costs_and_changes.push((
                cost as isize,
                costs_and_changes.last().map_or(0, |l| cost as isize - l.0),
            ));
            evolve_number(&mut curr_magic);
        }
        let sequence_costs = costs_and_changes
            .windows(4)
            .skip(1)
            .map(|w| ((w[0].1, w[1].1, w[2].1, w[3].1), w[3].0));
        let mut seen_sequences = AHashSet::new();
        for (k, v) in sequence_costs {
            if seen_sequences.contains(&k) {
                continue;
            }
            sequence_bananas
                .entry(k)
                .and_modify(|v2| v2.push(v))
                .or_insert(vec![v]);
            seen_sequences.insert(k);
        }
    }
    sequence_bananas
        .get(
            sequence_bananas
                .iter()
                .max_by(|a, b| a.1.iter().sum::<isize>().cmp(&b.1.iter().sum::<isize>()))
                .unwrap()
                .0,
        )
        .unwrap()
        .iter()
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
