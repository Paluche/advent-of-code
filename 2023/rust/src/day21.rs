use std::collections::HashSet;

use pathfinding::{
    matrix::Matrix,
    prelude::directions::{E, N, S, W},
};

fn find_start(map: &Matrix<char>) -> (usize, usize) {
    for (r, row) in map.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v == 'S' {
                return (r, c);
            }
        }
    }

    panic!()
}

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn run<F>(
    start: (usize, usize),
    steps: usize,
    successors: F,
) -> HashSet<(usize, usize)>
where
    F: Fn((usize, usize)) -> Vec<(usize, usize)>,
{
    let mut ret: HashSet<(usize, usize)> = HashSet::new();

    ret.insert(start);

    for _ in 0..steps {
        let mut next_ret: HashSet<(usize, usize)> = HashSet::new();
        for position in ret {
            successors(position).iter().for_each(|p| {
                next_ret.insert(*p);
            });
        }

        ret = next_ret;
    }

    ret
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let start = find_start(&map);

    run(start, 64, |p| {
        [N, E, S, W]
            .iter()
            .filter_map(|d| {
                let ret = map.move_in_direction(p, *d)?;
                if *map.get(ret).unwrap() == '#' {
                    None
                } else {
                    Some(ret)
                }
            })
            .collect()
    })
    .len()
}
