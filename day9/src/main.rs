fn parse(input: &str) -> Vec<isize> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .flat_map(|(c_idx, c)| {
            (0..c.to_digit(10).unwrap()).map(move |_| {
                if c_idx % 2 == 0 {
                    c_idx as isize / 2
                } else {
                    -1
                }
            })
        })
        .collect()
}

fn checksum(filesystem: &[isize]) -> usize {
    filesystem
        .iter()
        .enumerate()
        .map(|(c_idx, c)| if c >= &0 { c_idx * *c as usize } else { 0 })
        .sum()
}

fn part1(filesystem: &mut [isize]) -> usize {
    for space_idx in 0..filesystem.len() {
        if filesystem[space_idx] >= 0 {
            continue;
        }
        let last = filesystem
            .iter()
            .enumerate()
            .rev()
            .find(|(_, n)| n >= &&0)
            .unwrap();
        if last.0 < space_idx {
            break;
        }
        filesystem.swap(space_idx, last.0);
    }
    checksum(filesystem)
}

fn get_chunk_size(filesystem: &[isize], idx: usize) -> usize {
    let mut chunk_size = 0;
    while idx + chunk_size < filesystem.len() && filesystem[idx + chunk_size] == filesystem[idx] {
        chunk_size += 1;
    }
    chunk_size
}

fn part2(filesystem: &mut [isize]) -> usize {
    for move_id in (0..=*filesystem.iter().max().unwrap()).rev() {
        let move_idx = filesystem
            .iter()
            .enumerate()
            .find(|(_, c)| c == &&move_id)
            .unwrap()
            .0;
        let move_chunk_size = get_chunk_size(filesystem, move_idx);
        for free_idx in 0..move_idx {
            if filesystem[free_idx] >= 0 {
                continue;
            }
            if get_chunk_size(filesystem, free_idx) >= move_chunk_size {
                for i in 0..move_chunk_size {
                    filesystem.swap(move_idx + i, free_idx + i);
                }
                break;
            }
        }
    }
    checksum(filesystem)
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input.clone()));
    println!("{}", part2(&mut input));
}
