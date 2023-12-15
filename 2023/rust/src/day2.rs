// In the bag only 12 red cubes, 13 green cubes, and 14 blue cubes.
static MAX: [(&str, usize); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn str_to_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn is_doable(line: &str) -> usize {
    let (key, value) = line.split_once(':').unwrap();

    if value.split(';').any(set_fails) {
        0
    } else {
        str_to_usize(&key["Game ".len()..])
    }
}

fn set_fails(line: &str) -> bool {
    line.split(',').any(|s| {
        let (count, id) = s[1..].split_once(' ').unwrap();
        let count = str_to_usize(count);
        MAX.iter().any(|(c, m)| id == *c && count > *m)
    })
}

fn minimal_cubes(line: &str) -> usize {
    line.split_once(':')
        .unwrap()
        .1
        .split(';')
        .fold([0; 3], |mut x, set| {
            for (i, color) in used_balls(set).iter().enumerate() {
                if x[i] < *color {
                    x[i] = *color;
                }
            }
            x
        })
        .iter()
        .product()
}

fn used_balls(line: &str) -> [usize; 3] {
    line.split(',').fold([0; 3], |mut x, s| {
        let (count, id) = s[1..].split_once(' ').unwrap();
        x[match id {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => panic!(),
        }] = str_to_usize(count);
        x
    })
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(is_doable).sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input.lines().map(minimal_cubes).sum()
}
