use regex::Regex;

#[derive(Debug, Clone)]
struct Machine {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    x3: isize,
    y3: isize,
}

#[rustfmt::skip]
fn parse(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .split("\n\n")
        .map(|m| {
            let mut captures = re.captures_iter(m);
            Machine {
                x1: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                y1: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                x2: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                y2: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                x3: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                y3: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn solve(m: &Machine) -> isize {
    let d = m.x1 * m.y2 - m.y1 * m.x2;
    let a = (m.x3 * m.y2 - m.x2 * m.y3) / d;
    let b = (m.x1 * m.y3 - m.x3 * m.y1) / d;
    if a * m.x1 + b * m.x2 == m.x3 && a * m.y1 + b * m.y2 == m.y3 {
        3 * a + b
    } else {
        0
    }
}

fn part1(machines: &[Machine]) -> isize {
    machines.iter().map(solve).sum()
}

fn part2(machines: &mut [Machine]) -> isize {
    machines
        .iter_mut()
        .map(|m| {
            m.x3 += 10000000000000;
            m.y3 += 10000000000000;
            m
        })
        .map(|m| solve(m))
        .sum()
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&mut input));
}
