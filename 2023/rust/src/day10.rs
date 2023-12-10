use pathfinding::matrix::Matrix;

fn find_entry(matrix: &Matrix<char>) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                assert!(*matrix.get((i, j)).unwrap() == 'S');
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

fn next_direction(prev: Direction, c: &char) -> Option<Direction> {
    // | = │ is a vertical pipe connecting north and south.
    // - = ─ is a horizontal pipe connecting east and west.
    // L = └ is a 90-degree bend connecting north and east.
    // J = ┘ is a 90-degree bend connecting north and west.
    // 7 = ┐ is a 90-degree bend connecting south and west.
    // F = ┌ is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.

    match c {
        '|' => match prev {
            Direction::Up => Some(Direction::Up),
            Direction::Down => Some(Direction::Down),
            _ => None,
        },
        '-' => match prev {
            Direction::Left => Some(Direction::Left),
            Direction::Right => Some(Direction::Right),
            _ => None,
        },
        'L' => match prev {
            Direction::Down => Some(Direction::Right),
            Direction::Left => Some(Direction::Up),
            _ => None,
        },
        'J' => match prev {
            Direction::Down => Some(Direction::Left),
            Direction::Right => Some(Direction::Up),
            _ => None,
        },
        '7' => match prev {
            Direction::Up => Some(Direction::Left),
            Direction::Right => Some(Direction::Down),
            _ => None,
        },
        'F' => match prev {
            Direction::Up => Some(Direction::Right),
            Direction::Left => Some(Direction::Down),
            _ => None,
        },
        '.' => None,
        _ => panic!(),
    }
}

fn next_position(start: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let (r, c) = start;

    match direction {
        Direction::Up => Some((r.checked_sub(1)?, c)),
        Direction::Down => Some((r + 1, c)),
        Direction::Left => Some((r, c.checked_sub(1)?)),
        Direction::Right => Some((r, c + 1)),
    }
}

fn follow_pipe(
    matrix: &Matrix<char>,
    start: (usize, usize),
    direction: Direction,
) -> Option<Vec<(usize, usize)>> {
    let mut direction = direction;
    let mut ret: Vec<(usize, usize)> = vec![start];
    let mut cur = next_position(start, direction)?;

    while cur != start {
        direction = next_direction(direction, matrix.get(cur)?)?;
        cur = next_position(cur, direction)?;
        ret.push(cur);
    }

    Some(ret)
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();

    let start = find_entry(&matrix);

    [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .filter_map(|d| follow_pipe(&matrix, start, *d))
    .max_by(|a, b| a.len().cmp(&b.len()))
    .unwrap()
    .len()
        / 2
}
