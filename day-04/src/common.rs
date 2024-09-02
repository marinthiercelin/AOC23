pub struct Card {
    pub card_number: u32,
    pub winning_numbers: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    /// Parse a Card from a string input
    /// The string must be in the format "Card {card_number}: {winning_numbers} | {numbers}"
    pub fn parse(line: &str) -> Option<Self> {
        let mut parts = line.split(":");
        let card_number = parts.next()?.split_whitespace().last()?.parse().ok()?;
        let mut numbers = parts.next()?.split("|");
        let winning_numbers = numbers.next()?.split_whitespace().map(|n| n.parse::<u32>().ok()).collect::<Option<Vec<_>>>()?;
        let numbers = numbers.next()?.split_whitespace().map(|n| n.parse::<u32>().ok()).collect::<Option<Vec<_>>>()?;
        Some(Self { card_number, winning_numbers, numbers })
    }

    /// Get the winning numbers of the card
    pub fn count_matching_numbers(&self) -> u32 {
        self.numbers.iter().filter(|&n| self.winning_numbers.contains(n)).count() as u32
    }
}