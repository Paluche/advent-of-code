use crate::utils;
// Card   <id>: 10 numbers, each 2 digits | x (25) numbers 2 digits each.
// Number could start with a space as first digit meaning 0.

// Returns the number of winning numbers.
fn parse_card(line: &str) -> usize {
    let (_card_id, results) = line.split_once(':').unwrap();
    let (winning, selected) = results.split_once('|').unwrap();

    let winning = utils::parse_numbers::<u8>(winning);
    utils::parse_numbers(selected)
        .iter()
        .filter(|n| winning.contains(n))
        .count()
}

fn part1_res(line: &str) -> usize {
    let n = parse_card(line);
    if n != 0 {
        return 1 << (n - 1);
    }

    0
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(part1_res).sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let cards_results = input.lines().map(parse_card);
    let mut cards_copies = vec![1; input.lines().count()];

    for (i, result) in cards_results.enumerate() {
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
