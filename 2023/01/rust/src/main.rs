use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_digits(chars: std::str::Chars) -> Vec<u32> {
    let mut ret:Vec<u32> = Vec::new();

    for byte in chars {
        if byte.is_ascii_digit() {
            ret.push(byte.to_digit(10).unwrap())
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
        let digits = get_digits(line.chars());

        ret.push(digits[0] * 10 + digits.last().unwrap());
    }

    let total: u32 = ret.iter().sum();
    println!("{}", total);

    Ok(())
}
