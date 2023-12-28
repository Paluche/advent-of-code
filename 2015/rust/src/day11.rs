// 8 lowercase letters
// in
fn next(c: char, overlap: bool) -> Option<char> {
    Some(match c {
        'a' => 'b',
        'b' => 'c',
        'c' => 'd',
        'd' => 'e',
        'e' => 'f',
        'f' => 'g',
        'g' => 'h',
        'h' => 'i',
        'i' => 'j',
        'j' => 'k',
        'k' => 'l',
        'l' => 'm',
        'm' => 'n',
        'n' => 'o',
        'o' => 'p',
        'p' => 'q',
        'q' => 'r',
        'r' => 's',
        's' => 't',
        't' => 'u',
        'u' => 'v',
        'v' => 'w',
        'w' => 'x',
        'x' => 'y',
        'y' => 'z',
        'z' => {
            if !overlap {
                return None;
            } else {
                'a'
            }
        }
        _ => panic!(),
    })
}

fn is_password_valid(password: &[char]) -> bool {
    // Passwords must include one increasing straight of at least three 4
    // letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip
    // letters; abd doesn't count.
    let mut ok = false;
    'main: for w in password.windows(3) {
        for w in w.windows(2) {
            if let Some(x) = next(w[0], false) {
                if x != w[1] {
                    continue 'main;
                }
            } else {
                continue 'main;
            }
        }

        ok = true;
        break;
    }

    if !ok {
        return false;
    }

    // Passwords may not contain the letters i, o, or l, as these letters can
    // be mistaken for other characters and are therefore confusing.

    for c in password {
        if matches!(c, 'i' | 'o' | 'l') {
            return false;
        }
    }

    // Passwords must contain at least two different, non-overlapping pairs of
    // letters, like aa, bb, or zz
    let mut count = 0;
    let mut skip = false;
    for w in password.windows(2) {
        if skip {
            skip = false;
            continue;
        }

        if w[0] == w[1] {
            count += 1;
            skip = true;
        }
    }

    count == 2
}

fn increment_password(password: &mut [char]) {
    for c in password.iter_mut().rev() {
        *c = next(*c, true).unwrap();

        if *c != 'a' {
            break;
        }
    }
}

fn parse_input(input: &str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

fn vec_char_to_string(password: &[char]) -> String {
    let mut ret = String::with_capacity(password.len());

    password.iter().for_each(|c| ret.push(*c));

    ret
}

fn next_password(password: &mut [char]) {
    increment_password(password);

    while !is_password_valid(password) {
        increment_password(password);
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    let mut password = parse_input(input);

    next_password(&mut password);

    vec_char_to_string(&password)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    let mut password = parse_input(input);

    next_password(&mut password);
    next_password(&mut password);

    vec_char_to_string(&password)
}
