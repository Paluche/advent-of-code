use crate::utils;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();

    let times = utils::parse_numbers::<usize>(&lines.next().unwrap()["Time:".len()..]);
    let distances = utils::parse_numbers::<usize>(&lines.next().unwrap()["Distance:".len()..]);

    std::iter::zip(times, distances).collect()
}

fn find_winning_values(time: usize, distance: usize) -> Vec<usize> {
    (0..time).filter(|x| ((time - x) * x) > distance).collect()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(t, d)| find_winning_values(*t, *d).len())
        .product()
}
