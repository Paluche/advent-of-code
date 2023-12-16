use pathfinding::matrix::Matrix;

fn search_row_mirror(matrix: &Matrix<char>, smudge_count: u8) -> usize {
    'a: for i in 1..matrix.rows {
        let width = i.min(matrix.rows - i);
        let mut smudge = 0;

        for r in 0..width {
            for c in 0..matrix.columns {
                if matrix.get((i - 1 - r, c)).unwrap()
                    != matrix.get((i + r, c)).unwrap()
                {
                    smudge += 1;
                    if smudge > smudge_count {
                        continue 'a;
                    }
                }
            }
        }

        if smudge == smudge_count {
            return i;
        }
    }
    0
}

fn search_col_mirror(matrix: &Matrix<char>, smudge_count: u8) -> usize {
    'a: for i in 1..matrix.columns {
        let width = i.min(matrix.columns - i);
        let mut smudge = 0;

        for c in 0..width {
            for r in 0..matrix.rows {
                if matrix.get((r, i - 1 - c)).unwrap()
                    != matrix.get((r, i + c)).unwrap()
                {
                    smudge += 1;
                    if smudge > smudge_count {
                        continue 'a;
                    }
                }
            }
        }

        if smudge == smudge_count {
            return i;
        }
    }
    0
}

fn parse_input(input: &str) -> Vec<Matrix<char>> {
    input
        .split("\n\n")
        .map(|i| {
            Matrix::from_rows(
                i.lines().take_while(|x| !x.is_empty()).map(|l| l.chars()),
            )
            .unwrap()
        })
        .collect()
}

fn run(input: &str, smudge_count: u8) -> usize {
    parse_input(input)
        .iter()
        .map(|m| {
            search_col_mirror(m, smudge_count)
                + 100 * search_row_mirror(m, smudge_count)
        })
        .sum()
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    run(input, 0)
}

#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
    run(input, 1)
}
