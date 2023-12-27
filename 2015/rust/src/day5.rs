fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_nice_1(s: &str) -> bool {
    let mut vowels = 0;
    let mut double_letters = 0;
    let chars: Vec<char> = s.chars().collect();

    for w in chars.windows(2) {
        let (a, b) = (w[0], w[1]);

        if is_vowel(a) {
            vowels += 1;
        }

        if a == b {
            double_letters += 1;
        }

        if matches!((a, b), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')) {
            return false;
        }
    }

    if is_vowel(*chars.last().unwrap()) {
        vowels += 1;
    }

    (vowels >= 3) && double_letters >= 1
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice_1(l)).count()
}

fn is_nice_2(s: &str) -> bool {
    let mut cond1 = false;
    let chars: Vec<char> = s.chars().collect();

    for (i, w) in chars.windows(2).enumerate() {
        for x in chars[i + 2..].windows(2) {
            if w == x {
                cond1 = true;
                break;
            }
        }
    }

    if !cond1 {
        return false;
    }

    for w in chars.windows(3) {
        if w[0] == w[2] {
            return true;
        }
    }

    false
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_nice_2(l)).count()
}
