use pathfinding::matrix::Matrix;
use core::fmt::Display;

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
