use ahash::AHashSet;
use glam::IVec2;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

const DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: -1 },
    IVec2 { x: 0, y: 1 },
];

fn get_region(pos: &IVec2, map: &[Vec<char>]) -> AHashSet<IVec2> {
    let mut res = AHashSet::new();
    let mut prev_amount = 0;
    let plot_type = map[pos.x as usize][pos.y as usize];
    res.insert(*pos);
    while res.len() != prev_amount {
        let mut new_plots = AHashSet::new();
        for plot in &res {
            for direction in DIRECTIONS {
                let new_pos = plot + direction;
                if let Some(Some(new_plot)) = map
                    .get(new_pos.x as usize)
                    .map(|v| v.get(new_pos.y as usize))
                {
                    if new_plot == &plot_type {
                        new_plots.insert(new_pos);
                    }
                }
            }
        }
        prev_amount = res.len();
        res.extend(new_plots);
    }
    res
}

fn get_perimeter(region: &AHashSet<IVec2>) -> usize {
    let mut res = 0;
    for pos in region {
        for direction in DIRECTIONS {
            if !region.contains(&(pos + direction)) {
                res += 1;
            }
        }
    }
    res
}

fn get_connected(pos: &IVec2, all_pos: &AHashSet<IVec2>) -> AHashSet<IVec2> {
    let mut res = AHashSet::new();
    res.insert(*pos);
    let mut last_len = 0;
    while last_len != res.len() {
        let mut new_positions = AHashSet::new();
        for checking in &res {
            for direction in DIRECTIONS {
                if all_pos.contains(&(checking + direction)) {
                    new_positions.insert(checking + direction);
                }
            }
        }
        last_len = res.len();
        res.extend(new_positions);
    }
    res
}

fn get_sides(region: &AHashSet<IVec2>) -> usize {
    let mut res = 0;
    for direction in DIRECTIONS {
        let mut outside = AHashSet::new();
        for plot in region {
            if !region.contains(&(plot + direction)) {
                outside.insert(plot + direction);
            }
        }
        while !outside.is_empty() {
            let connected = get_connected(outside.iter().next().unwrap(), &outside);
            res += 1;
            outside.retain(|p| !connected.contains(p));
        }
    }
    res
}

fn part12(map: &[Vec<char>], part2: bool) -> usize {
    let mut positions: AHashSet<IVec2> = (0..map.len())
        .flat_map(|x| {
            (0..map[0].len()).map(move |y| IVec2 {
                x: x as i32,
                y: y as i32,
            })
        })
        .collect();
    let mut regions = vec![];
    while !positions.is_empty() {
        let region = get_region(positions.iter().next().unwrap(), map);
        positions.retain(|p| !region.contains(p));
        regions.push(region);
    }
    regions
        .iter()
        .map(|r| {
            r.len()
                * if part2 {
                    get_sides(r)
                } else {
                    get_perimeter(r)
                }
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part12(&input, false));
    println!("{}", part12(&input, true));
}
