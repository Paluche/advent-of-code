use std::collections::HashSet;
use pathfinding::matrix::{
    directions::{E, N, S, W},
    Matrix,
};

type Position = (usize, usize);
type Direction = (isize, isize);

enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

impl Terrain {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(N),
            '>' => Self::Slope(E),
            '<' => Self::Slope(W),
            'v' => Self::Slope(S),
            _ => panic!(),
        }
    }
}

fn next(
    map: &Matrix<Terrain>,
    start: Position,
    direction: Direction,
    weight: usize,
) -> Option<(Position, usize)> {
    let position = map.move_in_direction(start, direction)?;

    let weight = weight + 1;

    match map[position] {
        Terrain::Path => Some((position, weight)),
        Terrain::Forest => None,
        Terrain::Slope(d) => next(map, position, d, weight),
    }
}

fn successors(
    map: &Matrix<Terrain>,
    start: Position,
) -> Vec<(Position, usize)> {
    [N, E, S, W]
        .iter()
        .filter_map(|&d| next(map, start, d, 0))
        .collect()
}

fn parse_input(input: &str) -> (Matrix<Terrain>, Position, Position) {
    let map: Matrix<Terrain> = Matrix::from_rows(
        input
            .lines()
            .take_while(|x| !x.is_empty())
            .map(|l| l.chars().map(Terrain::from_char)),
    )
    .unwrap();

    let start: Position = (0..map.columns)
        .find_map(|c| match map[(0, c)] {
            Terrain::Path => Some((0, c)),
            _ => None,
        })
        .unwrap();

    let end: Position = (0..map.columns)
        .find_map(|c| match map[(map.rows - 1, c)] {
            Terrain::Path => Some((map.rows - 1, c)),
            _ => None,
        })
        .unwrap();

    (map, start, end)
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let (map, start, end) = parse_input(input);

    let mut ways: Vec<(Position, usize, HashSet<Position>)> = vec![(start, 0, HashSet::new())];
    let mut ok_ways: Vec<usize> = Vec::new();

    while !ways.is_empty() {
        ways = ways.iter().filter_map(|(position, weight, path)| {
            let next: Vec<(Position, usize)> = successors(&map, *position)
                .iter()
                .filter_map(|(pos, weight)| if path.contains(pos) {
                    None
                } else {
                    Some((*pos, *weight))
                })
                .collect();

            if next.is_empty() {
                return None;
            }

            Some(next.iter().filter_map(|(position, next_weight)| {
                let mut path = path.clone();

                path.insert(*position);

                if *position == end {
                    ok_ways.push(weight + next_weight);
                    None
                } else {
                    Some((*position, weight + next_weight, path))
                }
            }).collect::<Vec<(Position, usize, HashSet<Position>)>>()
            )
        }
        )
        .flatten()
        .collect();
    }

    *ok_ways.iter().max().unwrap()
}
