use crate::utils;

fn process_line_1(line: &str) -> isize {
    let mut numbers: Vec<isize> = utils::parse_numbers::<isize>(line);
    let mut ret: isize = 0;

    while numbers.iter().any(|x| *x != 0) {
        ret += *numbers.iter().last().unwrap();
        numbers = numbers.windows(2).map(|t| t[1] - t[0]).collect()
    }

    ret
}

fn process_line_2(line: &str) -> isize {
    let mut numbers: Vec<isize> = utils::parse_numbers::<isize>(line);
    let mut data: Vec<isize> = Vec::new();

    while numbers.iter().any(|x| *x != 0) {
        data.push(numbers[0]);
        numbers = numbers.windows(2).map(|t| t[1] - t[0]).collect()
    }

    let mut ret: isize = 0;
    for x in data.iter().rev() {
        ret = x - ret;
    }

    ret
}

#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
    input.lines().map(process_line_1).sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> isize {
    input.lines().map(process_line_2).sum()
}
