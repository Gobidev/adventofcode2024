use glam::IVec2;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

#[rustfmt::skip]
fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"-?\d+").unwrap();
    input.lines().map(|l| {
        let mut captures = re.captures_iter(l);
        Robot {
            position: IVec2 {
                x: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                y: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
            },
            velocity: IVec2 {
                x: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                y: captures.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
            }
        }
    }).collect()
}

fn get_pos_after_steps(robot: &Robot, steps: &i32, dimensions: &IVec2) -> IVec2 {
    (robot.position + steps * robot.velocity).rem_euclid(*dimensions)
}

fn part1(input: &[Robot], dimensions: &IVec2) -> usize {
    input
        .iter()
        .fold([0, 0, 0, 0], |mut acc, curr| {
            let p = get_pos_after_steps(curr, &100, dimensions);
            if p.x < (dimensions.x / 2) && p.y < (dimensions.y / 2) {
                acc[0] += 1;
            }
            if p.x > (dimensions.x / 2) && p.y < (dimensions.y / 2) {
                acc[1] += 1;
            }
            if p.x < (dimensions.x / 2) && p.y > (dimensions.y / 2) {
                acc[2] += 1
            }
            if p.x > (dimensions.x / 2) && p.y > (dimensions.y / 2) {
                acc[3] += 1;
            }
            acc
        })
        .iter()
        .product()
}

fn print_map(robots: &[Robot], dimensions: &IVec2) -> String {
    let mut res = String::new();
    for y in 0..dimensions.x {
        for x in 0..dimensions.y {
            if robots
                .iter()
                .map(|r| (r.position.x, r.position.y))
                .any(|p| p == (x, y))
            {
                res.push('#');
            } else {
                res.push(' ');
            }
        }
        res.push('\n');
    }
    res
}

fn has_tree(robots: &[Robot]) -> bool {
    let check: Vec<_> = (0..8).map(|i| IVec2::new(i, 0)).collect();
    robots.iter().any(|p| {
        check
            .iter()
            .all(|c| robots.iter().any(|p2| p2.position == p.position + c))
    })
}

fn part2(robots: &mut [Robot], dimensions: &IVec2) -> i32 {
    for i in 0.. {
        if has_tree(robots) {
            eprintln!("{}", print_map(robots, dimensions));
            return i;
        }
        robots
            .iter_mut()
            .for_each(|r| r.position = get_pos_after_steps(r, &1, dimensions))
    }
    unreachable!()
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input, &IVec2::new(101, 103)));
    println!("{}", part2(&mut input, &IVec2::new(101, 103)));
}
