use rayon::prelude::*;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.replace(':', "")
                .split(' ')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_possible(res: usize, curr: usize, nums: &[usize], part2: bool) -> bool {
    if curr > res {
        return false;
    }
    if nums.is_empty() {
        return res == curr;
    }
    is_possible(res, curr * nums[0], &nums[1..], part2)
        || is_possible(res, curr + nums[0], &nums[1..], part2)
        || if part2 {
            is_possible(
                res,
                curr * 10usize.pow(nums[0].ilog10() + 1) + nums[0],
                &nums[1..],
                part2,
            )
        } else {
            false
        }
}

fn part12(input: &[Vec<usize>], part2: bool) -> usize {
    input
        .par_iter()
        .filter(|l| is_possible(l[0], l[1], &l[2..], part2))
        .map(|l| l[0])
        .sum()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part12(&input, false));
    println!("{}", part12(&input, true));
}
