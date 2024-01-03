fn parse_marker(input: &str, index: &mut usize) -> (usize, usize) {
    let mut s = String::new();
    loop {
        let c = &input[*index..(*index + 1)];
        *index += 1;

        if c == ")" {
            break;
        }

        s.push_str(c);
    }

    let (width, count) = s.split_once('x').unwrap();
    let width: usize = width.parse().expect("{}");
    let count: usize = count.parse().expect("{}");

    (width, count)
}

fn decompress(input: &str, part2: bool) -> usize {
    let input = input.lines().next().unwrap();
    let mut index = 0;
    let mut ret = 0;

    while index < input.len() {
        let c = &input[index..(index + 1)];

        index += 1;

        if c == "(" {
            // start of a marker
            let (width, count) = parse_marker(input, &mut index);

            if part2 {
                ret +=
                    count * decompress(&input[index..(index + width)], part2);
            } else {
                ret += count * width;
            }
            index += width;
        } else {
            ret += 1;
        }
    }

    ret
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    decompress(input, false)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    decompress(input, true)
}
