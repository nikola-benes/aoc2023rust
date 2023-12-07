mod aoc;
use aoc::*;

const CARDS: &'static str = "23456789TJQKA";
const JOKER: i32 = -1;
const NONE: i32 = -2;

fn card_val(c: char) -> i32 {
    CARDS.find(c).unwrap() as i32
}

fn score(hand: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut last = NONE;
    let mut same = Vec::new();

    let mut jokers = 0;

    for c in hand.clone().sorted() {
        if c == JOKER {
            jokers += 1
        } else if c == last {
            *same.last_mut().unwrap() += 1
        } else {
            same.push(1)
        }
        last = c
    }

    same.sort();
    same.reverse();

    if same.is_empty() {
        same.push(jokers)
    } else {
        same[0] += jokers
    }
    (same, hand)
}

fn solve(hands: Vec<(Vec<i32>, i32)>) -> i32 {
    hands
        .into_iter()
        .map(|(hand, bid)| (score(hand), bid))
        .sorted_it()
        .enumerate()
        .map(|(i, (_, b))| (i as i32 + 1) * b)
        .sum_()
}

fn main() {
    let mut hands = lines().map_v(|line| {
        let (cs, val) = line.split_once(' ').unwrap();
        let hand = cs.chars().map_v(card_val);
        let bid = val.parse_::<i32>();
        (hand, bid)
    });

    println!("{}", solve(hands.clone()));

    let orig_j = card_val('J');

    for (h, _) in hands.iter_mut() {
        for c in h.iter_mut() {
            if *c == orig_j {
                *c = JOKER
            }
        }
    }

    println!("{}", solve(hands));
}
