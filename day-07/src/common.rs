
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Bet {
    pub hand: Hand,
    pub bid: u32,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

pub type Card = u32;

fn parse_card(card_input: char) -> Card {
    match card_input {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card_input.to_digit(10).unwrap(),
    }
}

impl Hand {
    fn parse<F: FnOnce(&[Card]) -> HandType>(hand_input: &str, get_hand_type: F) -> Hand {
        let cards: Vec<Card> = hand_input.chars().map(parse_card).collect();
        let hand_type = get_hand_type(&cards);
        Hand { cards, hand_type }
    }
}

impl Bet {
    pub fn parse<F: FnMut(&[Card]) -> HandType>(bet_input: &str, get_hand_type: F) -> Bet {
        let mut parts = bet_input.split_whitespace();
        let hand = Hand::parse(parts.next().unwrap(), get_hand_type);
        let bid = parts.next().unwrap().parse().unwrap();
        Bet { hand, bid }
    }
}