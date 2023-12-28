use pathfinding::cycle_detection::brent;

const SEED: usize = 20151125;

fn next(prev: &mut usize) {
    *prev *= 252533;
    *prev %= 33554393;
}

fn parse_input(input: &str) -> (usize, usize) {
    let input = input.strip_prefix(
        "To continue, please consult the code grid in the manual.  Enter the code at row ")
     .unwrap();

    let (row, column) = input.split_once(", column ").unwrap();
    let column = column.strip_suffix(".\n").unwrap();

    (row.parse().expect(""), column.parse().expect(""))
}

fn to_index((row, column): (usize, usize)) -> usize {
    let row = row - 1;
    let column = column - 1;
    (0..=column).sum::<usize>() + (column..=row + column).sum::<usize>()
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let (cycle_size, first, start) = brent(SEED, |mut a| {
        next(&mut a);
        a
    });
    let mut ret = first;
    let index = to_index(parse_input(input));

    for _ in 0..((index - start) % cycle_size) {
        next(&mut ret);
    }

    ret
}
