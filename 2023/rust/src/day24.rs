use std::fmt::{Display, Formatter};

use nalgebra::matrix;

fn parse_numbers<T: std::str::FromStr>(s: &str) -> (T, T, T) {
    let mut s = s.split(',');
    (
        s.next().unwrap().trim().parse::<T>().ok().unwrap(),
        s.next().unwrap().trim().parse::<T>().ok().unwrap(),
        s.next().unwrap().trim().parse::<T>().ok().unwrap(),
    )
}

#[derive(Debug, Clone, Copy)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hail {
    fn new(line: &str) -> Self {
        let (pos, dir) = line.split_once(" @ ").unwrap();
        let (x, y, z) = parse_numbers::<f64>(pos);
        let (a, b, c) = parse_numbers::<f64>(dir);
        Self { x, y, z, a, b, c }
    }

    fn cross(&self, other: &Self) -> Option<(f64, f64)> {
        let a = matrix![self.b, -self.a;
                        other.b, -other.a];
        let b = matrix![self.b * self.x - self.a * self.y;
                        other.b * other.x - other.a * other.y];

        if let Some(a_inv) = a.try_inverse() {
            let c = a_inv * b;
            let ret = (c[(0, 0)], c[(1, 0)]);

            if self.in_future(ret) && other.in_future(ret) {
                Some(ret)
            } else {
                None
            }
        } else if self.b * (other.x - self.x) - self.a * (other.y - self.y)
            == 0.0
        {
            panic!()
        } else {
            None
        }
    }

    fn in_future(&self, (x, y): (f64, f64)) -> bool {
        if self.a == 0.0 {
            (y - self.y) / self.b > 0.0
        } else {
            (x - self.x) / self.a > 0.0
        }
    }
}

impl Display for Hail {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.x, self.y, self.z, self.a, self.b, self.c
        )
    }
}

fn inside((x, y): (f64, f64), min: f64, max: f64) -> bool {
    min <= x && x <= max && min <= y && y <= max
}

fn parse_input(input: &str) -> Vec<Hail> {
    input.lines().map(Hail::new).collect()
}

#[aoc(day24, part1)]
fn part1(input: &str) -> usize {
    let hails = parse_input(input);
    let mut ret = 0;
    let min = 200000000000000_f64;
    let max = 400000000000000_f64;

    for (i, a) in hails[..hails.len() - 1].iter().enumerate() {
        for b in hails[(i + 1)..].iter() {
            if let Some(x) = a.cross(b) {
                if inside(x, min, max) {
                    ret += 1;
                }
            }
        }
    }

    ret
}
