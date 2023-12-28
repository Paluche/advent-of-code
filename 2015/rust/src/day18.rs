use pathfinding::matrix::Matrix;

fn parse_input(input: &str, part2: bool) -> Matrix<bool> {
    let mut ret = Matrix::from_rows(
        input.lines().map(|l| l.chars().map(|c| matches!(c, '#'))),
    )
    .unwrap();

    if part2 {
        for r in [0, ret.rows - 1] {
            for c in [0, ret.columns - 1] {
                ret[(r, c)] = true;
            }
        }
    }

    ret
}

fn is_corner((row, col): (usize, usize), grid: &Matrix<bool>) -> bool {
    ((row == 0) || (row == grid.rows - 1))
        && ((col == 0) || (col == grid.columns - 1))
}

fn next(grid: &Matrix<bool>, part2: bool) -> Matrix<bool> {
    Matrix::from_fn(grid.rows, grid.columns, |pos| {
        if part2 && is_corner(pos, grid) {
            true
        } else {
            match grid.neighbours(pos, true).filter(|&x| grid[x]).count() {
                2 => grid[pos],
                3 => true,
                _ => false,
            }
        }
    })
}

fn run(input: &str, part2: bool) -> usize {
    let mut grid = parse_input(input, part2);

    for _ in 0..100 {
        grid = next(&grid, part2);
    }

    grid.iter().map(|r| r.iter().filter(|&c| *c).count()).sum()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    run(input, false)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
    run(input, true)
}
