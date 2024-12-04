fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &[Vec<char>]) -> u32 {
    let xmas = ['X', 'M', 'A', 'S'];
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    input
        .iter()
        .enumerate()
        .map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c != &xmas[0] {
                        return 0;
                    }
                    directions
                        .iter()
                        .map(|(dir_x, dir_y)| {
                            xmas.iter()
                                .enumerate()
                                .skip(1)
                                .fold(1, |acc, (shift, xmas_char)| {
                                    let Some(Some(cc)) = input
                                        .get((line_idx as i32 + (dir_y * (shift as i32))) as usize)
                                        .map(|ll| {
                                            ll.get(
                                                (col_idx as i32 + dir_x * (shift as i32)) as usize,
                                            )
                                        })
                                    else {
                                        return 0;
                                    };
                                    if cc != xmas_char {
                                        return 0;
                                    }
                                    acc
                                })
                        })
                        .sum::<u32>()
                })
                .sum::<u32>()
        })
        .sum()
}

fn part2(input: &[Vec<char>]) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c != &'A' {
                        return 0;
                    }
                    let Some(Some(up_left)) = input
                        .get(line_idx.overflowing_sub(1).0)
                        .map(|l| l.get(col_idx.overflowing_sub(1).0))
                    else {
                        return 0;
                    };
                    let Some(Some(up_right)) = input
                        .get(line_idx.overflowing_sub(1).0)
                        .map(|l| l.get(col_idx + 1))
                    else {
                        return 0;
                    };
                    let Some(Some(down_left)) = input
                        .get(line_idx + 1)
                        .map(|l| l.get(col_idx.overflowing_sub(1).0))
                    else {
                        return 0;
                    };
                    let Some(Some(down_right)) =
                        input.get(line_idx + 1).map(|l| l.get(col_idx + 1))
                    else {
                        return 0;
                    };
                    if ((up_left == &'M' && down_right == &'S')
                        || (up_left == &'S' && down_right == &'M'))
                        && ((up_right == &'M' && down_left == &'S')
                            || (up_right == &'S' && down_left == &'M'))
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}
fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
