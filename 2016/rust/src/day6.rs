use std::collections::HashMap;

use pathfinding::matrix::Matrix;

fn run<F>(input: &str, mut f: F) -> String
where
    F: FnMut(&HashMap<char, usize>) -> char,
{
    let message = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let mut ret = String::with_capacity(message.columns);

    for column in 0..message.columns {
        let mut count: HashMap<char, usize> = HashMap::new();

        for row in 0..message.rows {
            let c = message[(row, column)];
            if let Some(x) = count.get_mut(&c) {
                *x += 1;
            } else {
                count.insert(c, 0);
            }
        }

        ret.push(f(&count));
    }

    ret
}

#[aoc(day6, part1)]
fn part1(input: &str) -> String {
    run(input, |count| {
        *count.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().0
    })
}

#[aoc(day6, part2)]
fn part2(input: &str) -> String {
    run(input, |count| {
        *count.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap().0
    })
}
