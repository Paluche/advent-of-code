use std::collections::HashSet;

enum Direction {
    N,
    S,
    E,
    W,
}

enum Turn {
    L,
    R,
}

impl Turn {
    fn new(c: char) -> Self {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<(isize, (isize, isize))> {
    let mut direction: Direction = Direction::N;

    input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| {
            let mut s = s.chars();
            let turn = Turn::new(s.next().unwrap());

            let mut range = 0;

            for c in s {
                range *= 10;
                range += c.to_digit(10).unwrap() as isize;
            }

            (
                range,
                match direction {
                    Direction::N => match turn {
                        Turn::L => {
                            direction = Direction::W;
                            (0, -1)
                        }
                        Turn::R => {
                            direction = Direction::E;
                            (0, 1)
                        }
                    },
                    Direction::S => match turn {
                        Turn::L => {
                            direction = Direction::E;
                            (0, 1)
                        }
                        Turn::R => {
                            direction = Direction::W;
                            (0, -1)
                        }
                    },
                    Direction::E => match turn {
                        Turn::L => {
                            direction = Direction::N;
                            (-1, 0)
                        }
                        Turn::R => {
                            direction = Direction::S;
                            (1, 0)
                        }
                    },
                    Direction::W => match turn {
                        Turn::L => {
                            direction = Direction::S;
                            (1, 0)
                        }
                        Turn::R => {
                            direction = Direction::N;
                            (-1, 0)
                        }
                    },
                },
            )
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    let (mut row, mut col): (isize, isize) = (0, 0);

    for (range, (r, c)) in parse_input(input) {
        row += range * r;
        col += range * c;
    }

    row.unsigned_abs() + col.unsigned_abs()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let (mut row, mut col): (isize, isize) = (0, 0);
    let mut path: HashSet<(isize, isize)> = HashSet::new();

    for (range, (r, c)) in parse_input(input) {
        for _ in 0..range {
            row += r;
            col += c;
            if !path.insert((row, col)) {
                return row.unsigned_abs() + col.unsigned_abs();
            }
        }
    }

    panic!()
}
