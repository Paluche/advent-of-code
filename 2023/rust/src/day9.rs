use crate::utils;

fn process_line(line: &str) -> isize {
    let mut numbers: Vec<isize> = utils::parse_numbers::<isize>(line);
    let mut ret: isize = 0;

    while numbers.iter().any(|x| *x != 0) {
        ret += *numbers.iter().last().unwrap();
        numbers = numbers.windows(2).map(|t| t[1] - t[0]).collect()
    }

    ret
}

#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
    input.lines().map(process_line).sum()
}
