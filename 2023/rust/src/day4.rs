use crate::utils::parse_numbers;
// Card   <id>: 10 numbers, each 2 digits | x (25) numbers 2 digits each.
// Number could start with a space as first digit meaning 0.

// Returns the number of winning numbers.
fn parse_card(line: &str) -> usize {
    let (winning, selected) =
        line.split_once(':').unwrap().1.split_once('|').unwrap();

    let winning = parse_numbers::<u8>(winning);

    parse_numbers(selected)
        .iter()
        .filter(|n| winning.contains(n))
        .count()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let n = parse_card(l);
            if n != 0 {
                1 << (n - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut cards_copies = vec![1; input.lines().count()];

    for (i, result) in input.lines().map(parse_card).enumerate() {
        let cur_copies = cards_copies[i];

        for j in i + 1..i + 1 + result {
            if j >= cards_copies.len() {
                break;
            }

            cards_copies[j] += cur_copies;
        }
    }

    cards_copies.iter().sum()
}
