fn run(input: &str, floor_dest: Option<isize>) -> isize {
    let mut floor = 0isize;
    let input: Vec<char> = input.lines().next().unwrap().chars().collect();

    for (i, c) in input.iter().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }

        if let Some(floor_dest) = floor_dest {
            if floor == floor_dest {
                return i as isize;
            }
        }
    }

    floor
}

#[aoc(day1, part1)]
fn part1(input: &str) -> isize {
    run(input, None)
}

#[aoc(day1, part2)]
fn part2(input: &str) -> isize {
    run(input, Some(-1_isize))
}
