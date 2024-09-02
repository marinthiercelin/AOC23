use crate::common::Card;

pub fn run(input: &str) -> String {
    let result : u32 = input
    .lines()
    .map(|line| Card::parse(line).unwrap())
    .map(|card| value_of_card(&card))
    .sum();
    result.to_string()
}

/// Calculate the value of a card
/// The value of a card is equal to 
/// 0 if matching_number_count is 0 or 2^(matching_number_count - 1)
/// Where matching_number_count is the ammount of
/// numbers on the card that are also on the winning numbers
fn value_of_card(card: &Card) -> u32 {
    let matching_numbers = card.count_matching_numbers();
    let value = if matching_numbers == 0 {
        0
    } else {
        2u32.pow(matching_numbers as u32 - 1)
    };
    value
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
        let expected_output = "13"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
