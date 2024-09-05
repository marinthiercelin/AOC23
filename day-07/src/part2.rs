use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use crate::common::{Bet, Card, Hand, HandType};

const JOKER: u32 = 11;

pub fn run(input: &str) -> String {
    let mut bets: Vec<Bet> = input.lines().map(|line|Bet::parse(line, get_hand_type)).collect();
    bets.sort_by(|a, b| compare_hands(&a.hand, &b.hand));
    let result = bets
        .iter()
        .enumerate()
        .map(|(bet_index, bet)| (bet_index + 1) as u32 * bet.bid)
        .sum::<u32>();
    result.to_string()
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut card_count_by_value: HashMap<Card, u32> = cards
        .into_iter()
        .fold(HashMap::new(), |mut acc, &card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
    let joker_count = card_count_by_value.remove(&JOKER).unwrap_or(0);
    let mut card_counts = card_count_by_value.into_values().collect::<Vec<u32>>();
    card_counts.sort_by(|a, b| b.cmp(a));
    match (card_counts.get(0).unwrap_or(&0) + joker_count, card_counts.get(1).unwrap_or(&0)) {
        (5, 0) => HandType::FiveOfAKind,
        (4, 1) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, 1) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, 1) => HandType::Pair,
        (1, _) => HandType::HighCard,
        _ => panic!("Invalid hand"),
    }
}

fn compare_hands(first: &Hand, other: &Hand) -> Ordering {
    match first.hand_type.cmp(&other.hand_type) {
        Ordering::Equal => {
            for (&a, &b) in first.cards.iter().zip(other.cards.iter()) {
                if a == JOKER && b != JOKER {
                    return Ordering::Less
                } else if a != JOKER && b == JOKER {
                    return Ordering::Greater
                } else {
                    match a.cmp(&b) {
                        Ordering::Equal => continue,
                        x => return x,
                    }
                }
            }
            Ordering::Equal
        }
        x => x,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"; // Add your test input here
        let expected_output = "5905"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
