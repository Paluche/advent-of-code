use pathfinding::matrix::Matrix;

fn find_entry(matrix: &Matrix<char>) -> Point {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                assert_eq!(*matrix.get((i, j)).unwrap(), 'S');
                return (i, j);
            }
        }
    }

    panic!("No entry point");
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Point = (usize, usize);
type Vector = (i8, i8);

fn next_direction(prev: &Direction, c: &char) -> Option<(Direction, Vector)> {
    // | = │ is a vertical pipe connecting north and south.
    // - = ─ is a horizontal pipe connecting east and west.
    // L = └ is a 90-degree bend connecting north and east.
    // J = ┘ is a 90-degree bend connecting north and west.
    // 7 = ┐ is a 90-degree bend connecting south and west.
    // F = ┌ is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.

    if false {
        match c {
            '|' => match prev {
                Direction::Up => {
                    Some((Direction::Up, /* ↑ → */ (0, 1)))
                }
                Direction::Down => {
                    Some((Direction::Down, /* ↓ ← */ (0, -1)))
                }
                _ => None,
            },
            '-' => match prev {
                Direction::Right => {
                    Some((Direction::Right, /* → ↓ */ (1, 0)))
                }
                Direction::Left => {
                    Some((Direction::Left, /* ← ↑ */ (-1, 0)))
                }
                _ => None,
            },
            'L' => match prev {
                Direction::Left => {
                    Some((Direction::Up, /*   ↗ */ (-1, 1)))
                }
                Direction::Down => {
                    Some((Direction::Right, /* ↳ ↙ */ (1, -1)))
                }
                _ => None,
            },
            'J' => match prev {
                Direction::Right => {
                    Some((Direction::Up, /* ↲ ↘ */ (1, 1)))
                }
                Direction::Down => {
                    Some((Direction::Left, /*   ↖ */ (-1, -1)))
                }
                _ => None,
            },
            '7' => match prev {
                Direction::Up => {
                    Some((Direction::Left, /* ↰ ↗ */ (1, -1)))
                }
                Direction::Right => {
                    Some((Direction::Down, /* ↴ ↙ */ (-1, 1)))
                }
                _ => None,
            },
            'F' => match prev {
                Direction::Up => {
                    Some((Direction::Right, /* ↱ ↘ */ (1, 1)))
                }
                Direction::Left => {
                    Some((Direction::Down, /*   ↖ */ (-1, -1)))
                }
                _ => None,
            },
            '.' => None,
            _ => panic!(),
        }
    } else {
        match c {
            '|' => match prev {
                Direction::Up => {
                    Some((Direction::Up, /* ↑ ← */ (0, -1)))
                }
                Direction::Down => {
                    Some((Direction::Down, /* ↓ → */ (0, 1)))
                }
                _ => None,
            },
            '-' => match prev {
                Direction::Right => {
                    Some((Direction::Right, /* → ↑ */ (-1, 0)))
                }
                Direction::Left => {
                    Some((Direction::Left, /* ← ↓ */ (1, 0)))
                }
                _ => None,
            },
            'L' => match prev {
                Direction::Left => {
                    Some((Direction::Up, /*   ↙ */ (1, -1)))
                }
                Direction::Down => {
                    Some((Direction::Right, /* ↳ ↗ */ (-1, 1)))
                }
                _ => None,
            },
            'J' => match prev {
                Direction::Right => {
                    Some((Direction::Up, /* ↲ ↖ */ (-1, -1)))
                }
                Direction::Down => {
                    Some((Direction::Left, /*   ↘ */ (1, 1)))
                }
                _ => None,
            },
            '7' => match prev {
                Direction::Up => {
                    Some((Direction::Left, /* ↰ ↙ */ (1, -1)))
                }
                Direction::Right => {
                    Some((Direction::Down, /* ↴ ↗ */ (-1, 1)))
                }
                _ => None,
            },
            'F' => match prev {
                Direction::Up => {
                    Some((Direction::Right, /* ↱ ↖ */ (-1, -1)))
                }
                Direction::Left => {
                    Some((Direction::Down, /*   ↘ */ (1, 1)))
                }
                _ => None,
            },
            '.' => None,
            _ => panic!(),
        }
    }
}

fn next_position(start: Point, direction: Direction) -> Option<Point> {
    let (r, c) = start;

    match direction {
        Direction::Up => Some((r.checked_sub(1)?, c)),
        Direction::Down => Some((r + 1, c)),
        Direction::Left => Some((r, c.checked_sub(1)?)),
        Direction::Right => Some((r, c + 1)),
    }
}

fn get_loop(
    matrix: &Matrix<char>,
    start: Point,
) -> Option<Vec<(Point, Vector)>> {
    [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .filter_map(|d| {
        let mut direction = *d;
        let mut cur = next_position(start, *d)?;
        let mut ret: Vec<(Point, Vector)> = Vec::new();
        let mut vector: Vector = (0, 0);

        while cur != start {
            let symbol = matrix.get(cur)?;
            (direction, vector) = next_direction(&direction, symbol)?;

            ret.push((cur, vector));
            cur = next_position(cur, direction)?;
        }

        ret.push((start, vector));

        Some(ret)
    })
    .max_by(|a, b| a.len().cmp(&b.len()))
}

fn get_area(pipe: &[(Point, Vector)]) -> Vec<Point> {
    let mut ret: Vec<Point> = Vec::new();

    let mut rows = pipe.iter().map(|((r, _), _)| r).collect::<Vec<_>>();
    rows.sort();
    rows.dedup();

    for row in rows {
        let mut column_limits: Vec<(usize, i8)> = pipe
            .iter()
            .filter_map(
                |((pr, pc), (_, vc))| {
                    if pr == row {
                        Some((*pc, *vc))
                    } else {
                        None
                    }
                },
            )
            .collect();
        column_limits.sort_by(|(pca, _), (pcb, _)| pca.cmp(pcb));

        for i in 0..column_limits.len() - 1 {
            let (pca, vca) = column_limits[i];
            let (pcb, vcb) = column_limits[i + 1];

            if vca > 0 && vcb < 0 {
                // Between (row, a) and (row, b) is inside the area, enumerate the pixels.
                for x in (pca + 1)..pcb {
                    ret.push((*row, x));
                }
            }
        }
    }

    ret
}

fn parse_input(input: &str) -> (Matrix<char>, Vec<(Point, Vector)>) {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();

    let pipe = get_loop(&matrix, find_entry(&matrix)).unwrap();

    (matrix, pipe)
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    parse_input(input).1.len() / 2
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let (matrix, pipe) = parse_input(input);
    let area: Vec<Point> = get_area(&pipe);

    if false {
        let mut debug: Matrix<char> =
            Matrix::from_fn(matrix.rows, matrix.columns, |_| ' ');

        // Mark area
        for cur in area.iter() {
            *debug.get_mut(*cur).unwrap() = '█';
        }

        for (cur, _) in pipe.iter() {
            *debug.get_mut(*cur).unwrap() = *matrix.get(*cur).unwrap();
        }

        for row in debug.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            println!();
        }
    }

    area.len()
}
