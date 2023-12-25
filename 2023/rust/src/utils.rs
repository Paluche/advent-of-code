use core::fmt::Display;
use num::Integer;
use pathfinding::matrix::Matrix;

pub fn parse_numbers<T: std::str::FromStr>(s: &str) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();

    for number in s.split_whitespace() {
        if let Ok(number) = number.parse::<T>() {
            ret.push(number)
        }
    }

    ret
}

pub fn print_matrix<T: Display>(matrix: &Matrix<T>) {
    println!("{}x{}", matrix.rows, matrix.columns);

    for row in matrix {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}

pub fn shoelace<T>(points: &[(T, T)]) -> usize
where
    T: TryInto<isize> + std::ops::Mul + std::ops::Sub + Copy,
{
    (points
        .windows(2)
        .map(|w| {
            w[0].1.try_into().ok().unwrap() * w[1].0.try_into().ok().unwrap()
                - w[0].0.try_into().ok().unwrap()
                    * w[1].1.try_into().ok().unwrap()
        })
        .sum::<isize>()
        / 2_isize)
        .unsigned_abs()
}
