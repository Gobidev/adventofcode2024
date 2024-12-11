use ahash::AHashMap;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn split_number(num: &usize) -> (usize, usize) {
    let mut digit_count = 0;
    let mut rest = *num;
    while rest > 0 {
        rest /= 10;
        digit_count += 1;
    }
    let divisor = 10usize.pow(digit_count / 2);
    (num % divisor, num / divisor)
}

fn get_amount(num: &usize, steps: &usize, cache: &mut AHashMap<(usize, usize), usize>) -> usize {
    if let Some(val) = cache.get(&(*num, *steps)) {
        return *val;
    }
    if steps == &0 {
        return 1;
    }
    let mut res = 0;
    if num == &0 {
        res += get_amount(&1, &(steps - 1), cache);
    } else if num.to_string().chars().count() % 2 == 0 {
        let (splitl, splitr) = split_number(num);
        res += get_amount(&splitl, &(steps - 1), cache);
        res += get_amount(&splitr, &(steps - 1), cache);
    } else {
        res += get_amount(&(num * 2024), &(steps - 1), cache);
    }
    cache.insert((*num, *steps), res);
    res
}

fn part(input: &[usize], steps: usize) -> usize {
    let mut cache = AHashMap::new();
    input
        .iter()
        .map(|n| get_amount(n, &steps, &mut cache))
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part(&input, 25));
    println!("{}", part(&input, 75));
}
