use crate::utils;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();

    let times =
        utils::parse_numbers::<usize>(&lines.next().unwrap()["Time:".len()..]);
    let distances = utils::parse_numbers::<usize>(
        &lines.next().unwrap()["Distance:".len()..],
    );

    std::iter::zip(times, distances).collect()
}

fn run(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(t, d)| {
            (0..*t)
                .filter(|x| ((*t - x) * x) > *d)
                .collect::<Vec<usize>>()
                .len()
        })
        .product()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    run(input)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    run(&input.replace(' ', ""))
}
