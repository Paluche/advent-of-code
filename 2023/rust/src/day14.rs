use pathfinding::directed::cycle_detection::brent;
use pathfinding::matrix::Matrix;

fn tilt_north(dish: &mut Matrix<char>) {
    for c in 0..dish.columns {
        let mut rs: Option<usize> = None;
        for r in 0..dish.rows {
            match dish[(r, c)] {
                '.' => (),
                '#' => rs = Some(r),
                'O' => match rs {
                    Some(x) => {
                        let n = x + 1;
                        if r != n {
                            dish[(n, c)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        rs = Some(n);
                    }
                    None => {
                        if r != 0 {
                            dish[(0, c)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        rs = Some(0);
                    }
                },
                _ => panic!(),
            }
        }
    }
}

fn tilt_south(dish: &mut Matrix<char>) {
    let limit = dish.rows - 1;
    for c in 0..dish.columns {
        let mut rs: Option<usize> = None;
        for r in (0..dish.rows).rev() {
            match dish[(r, c)] {
                '.' => (),
                '#' => rs = Some(r),
                'O' => match rs {
                    Some(x) => {
                        let n = x - 1;
                        if r != n {
                            dish[(n, c)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        rs = Some(n);
                    }
                    None => {
                        if r != limit {
                            dish[(limit, c)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        rs = Some(limit);
                    }
                },
                _ => panic!(),
            }
        }
    }
}

fn tilt_west(dish: &mut Matrix<char>) {
    for r in 0..dish.rows {
        let mut cs: Option<usize> = None;
        for c in 0..dish.columns {
            match dish[(r, c)] {
                '.' => (),
                '#' => cs = Some(c),
                'O' => match cs {
                    Some(x) => {
                        let n = x + 1;
                        if c != n {
                            dish[(r, n)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        cs = Some(n);
                    }
                    None => {
                        if c != 0 {
                            dish[(r, 0)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        cs = Some(0);
                    }
                },
                _ => panic!(),
            }
        }
    }
}

fn tilt_east(dish: &mut Matrix<char>) {
    let limit = dish.columns - 1;
    for r in 0..dish.rows {
        let mut cs: Option<usize> = None;
        for c in (0..dish.columns).rev() {
            match dish[(r, c)] {
                '.' => (),
                '#' => cs = Some(c),
                'O' => match cs {
                    Some(x) => {
                        let n = x - 1;
                        if c != n {
                            dish[(r, n)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        cs = Some(n);
                    }
                    None => {
                        if c != limit {
                            dish[(r, limit)] = dish[(r, c)];
                            dish[(r, c)] = '.';
                        }
                        cs = Some(limit);
                    }
                },
                _ => panic!(),
            }
        }
    }
}

fn cycle(dish: &mut Matrix<char>) {
    tilt_north(dish);
    tilt_west(dish);
    tilt_south(dish);
    tilt_east(dish);
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
    let mut dish: Matrix<char> = Matrix::from_rows(
        input
            .lines()
            .take_while(|x| !x.is_empty())
            .map(|l| l.chars()),
    )
    .unwrap();
    tilt_north(&mut dish);
    count_load(&dish)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let dish: Matrix<char> = Matrix::from_rows(
        input
            .lines()
            .take_while(|x| !x.is_empty())
            .map(|l| l.chars()),
    )
    .unwrap();

    const CYCLES: usize = 1_000_000_000;
    // Doing the billion cycle will take too much time. The dish patterns must
    // have some kind of cycle. Find out which it is, then compute only the
    // required steps.
    // Using brent's algorithm.
    let (size, mut dish, start) = brent(dish, |mut d| {
        cycle(&mut d);
        d
    });

    for _ in 0..(CYCLES - start) % (size) {
        cycle(&mut dish);
    }
    count_load(&dish)
}
