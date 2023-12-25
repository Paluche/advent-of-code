use pathfinding::{
    directed::bfs::bfs_reach,
    matrix::{
        directions::{E, N, S, W},
        Matrix,
    },
};
use crate::utils::shoelace;

fn find_entry(matrix: &Matrix<char>) -> Position {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No entry point");
}

type Position = (usize, usize);

fn parse_input(input: &str) -> Vec<Position> {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let start = find_entry(&matrix);

    // To be thorough, we should analyze the neighbors of the S tile and decide
    // by what to replace the S tile with.

    bfs_reach(
        start,
        |x| { (match matrix[*x] {
            'S' => vec![S, E, W, N],
            '|' => vec![S, N],
            '-' => vec![E, W],
            'L' => vec![N, E],
            'J' => vec![W, N],
            '7' => vec![W, S],
            'F' => vec![S, E],
            '.' => Vec::new(),
            _ => panic!(),
        }) .iter()
        .filter_map(|d| matrix.move_in_direction(*x, *d))
        .collect::<Vec<(usize, usize)>>()
        },
    ).collect()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    parse_input(input).len() / 2
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    shoelace::<usize>(&parse_input(input))
}
