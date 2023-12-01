use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

static DIGITS: [&str ; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn is_str_digit(s: &str) -> Option<u32> {
    for (i, x) in DIGITS.iter().enumerate() {
        if s.starts_with(x) {
            return Some (i as u32 + 1)
        }
    }
    None
}

fn is_ascii_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

fn get_digits(line: &str) -> Vec<u32> {
    let mut ret:Vec<u32> = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if let Some(b) = is_ascii_digit(c).or(is_str_digit(&line[i..])) {
            ret.push(b);
        }
    }

    ret
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given.");

    let lines = read_lines(input_path).unwrap();

    let mut ret: Vec<u32> = Vec::new();

    for line in lines {
        let line = line?;
        let digits = get_digits(&line);

        ret.push(digits[0] * 10 + digits.last().unwrap());
    }

    let total: u32 = ret.iter().sum();
    println!("{}", total);

    Ok(())
}
