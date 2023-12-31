use pathfinding::matrix::Matrix;

type Position = (usize, usize);
type Direction = (isize, isize);

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(
        input
            .lines()
            .take_while(|x| !x.is_empty())
            .map(|l| l.chars()),
    )
    .unwrap()
}

fn go(matrix: &Matrix<char>, start: Position, direction: Direction) -> usize {
    let mut energized = Matrix::from_fn(matrix.rows, matrix.columns, |_| 0);
    let mut cache: Vec<(Position, Direction)> = Vec::new();

    be_a_beam(&mut energized, matrix, &mut cache, start, direction);

    energized
        .iter()
        .map(|r| r.iter().filter(|x| **x > 0).count())
        .sum()
}

fn be_a_beam(
    energized: &mut Matrix<u8>,
    matrix: &Matrix<char>,
    cache: &mut Vec<(Position, Direction)>,
    start: Position,
    direction: Direction,
) {
    if cache.contains(&(start, direction)) {
        return;
    }

    cache.push((start, direction));

    let mut pos = start;
    let mut direction = direction;

    loop {
        energized[pos] += 1;

        match matrix[pos] {
            '.' => (),
            '/' => direction = (-direction.1, -direction.0),
            '\\' => direction = (direction.1, direction.0),
            '|' => match direction {
                (1, 0) | (-1, 0) => (),
                (0, 1) | (0, -1) => {
                    be_a_beam(energized, matrix, cache, pos, (1, 0));
                    be_a_beam(energized, matrix, cache, pos, (-1, 0));
                    return;
                }
                _ => panic!(),
            },
            '-' => match direction {
                (1, 0) | (-1, 0) => {
                    be_a_beam(energized, matrix, cache, pos, (0, 1));
                    be_a_beam(energized, matrix, cache, pos, (0, -1));
                    return;
                }
                (0, 1) | (0, -1) => (),
                _ => panic!(),
            },
            _ => panic!(),
        }

        let next_row = pos.0 as isize + direction.0;
        let next_col = pos.1 as isize + direction.1;

        if next_row < 0
            || next_row as usize >= matrix.rows
            || next_col < 0
            || next_col as usize >= matrix.columns
        {
            return;
        }

        pos = (next_row as usize, next_col as usize);
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    go(&parse_input(input), (0, 0), (0, 1))
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    let matrix = parse_input(input);

    (0..matrix.rows)
        .map(|r| {
            go(&matrix, (r, 0), (0, 1)).max(go(
                &matrix,
                (r, matrix.columns - 1),
                (0, -1),
            ))
        })
        .max()
        .unwrap()
        .max(
            (0..matrix.columns)
                .map(|c| {
                    go(&matrix, (0, c), (1, 0)).max(go(
                        &matrix,
                        (matrix.rows - 1, c),
                        (-1, 0),
                    ))
                })
                .max()
                .unwrap(),
        )
}
