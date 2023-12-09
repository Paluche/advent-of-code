use std::cmp::Ordering;
use std::iter::zip;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

type Card = u8;
type EvalHandItem = (u8, u8);
type Bid = usize;
type EvalHand = Vec<EvalHandItem>;
type Cards = Vec<Card>;
type Hand = (Cards, EvalHand, Bid);

fn compare_hand_items(a: &EvalHandItem, b: &EvalHandItem) -> Ordering {
    let (ac, an) = a;
    let (bc, bn) = b;
    let number_ord = bn.cmp(an);

    if number_ord != Ordering::Equal {
        return number_ord;
    }

    bc.cmp(ac)
}

// Returns the different cards in the hand and the number of time they have been encountered.
fn parse_line(line: &str) -> Hand {
    let (cards, bid) = line.split_once(' ').unwrap();

    let mut hand: EvalHand = CARDS
        .iter()
        .map(|c| {
            (
                CARDS.iter().position(|x| x == c).unwrap() as Card,
                cards.matches(&c.to_string()).count() as u8,
            )
        })
        .filter(|(_, x)| *x != 0)
        .collect();

    hand.sort_by(compare_hand_items);

    let cards = cards
        .chars()
        .map(|c| CARDS.iter().position(|x| *x == c).unwrap() as u8)
        .collect();

    (cards, hand, bid.parse::<Bid>().unwrap())
}

fn load_input(input: &str) -> Vec<Hand> {
    input.lines().map(parse_line).collect()
}

fn compare_hands(h1: &Hand, h2: &Hand) -> Ordering {
    let (c1, eh1, _) = h1;
    let (c2, eh2, _) = h2;

    // The lesser there is different cards in the hand, the highest the value of the hand is.
    // Five of a kind  5         -> 1
    // Four of a kind  4 1       -> 2
    // Full house      3 2       -> 2
    // Three of a kind 3 1 1     -> 3
    // Two pairs       2 2 1     -> 3
    // One pair        2 1 1 1   -> 4
    // High card       1 1 1 1 1 -> 5

    {
        let ord = eh2.len().cmp(&eh1.len());

        if ord != Ordering::Equal {
            return ord;
        }
    }

    // Differentiate the different types which have the same number of cards.
    {
        let ord = eh1[0].1.cmp(&eh2[0].1);

        if ord != Ordering::Equal {
            return ord;
        }
    }

    // Compare the equals, we are comparing two hands of the same type. Search for the highest card
    // by the card type, starting from the first card in the hand.
    for (card1, card2) in zip(c1, c2) {
        let ord = card1.cmp(card2);

        if ord != Ordering::Equal {
            return ord;
        }
    }

    // Definitely, the two hands are equals.
    Ordering::Equal
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut games = load_input(input);

    games.sort_by(compare_hands);

    games
        .iter()
        .map(|(_, _, b)| b)
        .enumerate()
        .map(|(a, b)| (a + 1) * b)
        .sum()
}
