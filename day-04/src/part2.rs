use crate::common::Card;

pub fn run(input: &str) -> String {
    let cards : Vec<Card> = input
        .lines()
        .map(|line| Card::parse(line).unwrap()).collect();
    let mut cards_with_counts: Vec<(&Card, u32)> = cards.iter().map(|c| (c, 1)).collect();
    for (card_index, card) in cards.iter().enumerate() {
        let matching_numbers = card.count_matching_numbers();
        let count = cards_with_counts[card_index].1;
        for copy_index in 1..=matching_numbers {
            cards_with_counts[card_index + copy_index as usize].1 += count;
        }
    }
    let total = cards_with_counts.iter().map(|(_, count)| count).sum::<u32>();
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"; // Add your test input here
        let expected_output = "30"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}