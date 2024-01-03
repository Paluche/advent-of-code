fn run<F>(input: &str, mut f: F) -> usize
where
    F: FnMut(&[Vec<char>], &[Vec<char>]) -> bool,
{
    input
        .lines()
        .map(|l| {
            let mut a = Vec::new();
            let mut b = Vec::new();
            l.split('[').for_each(|s| {
                if let Some((x, y)) = s.split_once(']') {
                    b.push(x.chars().collect());
                    a.push(y.chars().collect());
                } else {
                    a.push(s.chars().collect());
                }
            });

            (a, b)
        })
        .filter(|(a, b)| f(a, b))
        .count()
}

fn contains_abba(input: &[char]) -> bool {
    input
        .windows(4)
        .any(|w| w[0] != w[1] && w[0] == w[3] && w[1] == w[2])
}

fn part1_check(a: &[Vec<char>], b: &[Vec<char>]) -> bool {
    a.iter().any(|x| contains_abba(x)) && !b.iter().any(|x| contains_abba(x))
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    run(input, part1_check)
}

struct BabIter {
    input: Vec<char>,
    index: usize,
}

impl BabIter {
    fn new(input: &[char]) -> Self {
        Self {
            input: input.to_vec(),
            index: 0,
        }
    }
}

impl Iterator for BabIter {
    type Item = (char, char);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        for (i, w) in self.input[self.index..].windows(3).enumerate() {
            if w[0] != w[1] && w[0] == w[2] {
                self.index += i + 1;
                return Some((w[0], w[1]));
            }
        }

        self.index = self.input.len();
        None
    }
}

fn contains_bab(input: &[char], a: char, b: char) -> bool {
    input
        .windows(3)
        .any(|w| w[0] == a && w[1] == b && w[0] == w[2])
}

fn part2_check(
    outside_brackets: &[Vec<char>],
    inside_brackets: &[Vec<char>],
) -> bool {
    for a in outside_brackets {
        for (x, y) in BabIter::new(a) {
            for b in inside_brackets {
                if contains_bab(b, y, x) {
                    return true;
                }
            }
        }
    }

    false
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    run(input, part2_check)
}
