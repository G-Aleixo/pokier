use rand::seq::SliceRandom;

use crate::card::{
    Card, Suit
};

/// Wrapper over a vector of cards
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Self {
            cards
        }
    }
}

// constructors
impl Deck {
    /// Generate a standart 52 card deck
    pub fn new_standart() -> Self {
        let suits = [
            Suit::Club,
            Suit::Diamond,
            Suit::Heart,
            Suit::Spade
        ];

        let mut deck = Vec::new();

        for suit in suits {
            for value in 1..=13 {
                deck.push(
                    Card::new(suit, value).unwrap()
                );
            }
        };

        Self {
            cards: deck
        }
    }

    /// Generate a standart shuffled 52 card deck
    pub fn new_shuffled() -> Self {
        let mut deck = Deck::new_standart();
        deck.shuffle();
        deck
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn insert_card(&mut self, card: Card) {
        self.cards.append(&mut vec![card]);
    }

    /// Returns a `Vec<[Card; 2]>` with size `hand_count`
    pub fn deal_cards(&mut self, hand_count: usize) -> Option<Vec<[Card; 2]>> {
        // precheck if there are enough cards to deal
        if hand_count * 2 > self.cards.len() {
            // insufficient amount of cards
            return None;
        };

        let mut cards: Vec<[Card; 2]> = Vec::with_capacity(hand_count);

        for _ in 0..hand_count {
            let hand = [self.draw_card().expect("Check for insufficient card prevention shouldn't fail"),
                                   self.draw_card().expect("Check for insufficient card prevention shouldn't fail")];
            cards.append(&mut vec![hand]);
        };

        Some(cards)
    }
}