pub mod rank;
pub mod suit;
use crate::card::{rank::Rank, suit::Suit};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
