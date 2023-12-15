static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn is_str_digit(s: &str) -> Option<u32> {
    for (i, x) in DIGITS.iter().enumerate() {
        if s.starts_with(x) {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn is_ascii_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

fn get_digits_1(line: &str) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();

    for c in line.chars() {
        if let Some(b) = is_ascii_digit(c) {
            ret.push(b);
        }
    }

    ret
}

fn get_digits_2(line: &str) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if let Some(b) = is_ascii_digit(c).or(is_str_digit(&line[i..])) {
            ret.push(b);
        }
    }

    ret
}

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    let mut ret: Vec<u32> = Vec::new();

    for line in input.lines() {
        let digits = get_digits_1(line);

        ret.push(digits[0] * 10 + digits.last().unwrap());
    }

    let total: u32 = ret.iter().sum();

    total as usize
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut ret: Vec<u32> = Vec::new();

    for line in input.lines() {
        let digits = get_digits_2(line);

        ret.push(digits[0] * 10 + digits.last().unwrap());
    }

    let total: u32 = ret.iter().sum();

    total as usize
}
