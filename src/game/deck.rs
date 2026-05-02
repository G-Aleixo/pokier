use std::{char, slice::Iter};

use rand::{seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    cards: Vec<Card>,
    dealt: Vec<Card>
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl Deck {
    pub fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    pub fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }

    pub fn new() -> Deck {
        Deck::from_cards(Card::all_cards())
    }

    pub fn from_cards(cards: &[Card]) -> Deck {
        Deck {
            cards: cards.to_vec(),
            dealt: vec![]
        }
    }

    pub fn undealt_count(&self) -> usize {
        self.cards.len()
    }

    pub fn dealt_count(&self) -> usize {
        self.dealt.len()
    }

    pub fn count(&self) -> usize {
        self.undealt_count() + self.dealt_count()
    }

    pub fn dealt(&self) -> &[Card] {
        self.dealt.as_slice()
    }

    pub fn deal_one(&mut self) -> Result<Card, &'static str> {
        if let Some(card) = self.cards.pop() {
            self.dealt.push(card);
            Ok(card)
        } else {
            Err("No cards left")
        }
    }

    pub fn deal(&mut self, numcards: usize) -> Vec<Card> {
        let mut result = Vec::with_capacity(numcards);
        for _ in 0..numcards {
            if let Ok(card) = self.deal_one() {
                result.push(card);
            } else {
                break; // empty deck
            }
        }
        result
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn shuffled(mut self) -> Deck {
        self.shuffle();
        self
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank,
            suit
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Card, &'static str> {
        if s.len() != 2 {
            return Err("Invalid str size")
        }

        let s = s.to_string();
        let mut chars = s.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();

        if let Ok(rank) = Rank::from_char(c1)
            && let Ok(suit) = Suit::from_char(c2) {
                return Ok(Card::new(rank, suit));
        }

        if let Ok(suit) = Suit::from_char(c1) 
            && let Ok(rank) = Rank::from_char(c2) {
                return Ok(Card::new(rank, suit));
        }


        Err("Invalid str")
    }

    pub fn to_str(&self) -> String {
        format!("{}{}", self.rank.to_char(), self.suit.to_char())
    }

    pub fn all_cards() -> &'static [Card] {
        static CARDS: [Card; 52] = [
            Card { suit: Suit::Spades, rank: Rank::Two },
            Card { suit: Suit::Spades, rank: Rank::Three },
            Card { suit: Suit::Spades, rank: Rank::Four },
            Card { suit: Suit::Spades, rank: Rank::Five },
            Card { suit: Suit::Spades, rank: Rank::Six },
            Card { suit: Suit::Spades, rank: Rank::Seven },
            Card { suit: Suit::Spades, rank: Rank::Eight },
            Card { suit: Suit::Spades, rank: Rank::Nine },
            Card { suit: Suit::Spades, rank: Rank::Ten },
            Card { suit: Suit::Spades, rank: Rank::Jack },
            Card { suit: Suit::Spades, rank: Rank::Queen },
            Card { suit: Suit::Spades, rank: Rank::King },
            Card { suit: Suit::Spades, rank: Rank::Ace },
            Card { suit: Suit::Hearts, rank: Rank::Two },
            Card { suit: Suit::Hearts, rank: Rank::Three },
            Card { suit: Suit::Hearts, rank: Rank::Four },
            Card { suit: Suit::Hearts, rank: Rank::Five },
            Card { suit: Suit::Hearts, rank: Rank::Six },
            Card { suit: Suit::Hearts, rank: Rank::Seven },
            Card { suit: Suit::Hearts, rank: Rank::Eight },
            Card { suit: Suit::Hearts, rank: Rank::Nine },
            Card { suit: Suit::Hearts, rank: Rank::Ten },
            Card { suit: Suit::Hearts, rank: Rank::Jack },
            Card { suit: Suit::Hearts, rank: Rank::Queen },
            Card { suit: Suit::Hearts, rank: Rank::King },
            Card { suit: Suit::Hearts, rank: Rank::Ace },
            Card { suit: Suit::Diamonds, rank: Rank::Two },
            Card { suit: Suit::Diamonds, rank: Rank::Three },
            Card { suit: Suit::Diamonds, rank: Rank::Four },
            Card { suit: Suit::Diamonds, rank: Rank::Five },
            Card { suit: Suit::Diamonds, rank: Rank::Six },
            Card { suit: Suit::Diamonds, rank: Rank::Seven },
            Card { suit: Suit::Diamonds, rank: Rank::Eight },
            Card { suit: Suit::Diamonds, rank: Rank::Nine },
            Card { suit: Suit::Diamonds, rank: Rank::Ten },
            Card { suit: Suit::Diamonds, rank: Rank::Jack },
            Card { suit: Suit::Diamonds, rank: Rank::Queen },
            Card { suit: Suit::Diamonds, rank: Rank::King },
            Card { suit: Suit::Diamonds, rank: Rank::Ace },
            Card { suit: Suit::Clubs, rank: Rank::Two },
            Card { suit: Suit::Clubs, rank: Rank::Three },
            Card { suit: Suit::Clubs, rank: Rank::Four },
            Card { suit: Suit::Clubs, rank: Rank::Five },
            Card { suit: Suit::Clubs, rank: Rank::Six },
            Card { suit: Suit::Clubs, rank: Rank::Seven },
            Card { suit: Suit::Clubs, rank: Rank::Eight },
            Card { suit: Suit::Clubs, rank: Rank::Nine },
            Card { suit: Suit::Clubs, rank: Rank::Ten },
            Card { suit: Suit::Clubs, rank: Rank::Jack },
            Card { suit: Suit::Clubs, rank: Rank::Queen },
            Card { suit: Suit::Clubs, rank: Rank::King },
            Card { suit: Suit::Clubs, rank: Rank::Ace }
        ];
        &CARDS
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn ordinal(&self) -> usize {
        match self {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }

    pub fn from_char(ch: char) -> Result<Rank, &'static str> {
        match ch {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' => Ok(Rank::Ten),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            'A' | '1' => Ok(Rank::Ace),
            _ => Err("Invalid char")
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        }
    }

    pub fn ranks() -> &'static [Rank] {
        static RANKS: [Rank; 13] = [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
        &RANKS[..]
    }

    pub fn iterator() -> Iter<'static, Suit>{
        Suit::suits().iter()
    }
}


#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    pub fn ordinal(&self) -> usize {
        match self {
            Suit::Spades => 0,
            Suit::Hearts => 1,
            Suit::Diamonds => 2,
            Suit::Clubs => 3,
        }
    }

    pub fn from_char(ch: char) -> Result<Suit, &'static str> {
        match ch {
            'S' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            _ => Err("Invalid char")
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Suit::Spades => 'S',
            Suit::Hearts => 'H',
            Suit::Diamonds => 'D',
            Suit::Clubs => 'C',
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        }
    }

    pub fn suits() -> &'static [Suit] {
        static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        &SUITS[..]
    }

    pub fn iterator() -> Iter<'static, Suit>{
        Suit::suits().iter()
    }
}