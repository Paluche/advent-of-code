use pathfinding::matrix::{
    directions::{E, N, S, W},
    Matrix,
};

fn parse_input(input: &str) -> Vec<Vec<(isize, isize)>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'U' => N,
                    'L' => W,
                    'R' => E,
                    'D' => S,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    let keypad = Matrix::from_fn(3, 3, |(r, c)| r * 3 + (c + 1));
    let mut ret = 0;
    let mut pos: (usize, usize) = (1, 1);

    for instruction in instructions {
        for direction in instruction {
            if let Some(next) = keypad.move_in_direction(pos, direction) {
                pos = next;
            }
        }
        ret *= 10;
        ret += keypad[pos];
    }

    ret
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let keypad = Matrix::from_fn(5, 5, |(r, c)| match r {
        0 => match c {
            0 => None,
            1 => None,
            2 => Some('1'),
            3 => None,
            4 => None,
            _ => panic!(),
        },
        1 => match c {
            0 => None,
            1 => Some('2'),
            2 => Some('3'),
            3 => Some('4'),
            4 => None,
            _ => panic!(),
        },
        2 => match c {
            0 => Some('5'),
            1 => Some('6'),
            2 => Some('7'),
            3 => Some('8'),
            4 => Some('9'),
            _ => panic!(),
        },
        3 => match c {
            0 => None,
            1 => Some('A'),
            2 => Some('B'),
            3 => Some('C'),
            4 => None,
            _ => panic!(),
        },
        4 => match c {
            0 => None,
            1 => None,
            2 => Some('D'),
            3 => None,
            4 => None,
            _ => panic!(),
        },
        _ => panic!(),
    });
    let mut ret = String::new();
    let mut pos: (usize, usize) = (2, 0);

    assert_eq!(keypad[pos], Some('5'));

    for instruction in instructions {
        for direction in instruction {
            if let Some(next) = keypad.move_in_direction(pos, direction) {
                if keypad[next].is_some() {
                    pos = next;
                }
            }
        }
        ret.push(keypad[pos].unwrap());
    }

    ret
}
