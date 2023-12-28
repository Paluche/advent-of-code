use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

const EGGNOG_VOLUME: usize = 150;

// index, volume
// append for index..
// if volume > 150 => false
fn try_next(
    volume: usize,
    container_nb: usize,
    index: usize,
    containers: &[usize],
) -> Vec<usize> {
    containers
        .iter()
        .enumerate()
        .skip(index)
        .flat_map(|(i, v)| {
            let volume = volume + v;

            match volume.cmp(&EGGNOG_VOLUME) {
                Ordering::Less => {
                    try_next(volume, container_nb + 1, i + 1, containers)
                }
                Ordering::Equal => vec![container_nb + 1],
                Ordering::Greater => Vec::new(),
            }
        })
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
    try_next(0, 0, 0, &parse_input(input)).len()
}

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
    let res = try_next(0, 0, 0, &parse_input(input));
    let min = res.iter().min().unwrap();

    res.iter().filter(|x| *x == min).count()
}
