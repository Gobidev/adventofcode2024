use {once_cell::sync::Lazy, regex::Regex};

fn part1(input: &str) -> u32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    RE.captures_iter(input)
        .map(|c| {
            c.extract::<2>()
                .1
                .map(|d| d.parse::<u32>().unwrap())
                .iter()
                .product::<u32>()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .split(r"do()")
        .map(|s| part1(s.split(r"don't()").next().unwrap()))
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
