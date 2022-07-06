//! # Crate for implementation of objects commonly used in card games.
//! Mainly implements the Card-module and its submodules for Suit and Rank.
//! This is in turn used to implement the CardDeck module.

pub mod card;
pub use self::card::{rank::Rank, suit::Suit, Card};

pub mod card_deck;
pub use self::card_deck::CardDeck;
