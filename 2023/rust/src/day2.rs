// In the bag only 12 red cubes, 13 green cubes, and 14 blue cubes.
static RED_CUBES_MAX: usize = 12;
static GREEN_CUBES_MAX: usize = 13;
static BLUE_CUBES_MAX: usize = 14;

fn str_to_usize(s: &str) -> Option<usize> {
    match s.parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn is_doable(line: &str) -> Option<usize> {
    // Split game title, from content.
    let (key, value) = line.split_once(':')?;
    // Skip string "Game "
    let i = "Game ".len();
    let game_number = str_to_usize(&key[i..]);

    // Split the content in sets.
    for set in value.split(';') {
        if !check_set(set)? {
            return Some(0);
        }
    }

    game_number
}

fn check_set(line: &str) -> Option<bool> {
    for s in line.split(',') {
        let (count, id) = s[1..].split_once(' ')?;
        let count = str_to_usize(count)?;

        if id == "red" && count > RED_CUBES_MAX {
            return Some(false);
        }

        if id == "green" && count > GREEN_CUBES_MAX {
            return Some(false);
        }

        if id == "blue" && count > BLUE_CUBES_MAX {
            return Some(false);
        }
    }

    Some(true)
}

fn minimal_cubes(line: &str) -> Option<usize> {
    // Remove game title, from content.
    let start = line.find(':')? + 1;
    let value = &line[start..];

    // Split the content in sets.
    let mut res: (usize, usize, usize) = (0, 0, 0);
    for set in value.split(';') {
        let (red, green, blue) = used_balls(set)?;

        if res.0 < red {
            res.0 = red;
        }

        if res.1 < green {
            res.1 = green;
        }

        if res.2 < blue {
            res.2 = blue;
        }
    }

    Some(res.0 * res.1 * res.2)
}

fn used_balls(line: &str) -> Option<(usize, usize, usize)> {
    let mut ret: (usize, usize, usize) = (0, 0, 0);

    for s in line.split(',') {
        let (count, id) = s[1..].split_once(' ')?;
        let count = str_to_usize(count)?;

        if id == "red" {
            ret.0 = count
        }

        if id == "green" {
            ret.1 = count
        }

        if id == "blue" {
            ret.2 = count
        }
    }

    Some(ret)
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| -> usize { is_doable(l).unwrap() })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| -> usize { minimal_cubes(l).unwrap() })
        .sum()
}
