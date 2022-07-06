pub mod rank;
pub mod suit;
use crate::card::{rank::Rank, suit::Suit};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
