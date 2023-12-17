use crate::utils::print_matrix;
use pathfinding::matrix::{
    directions::{E, N, S, W},
    Matrix,
};

type Direction = (isize, isize);
type Position = (isize, isize);

#[derive(Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn to_digit(chars: &mut std::str::Chars) -> u8 {
    (chars.next().unwrap().to_digit(16).unwrap() * 16
        + chars.next().unwrap().to_digit(16).unwrap()) as u8
}

impl Color {
    fn from_str(input: &str) -> Self {
        let mut chars = input.strip_prefix("(#").unwrap().chars();

        let ret = Self {
            red: to_digit(&mut chars),
            green: to_digit(&mut chars),
            blue: to_digit(&mut chars),
        };

        assert_eq!(chars.next().unwrap(), ')');

        ret
    }
}

fn parse_input(input: &str) -> Vec<(Direction, isize, Color)> {
    input
        .lines()
        .map(|line| {
            let mut a = line.split_whitespace();

            let direction = match a.next().unwrap() {
                "R" => E,
                "L" => W,
                "U" => N,
                "D" => S,
                _ => panic!(),
            };

            let count = a.next().unwrap().parse::<isize>().unwrap();

            (direction, count, Color::from_str(a.next().unwrap()))
        })
        .collect()
}

fn get_edge(
    instructions: &[(Direction, isize, Color)],
) -> Vec<(Position, Direction, Color)> {
    // Run the instructions to determinate the list of points that makes the
    // edge.
    let mut position: Position = (0, 0);
    let mut edge: Vec<(Position, Direction, Color)> = Vec::new();

    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.1 {
            position =
                (position.0 + instruction.0 .0, position.1 + instruction.0 .1);

            edge.push((position, instruction.0, instruction.2));
        }
    });

    edge
}

fn get_wise(directions: Vec<Direction>) -> isize {
    directions
        .iter()
        .fold((0, (0, 0)), |(mut w, ld), d| {
            if ld == (0, 0) {
                return (w, *d);
            }

            w += match ld {
                S => match *d {
                    S => 0,
                    W => 1,
                    E => -1,
                    _ => panic!(),
                },
                W => match *d {
                    S => -1,
                    N => 1,
                    W => 0,
                    _ => panic!(),
                },
                N => match *d {
                    N => 0,
                    E => 1,
                    W => -1,
                    _ => panic!(),
                },
                E => match *d {
                    N => -1,
                    E => 0,
                    S => 1,
                    _ => panic!(),
                },
                _ => panic!(),
            };

            (w, *d)
        })
        .0
}

fn to_inside(direction: &Direction, edge_wise: isize) -> bool {
    match *direction {
        N => edge_wise > 0,
        E => false,
        S => edge_wise < 0,
        W => false,
        _ => panic!(),
    }
}
fn from_inside(direction: &Direction, edge_wise: isize) -> bool {
    match *direction {
        N => edge_wise < 0,
        E => false,
        S => edge_wise > 0,
        W => false,
        _ => panic!(),
    }
}

/// Get the points that compose the area within a loop.
fn get_area(
    edge: &[((isize, isize), Direction, Color)],
) -> Vec<(isize, isize)> {
    let edge_wise = get_wise(edge.iter().map(|(_, d, _)| *d).collect());

    println!("Edge wise: {edge_wise}");

    let edge:Vec<(Position, bool, bool)> = edge.windows(2).map(|w| {
        let (cur_pos, cur_dir, _) = w[0];
        let (_, next_dir, _) = w[1];
        (cur_pos,
         to_inside(&cur_dir, edge_wise) || to_inside(&next_dir, edge_wise),
         from_inside(&cur_dir, edge_wise) || from_inside(&next_dir, edge_wise)
        )
    }).collect();

    edge.iter()
        .filter_map(|((cur_row, cur_col), cur_to_inside, _)| {
            let mut ret: Vec<Position> = Vec::new();

            // Get the distance to the closest next point to the left that is on
            // the same row.
            let (next_col, next_from_inside) = match edge
                .iter()
                .filter_map(|((next_row, next_col), _, next_from_inside)| {
                    if (cur_row == next_row) && (cur_col < next_col) {
                        Some((*next_col, *next_from_inside))
                    } else {
                        None
                    }
                })
                .min_by(|(a, _), (b, _)| a.cmp(b))
            {
                Some(x) => x,
                None => return None,
            };

            if *cur_to_inside || next_from_inside {
                for i in cur_col + 1..next_col {
                    ret.push((*cur_row, i));
                }
            }

            Some(ret)
        })
        .flatten()
        .collect()
}

fn run(
    instructions: &[(Direction, isize, Color)],
) -> (Vec<Position>, Vec<Position>) {
    let edge = get_edge(instructions);

    (
        edge.iter().map(|(pos, _, _)| *pos).collect(),
        get_area(&edge),
    )
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    let (edge, area) = run(&parse_input(input));

    let min_row = edge.iter().map(|(row, _)| row).min().unwrap();
    let max_row = edge.iter().map(|(row, _)| row).max().unwrap();
    let min_col = edge.iter().map(|(_, col)| col).min().unwrap();
    let max_col = edge.iter().map(|(_, col)| col).max().unwrap();

    let mut matrix: Matrix<char> = Matrix::from_fn(
        (max_row - min_row + 1) as usize,
        (max_col - min_col + 1) as usize,
        |_| '.',
    );

    area.iter().for_each(|(r, c)| {
        matrix[((r - min_row) as usize, (c - min_col) as usize)] = 'O'
    });
    edge.iter().for_each(|(r, c)| {
        matrix[((r - min_row) as usize, (c - min_col) as usize)] = '#'
    });

    print_matrix(&matrix);

    edge.len() + area.len()
}
