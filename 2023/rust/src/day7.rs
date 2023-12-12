use std::cmp::Ordering;
use std::iter::zip;

type Card = u8;
type EvalHandItem = (u8, u8);
type Bid = usize;
type EvalHand = Vec<EvalHandItem>;
type Cards = Vec<Card>;
type Hand = (Cards, EvalHand, Bid);
type CardOrder = [char; 13];

const CARD_ORDER_1: CardOrder = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_ORDER_2: CardOrder = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn compare_hand_items(a: &EvalHandItem, b: &EvalHandItem) -> Ordering {
    let (ac, an) = a;
    let (bc, bn) = b;
    let number_ord = bn.cmp(an);

    if number_ord != Ordering::Equal {
        return number_ord;
    }

    bc.cmp(ac)
}

fn find_joker(eval_hand: &EvalHand) -> Option<(usize, u8)> {
    for (i, (c, n)) in eval_hand.iter().enumerate() {
        if *c == 0 {
            return Some((i, *n));
        }
    }

    None
}

// Returns the different cards in the hand and the number of time they have been encountered.
fn parse_line(line: &str, j_as_joker: bool) -> Hand {
    let (cards, bid) = line.split_once(' ').unwrap();
    let card_order = if j_as_joker {
        CARD_ORDER_2
    } else {
        CARD_ORDER_1
    };

    let mut eval_hand: EvalHand = card_order
        .iter()
        .map(|c| {
            (
                card_order.iter().position(|x| x == c).unwrap() as Card,
                cards.matches(&c.to_string()).count() as u8,
            )
        })
        .filter(|(_, x)| *x != 0)
        .collect();

    eval_hand.sort_by(compare_hand_items);

    if j_as_joker {
        if let Some((i, n)) = find_joker(&eval_hand) {
            if n != 5 {
                // Other all the cards were jokers, there was no optimization to do.
                eval_hand.remove(i);
                eval_hand[0].1 += n;
            }
        }
    }

    let cards = cards
        .chars()
        .map(|c| card_order.iter().position(|x| *x == c).unwrap() as u8)
        .collect();

    (cards, eval_hand, bid.parse::<Bid>().unwrap())
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

fn run(input: &str, j_as_joker: bool) -> usize {
    let mut hands: Vec<Hand> =
        input.lines().map(|l| parse_line(l, j_as_joker)).collect();

    hands.sort_by(compare_hands);

    hands
        .iter()
        .map(|(_, _, b)| b)
        .enumerate()
        .map(|(a, b)| (a + 1) * b)
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    run(input, false)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    run(input, true)
}
