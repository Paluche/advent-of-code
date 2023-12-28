fn expand(s: &[u8]) -> Vec<u8> {
    let mut last: Option<(u8, u8)> = None;
    let mut ret: Vec<u8> = Vec::new();

    for val in s.iter() {
        let mut count: u8 = 1;
        if let Some((last_val, last_count)) = last {
            if last_val == *val {
                count += last_count;
            } else {
                ret.push(last_count);
                ret.push(last_val);
            }
        }

        last = Some((*val, count))
    }

    let (val, count) = last.unwrap();

    ret.push(count);
    ret.push(val);

    ret
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let mut s = parse_input(input);

    for _ in 0..40 {
        s = expand(&s);
    }

    s.len()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut s = parse_input(input);

    for _ in 0..50 {
        s = expand(&s);
    }

    s.len()
}
