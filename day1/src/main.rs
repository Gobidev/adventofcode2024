fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let tmp = line.split_once(' ');
            (
                tmp.unwrap().0.parse::<u32>().unwrap(),
                tmp.unwrap().1.trim().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut firsts = input.0.clone();
    let mut seconds = input.1.clone();
    firsts.sort_unstable();
    seconds.sort_unstable();
    firsts
        .iter()
        .zip(seconds.iter())
        .map(|(f, s)| u32::abs_diff(*f, *s))
        .sum()
}

fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    input
        .0
        .iter()
        .map(|num| num * input.1.iter().filter(|x| x == &num).count() as u32)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input = parse(input);
    println!("{}", part1(&parsed_input));
    println!("{}", part2(&parsed_input));
}
