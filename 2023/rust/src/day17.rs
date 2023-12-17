use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::matrix::{
    directions::{E, N, S, W},
    Matrix,
};
use std::hash::Hash;

fn parse_input(input: &str) -> Matrix<u8> {
    Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as u8)),
    )
    .unwrap()
}

type Direction = (isize, isize);

fn opposed(direction: Direction) -> Direction {
    match direction {
        N => S,
        E => W,
        S => N,
        W => E,
        _ => panic!(),
    }
}

type Position = (usize, usize);

#[derive(Clone, Hash, PartialEq, Eq)]
struct Step {
    position: Position,
    direction: Option<Direction>, // Last direction taken.
    count: usize, // Number of step done in this directions to get to this position.
}

impl Step {
    fn new_start(position: Position) -> Self {
        Self {
            position,
            direction: None,
            count: 1,
        }
    }

    fn next(
        &self,
        map: &Matrix<u8>,
        direction: Direction,
        max_move: usize,
    ) -> Option<(Self, usize)> {
        let mut next_count = 1;

        if let Some(self_direction) = self.direction {
            if self_direction == direction {
                // Going in the same direction as previously.
                if self.count >= max_move {
                    // Cannot go further in this direction.
                    return None;
                }
                next_count = self.count + 1;
            } else if self_direction == opposed(direction)
            {
                // Do not go backward or not enough move to allow a turn yet.
                return None;
            }
        }

        let position = map.move_in_direction(self.position, direction)?;

        Some((
            Self {
                position,
                direction: Some(direction),
                count: next_count,
            },
            map[position] as usize,
        ))
    }
}

fn run(
    map: &Matrix<u8>,
    start: Position,
    end: Position,
    max_move: usize,
) -> usize {
    dijkstra(
        &Step::new_start(start),
        |step| [N, E, S, W]
            .iter()
            .filter_map(|d| step.next(map, *d, max_move))
            .collect::<Vec<(Step, usize)>>(),
        |step| step.position == end,
    )
    .unwrap()
    .1
}

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
    let map = parse_input(input);
    run(&map, (0, 0), (map.rows - 1, map.columns - 1), 3)
}
