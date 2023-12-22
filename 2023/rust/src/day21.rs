use std::collections::HashSet;

use pathfinding::{matrix::Matrix, prelude::directions::{W, N, E, S}};

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

fn run<F>(start: (usize, usize), steps: usize, successors: F) -> HashSet<(usize, usize)>
    where
    F: Fn((usize, usize)) -> Vec<(usize, usize)>
{
    let mut ret: HashSet<(usize, usize)> = HashSet::new();

    ret.insert(start);

    for _ in 0..steps {
        let mut next_ret: HashSet<(usize, usize)> = HashSet::new();
        for position in ret {
            successors(position).iter().for_each(|p| {next_ret.insert(*p);});
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
        [N, E, S, W].iter().filter_map(|d| {
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

// Plan for part 2.
// => Create a cache where we remember all the result within one original map.
// The elf move one step at a time, so the starting points are
//  - The S case,
//  - All cases on the row 0
//  - All cases on the row map.rows - 1
//  - All cases on the columns 0
//  - All cases on the columns map.columns - 1

// The output will be a list of positions on the outside edge of the map, with a list
// of number of step that can lead to that position. Then

// At a certain number of steps the map will be fully accessible. No interest
// in computing those points anymore..
//
// That certain number of step will create some pattern positions on the
// neighbors map.
//
//
