use crate::utils::parse_numbers;

fn run(input: &str) -> usize {
    let mut lines = input.lines();

    parse_numbers::<usize>(&lines.next().unwrap()["Time:".len()..])
        .iter()
        .zip(parse_numbers::<usize>(
            &lines.next().unwrap()["Distance:".len()..],
        ))
        .map(|(t, d)| {
            (0..*t)
                .filter(|x| ((*t - x) * x) > d)
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
