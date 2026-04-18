use crate::encode::{Encode, Decode};

#[derive(Debug)]
#[derive(Encode, Decode)]
pub enum CardError {
    InvalidValue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Encode, Decode)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug)]
#[derive(Encode, Decode)]
pub struct Card {
    suit: Suit,
    value: u8,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Result<Self, CardError> {
        // A is 1, K is 13
        if value < 1 || value > 13 {
            return Err(CardError::InvalidValue);
        };

        Ok(Self {
            suit,
            value
        })
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl Eq for Card {}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self.suit {
            Suit::Spade => "S",
            Suit::Heart => "H",
            Suit::Diamond => "D",
            Suit::Club => "C",
        };
        write!(f, "{}{}", self.value, suit)
    }
}
