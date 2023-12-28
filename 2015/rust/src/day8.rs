fn unescape(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let mut iter = s.chars();

    while let Some(c) = iter.next() {
        match c {
            '\\' => match iter.next().unwrap() {
                'x' => {
                    iter.next().unwrap();
                    iter.next().unwrap();
                    ret.push('x');
                }
                '"' => ret.push('"'),
                '\\' => ret.push('\\'),
                _ => panic!(),
            },
            '"' =>
                /* start/ end of the string */
                {}
            x => ret.push(x),
        }
    }

    ret
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(|l| l.len() - unescape(l).len()).sum()
}

fn escape(s: &str) -> String {
    let mut ret = String::with_capacity(2 * s.len());

    ret.push('\"');

    for c in s.chars() {
        match c {
            '\\' => {
                ret.push('\\');
                ret.push('\\');
            }
            '"' => {
                ret.push('\\');
                ret.push('"');
            }
            x => ret.push(x),
        }
    }

    ret.push('\"');
    ret
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input.lines().map(|l| escape(l).len() - l.len()).sum()
}
