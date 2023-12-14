use pathfinding::matrix::Matrix;

fn tilt_north(dish: &Matrix<char>) -> Matrix<char> {
    let mut ret: Matrix<char> = Matrix::from_fn(dish.rows,
        dish.columns,
        |(r, c)| match dish[(r, c)] { '#' => '#', _ => '.'});
    for r in 0..dish.rows {
        'a: for c in 0..dish.columns {
            if dish[(r, c)] == 'O' {
                for r_ in (1..=r).rev() {
                    if ret[(r_ -1, c)] =='#' || ret[(r_ -1, c)] =='O' {
                        ret[(r_, c)] = 'O';
                        continue 'a;
                    }
                }
                ret[(0, c)] = 'O';
            }
        }
    }

    ret
}

fn count_load(dish: &Matrix<char>) -> usize {
    let mut ret: usize = 0;

    for r in 0..dish.rows {
        for c in 0..dish.columns {
            if dish[(r, c)] == 'O' {
                ret += dish.rows - r;
            }
        }
    }

    ret
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    count_load(&tilt_north(&Matrix::from_rows(
                input.lines().take_while(|x| *x != "").map(|l| l.chars()),
                ).unwrap()))
}
